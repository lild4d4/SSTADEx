use crate::catalog::PrimitiveCatalog;
use crate::macro_model::{validate_macro_model, MacroCatalog, MacroModel, MacroValidationError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroRenderError {
    InvalidMacro(Vec<MacroValidationError>),
    MissingPrimitive {
        primitive: String,
    },
    MissingConnectedPin {
        macro_name: String,
        instance: String,
        pin: String,
    },
    UnsupportedNestedMacro {
        macro_name: String,
        instance: String,
        nested_macro: String,
    },
}

pub fn render_macro_subckt(
    macro_model: &MacroModel,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
) -> Result<String, MacroRenderError> {
    let validation_errors = validate_macro_model(macro_model, primitive_catalog, macro_catalog);
    if !validation_errors.is_empty() {
        return Err(MacroRenderError::InvalidMacro(validation_errors));
    }

    let mut lines = Vec::new();
    let ports = macro_model
        .ports
        .iter()
        .map(|port| port.name.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    lines.push(format!(".subckt {} {}", macro_model.subckt_name, ports));

    for instance in &macro_model.circuit.instances {
        if let Some(nested_macro) = instance.macro_name() {
            return Err(MacroRenderError::UnsupportedNestedMacro {
                macro_name: macro_model.name.clone(),
                instance: instance.id.clone(),
                nested_macro: nested_macro.to_string(),
            });
        }

        let Some(primitive_name) = instance.primitive_name() else {
            continue;
        };
        let primitive = primitive_catalog.get(primitive_name).ok_or_else(|| {
            MacroRenderError::MissingPrimitive {
                primitive: primitive_name.to_string(),
            }
        })?;
        let mut nets = Vec::with_capacity(primitive.pins.len());

        for pin in &primitive.pins {
            let net = macro_model
                .circuit
                .connections
                .iter()
                .find(|connection| {
                    connection.from.instance == instance.id && connection.from.pin == pin.name
                })
                .map(|connection| connection.net.as_str())
                .ok_or_else(|| MacroRenderError::MissingConnectedPin {
                    macro_name: macro_model.name.clone(),
                    instance: instance.id.clone(),
                    pin: pin.name.clone(),
                })?;

            nets.push(net);
        }

        lines.push(format!(
            "X{} {} {}",
            instance.id,
            nets.join(" "),
            primitive.subckt_name
        ));
    }

    lines.push(format!(".ends {}", macro_model.subckt_name));

    Ok(format!("{}\n", lines.join("\n")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::load_primitive_catalog;
    use crate::macro_model::{load_macro_catalog, load_macro_model};
    use std::path::Path;

    #[test]
    fn renders_current_source_subckt() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let primitive_catalog =
            load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
                .expect("primitive catalog should load");
        let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros"))
            .expect("macro catalog should load");
        let macro_model =
            load_macro_model(&workspace_root.join("analoglib/macros/current_source/macro.json"))
                .expect("current_source macro should load");

        let netlist =
            render_macro_subckt(&macro_model, &primitive_catalog, &macro_catalog).unwrap();

        assert!(netlist.contains(".subckt current_source VOUT VSS VBIAS"));
        assert!(netlist.contains("Xxcs VBIAS VOUT VSS simplecurrentsource"));
        assert!(netlist.contains(".ends current_source"));
    }
}
