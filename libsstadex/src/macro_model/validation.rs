use std::collections::HashSet;

use crate::catalog::PrimitiveCatalog;
use crate::macro_model::{MacroCatalog, MacroModel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroValidationError {
    DuplicatePort {
        macro_name: String,
        port: String,
    },
    MissingPublicPortNet {
        macro_name: String,
        port: String,
    },
    DuplicateInstance {
        macro_name: String,
        instance: String,
    },
    UnknownPrimitive {
        macro_name: String,
        instance: String,
        primitive: String,
    },
    UnknownMacro {
        macro_name: String,
        instance: String,
        referenced_macro: String,
    },
    UnknownPin {
        macro_name: String,
        instance: String,
        block: String,
        pin: String,
    },
    CyclicDependency {
        path: Vec<String>,
    },
}

pub fn validate_macro_catalog(
    macro_catalog: &MacroCatalog,
    primitive_catalog: &PrimitiveCatalog,
) -> Vec<MacroValidationError> {
    let mut errors = Vec::new();

    for macro_model in macro_catalog.list() {
        errors.extend(validate_macro_model(
            macro_model,
            primitive_catalog,
            macro_catalog,
        ));
    }

    let mut visited = HashSet::new();
    let mut active = Vec::new();
    for macro_model in macro_catalog.list() {
        detect_macro_cycles(
            &macro_model.name,
            macro_catalog,
            &mut visited,
            &mut active,
            &mut errors,
        );
    }

    errors
}

pub fn validate_macro_model(
    macro_model: &MacroModel,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
) -> Vec<MacroValidationError> {
    let mut errors = Vec::new();
    let mut ports = HashSet::new();

    for port in &macro_model.ports {
        if !ports.insert(port.name.clone()) {
            errors.push(MacroValidationError::DuplicatePort {
                macro_name: macro_model.name.clone(),
                port: port.name.clone(),
            });
        }

        if !macro_model
            .circuit
            .connections
            .iter()
            .any(|connection| connection.net == port.name)
        {
            errors.push(MacroValidationError::MissingPublicPortNet {
                macro_name: macro_model.name.clone(),
                port: port.name.clone(),
            });
        }
    }

    let mut instances = HashSet::new();
    for instance in &macro_model.circuit.instances {
        if !instances.insert(instance.id.clone()) {
            errors.push(MacroValidationError::DuplicateInstance {
                macro_name: macro_model.name.clone(),
                instance: instance.id.clone(),
            });
        }

        if let Some(primitive) = instance.primitive_name() {
            if !primitive_catalog.contains(primitive) {
                errors.push(MacroValidationError::UnknownPrimitive {
                    macro_name: macro_model.name.clone(),
                    instance: instance.id.clone(),
                    primitive: primitive.to_string(),
                });
            }
        } else if let Some(referenced_macro) = instance.macro_name() {
            if !macro_catalog.contains(referenced_macro) {
                errors.push(MacroValidationError::UnknownMacro {
                    macro_name: macro_model.name.clone(),
                    instance: instance.id.clone(),
                    referenced_macro: referenced_macro.to_string(),
                });
            }
        }
    }

    for connection in &macro_model.circuit.connections {
        let Some(instance) = macro_model
            .circuit
            .instances
            .iter()
            .find(|instance| instance.id == connection.from.instance)
        else {
            continue;
        };

        if let Some(primitive_name) = instance.primitive_name() {
            let Some(primitive) = primitive_catalog.get(primitive_name) else {
                continue;
            };
            if !primitive
                .pins
                .iter()
                .any(|pin| pin.name == connection.from.pin)
            {
                errors.push(MacroValidationError::UnknownPin {
                    macro_name: macro_model.name.clone(),
                    instance: instance.id.clone(),
                    block: primitive_name.to_string(),
                    pin: connection.from.pin.clone(),
                });
            }
        } else if let Some(macro_name) = instance.macro_name() {
            let Some(referenced_macro) = macro_catalog.get(macro_name) else {
                continue;
            };
            if !referenced_macro
                .ports
                .iter()
                .any(|port| port.name == connection.from.pin)
            {
                errors.push(MacroValidationError::UnknownPin {
                    macro_name: macro_model.name.clone(),
                    instance: instance.id.clone(),
                    block: macro_name.to_string(),
                    pin: connection.from.pin.clone(),
                });
            }
        }
    }

    errors
}

fn detect_macro_cycles(
    macro_name: &str,
    macro_catalog: &MacroCatalog,
    visited: &mut HashSet<String>,
    active: &mut Vec<String>,
    errors: &mut Vec<MacroValidationError>,
) {
    if let Some(cycle_start) = active.iter().position(|name| name == macro_name) {
        let mut path = active[cycle_start..].to_vec();
        path.push(macro_name.to_string());
        let error = MacroValidationError::CyclicDependency { path };
        if !errors.contains(&error) {
            errors.push(error);
        }
        return;
    }

    if !visited.insert(macro_name.to_string()) {
        return;
    }

    let Some(macro_model) = macro_catalog.get(macro_name) else {
        return;
    };

    active.push(macro_name.to_string());
    for instance in &macro_model.circuit.instances {
        if let Some(referenced_macro) = instance.macro_name() {
            detect_macro_cycles(referenced_macro, macro_catalog, visited, active, errors);
        }
    }
    active.pop();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::load_primitive_catalog;
    use crate::circuit::{Circuit, Connection, Instance, PinRef};
    use crate::macro_model::{MacroPort, MacroPortRole, load_macro_catalog};

    #[test]
    fn validates_current_source_macro() {
        let workspace_root = workspace_root();
        let primitive_catalog =
            load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
                .expect("primitive catalog should load");
        let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros"))
            .expect("macro catalog should load");
        let current_source = macro_catalog
            .get("current_source")
            .expect("current_source macro should load");

        let errors = validate_macro_model(current_source, &primitive_catalog, &macro_catalog);

        assert_eq!(errors, Vec::new());
    }

    #[test]
    fn reports_unknown_primitive_pin() {
        let workspace_root = workspace_root();
        let primitive_catalog =
            load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
                .expect("primitive catalog should load");
        let macro_catalog = MacroCatalog::new();
        let mut circuit = Circuit::new("bad_macro");
        circuit.add_instance(Instance::primitive("xcs", "simplecurrentsource"));
        circuit.connect(Connection::new(PinRef::new("xcs", "NOPE"), "VOUT"));
        let macro_model = MacroModel::new(
            "bad_macro",
            vec![MacroPort::new("VOUT", MacroPortRole::Output)],
            circuit,
        );

        let errors = validate_macro_model(&macro_model, &primitive_catalog, &macro_catalog);

        assert!(errors.contains(&MacroValidationError::UnknownPin {
            macro_name: "bad_macro".to_string(),
            instance: "xcs".to_string(),
            block: "simplecurrentsource".to_string(),
            pin: "NOPE".to_string(),
        }));
    }

    #[test]
    fn reports_macro_dependency_cycle() {
        let primitive_catalog = PrimitiveCatalog::new();
        let mut macro_catalog = MacroCatalog::new();

        macro_catalog.register(macro_with_macro_instance("a", "xb", "b"));
        macro_catalog.register(macro_with_macro_instance("b", "xc", "c"));
        macro_catalog.register(macro_with_macro_instance("c", "xa", "a"));

        let errors = validate_macro_catalog(&macro_catalog, &primitive_catalog);

        assert!(
            errors
                .iter()
                .any(|error| matches!(error, MacroValidationError::CyclicDependency { path } if path == &vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ] || path == &vec![
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string(),
                    "b".to_string()
                ] || path == &vec![
                    "c".to_string(),
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string()
                ]))
        );
    }

    fn workspace_root() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace")
            .to_path_buf()
    }

    fn macro_with_macro_instance(name: &str, instance: &str, referenced_macro: &str) -> MacroModel {
        let mut circuit = Circuit::new(name);
        circuit.add_instance(Instance::macro_instance(instance, referenced_macro));
        circuit.connect(Connection::new(PinRef::new(instance, "P"), "P"));
        MacroModel::new(
            name,
            vec![MacroPort::new("P", MacroPortRole::Inout)],
            circuit,
        )
    }
}
