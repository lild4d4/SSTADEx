use std::collections::HashSet;

use crate::catalog::PrimitiveCatalog;
use crate::macro_model::{validate_macro_model, MacroCatalog, MacroModel, MacroValidationError};
use crate::netlist::{small_signal_element_name, small_signal_param_name};

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
    MissingMacro {
        macro_name: String,
    },
    MissingSmallSignalModel {
        primitive: String,
    },
}

pub fn render_macro_netlist(
    macro_model: &MacroModel,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
) -> Result<String, MacroRenderError> {
    let mut rendered = Vec::new();
    let mut emitted = HashSet::new();

    render_macro_dependencies(
        macro_model,
        primitive_catalog,
        macro_catalog,
        &mut emitted,
        &mut rendered,
    )?;

    rendered.push(render_macro_subckt(
        macro_model,
        primitive_catalog,
        macro_catalog,
    )?);

    Ok(rendered.join("\n"))
}

pub fn render_macro_small_signal_netlist(
    macro_model: &MacroModel,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
) -> Result<String, MacroRenderError> {
    let validation_errors = validate_macro_model(macro_model, primitive_catalog, macro_catalog);
    if !validation_errors.is_empty() {
        return Err(MacroRenderError::InvalidMacro(validation_errors));
    }

    let mut lines = vec![format!("* Small-signal macro: {}", macro_model.name)];
    expand_macro_small_signal(
        macro_model,
        primitive_catalog,
        macro_catalog,
        &mut Vec::new(),
        &PortNetMap::identity(macro_model),
        &mut lines,
    )?;

    Ok(format!("{}\n", lines.join("\n")))
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
        if let Some(primitive_name) = instance.primitive_name() {
            let primitive = primitive_catalog.get(primitive_name).ok_or_else(|| {
                MacroRenderError::MissingPrimitive {
                    primitive: primitive_name.to_string(),
                }
            })?;
            let mut nets = Vec::with_capacity(primitive.pins.len());

            for pin in &primitive.pins {
                nets.push(connected_net(macro_model, &instance.id, &pin.name)?);
            }

            lines.push(format!(
                "X{} {} {}",
                instance.id,
                nets.join(" "),
                primitive.subckt_name
            ));
        } else if let Some(nested_macro_name) = instance.macro_name() {
            let nested_macro = macro_catalog.get(nested_macro_name).ok_or_else(|| {
                MacroRenderError::MissingMacro {
                    macro_name: nested_macro_name.to_string(),
                }
            })?;
            let mut nets = Vec::with_capacity(nested_macro.ports.len());

            for port in &nested_macro.ports {
                nets.push(connected_net(macro_model, &instance.id, &port.name)?);
            }

            lines.push(format!(
                "X{} {} {}",
                instance.id,
                nets.join(" "),
                nested_macro.subckt_name
            ));
        }
    }

    lines.push(format!(".ends {}", macro_model.subckt_name));

    Ok(format!("{}\n", lines.join("\n")))
}

fn render_macro_dependencies(
    macro_model: &MacroModel,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    emitted: &mut HashSet<String>,
    rendered: &mut Vec<String>,
) -> Result<(), MacroRenderError> {
    for instance in &macro_model.circuit.instances {
        let Some(nested_macro_name) = instance.macro_name() else {
            continue;
        };
        if emitted.contains(nested_macro_name) {
            continue;
        }

        let nested_macro =
            macro_catalog
                .get(nested_macro_name)
                .ok_or_else(|| MacroRenderError::MissingMacro {
                    macro_name: nested_macro_name.to_string(),
                })?;

        render_macro_dependencies(
            nested_macro,
            primitive_catalog,
            macro_catalog,
            emitted,
            rendered,
        )?;
        rendered.push(render_macro_subckt(
            nested_macro,
            primitive_catalog,
            macro_catalog,
        )?);
        emitted.insert(nested_macro_name.to_string());
    }

    Ok(())
}

fn expand_macro_small_signal(
    macro_model: &MacroModel,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    path: &mut Vec<String>,
    port_map: &PortNetMap,
    lines: &mut Vec<String>,
) -> Result<(), MacroRenderError> {
    for instance in &macro_model.circuit.instances {
        path.push(instance.id.clone());

        if let Some(primitive_name) = instance.primitive_name() {
            let primitive = primitive_catalog.get(primitive_name).ok_or_else(|| {
                MacroRenderError::MissingPrimitive {
                    primitive: primitive_name.to_string(),
                }
            })?;
            let small_signal = primitive.small_signal.as_ref().ok_or_else(|| {
                MacroRenderError::MissingSmallSignalModel {
                    primitive: primitive_name.to_string(),
                }
            })?;
            let instance_path = path.join("__");

            lines.push(String::new());
            lines.push(format!("* instance {} ({})", instance_path, primitive.name));

            for branch in &small_signal.branches {
                let vd =
                    resolved_instance_pin_net(macro_model, &instance.id, &branch.vd, port_map)?;
                let vg =
                    resolved_instance_pin_net(macro_model, &instance.id, &branch.vg, port_map)?;
                let vs =
                    resolved_instance_pin_net(macro_model, &instance.id, &branch.vs, port_map)?;
                let ro_element = small_signal_element_name("R", "ro", &instance_path, &branch.name);
                let ro_param = small_signal_param_name("ro", &instance_path, &branch.name);
                let gm_element = small_signal_element_name("G", "gm", &instance_path, &branch.name);
                let gm_param = small_signal_param_name("gm", &instance_path, &branch.name);

                lines.push(format!("{ro_element} {vd} {vs} {ro_param}"));
                lines.push(format!("{gm_element} {vd} {vs} {vg} {vs} {gm_param}"));
            }
        } else if let Some(nested_macro_name) = instance.macro_name() {
            let nested_macro = macro_catalog.get(nested_macro_name).ok_or_else(|| {
                MacroRenderError::MissingMacro {
                    macro_name: nested_macro_name.to_string(),
                }
            })?;
            let nested_port_map =
                PortNetMap::from_instance(macro_model, &instance.id, nested_macro, port_map)?;

            lines.push(String::new());
            lines.push(format!("* nested macro instance {}", path.join("__")));
            expand_macro_small_signal(
                nested_macro,
                primitive_catalog,
                macro_catalog,
                path,
                &nested_port_map,
                lines,
            )?;
        }

        path.pop();
    }

    Ok(())
}

struct PortNetMap {
    entries: Vec<(String, String)>,
}

impl PortNetMap {
    fn identity(macro_model: &MacroModel) -> Self {
        Self {
            entries: macro_model
                .ports
                .iter()
                .map(|port| (port.name.clone(), port.name.clone()))
                .collect(),
        }
    }

    fn from_instance(
        parent_macro: &MacroModel,
        instance: &str,
        nested_macro: &MacroModel,
        parent_port_map: &PortNetMap,
    ) -> Result<Self, MacroRenderError> {
        let mut entries = Vec::with_capacity(nested_macro.ports.len());

        for port in &nested_macro.ports {
            let parent_net = connected_net(parent_macro, instance, &port.name)?;
            let resolved = parent_port_map.resolve(parent_net);
            entries.push((port.name.clone(), resolved));
        }

        Ok(Self { entries })
    }

    fn resolve(&self, net: &str) -> String {
        self.entries
            .iter()
            .find(|(port, _)| port == net)
            .map(|(_, mapped)| mapped.clone())
            .unwrap_or_else(|| net.to_string())
    }
}

fn resolved_instance_pin_net(
    macro_model: &MacroModel,
    instance: &str,
    pin: &str,
    port_map: &PortNetMap,
) -> Result<String, MacroRenderError> {
    let local_net = connected_net(macro_model, instance, pin)?;

    Ok(port_map.resolve(local_net))
}

fn connected_net<'a>(
    macro_model: &'a MacroModel,
    instance: &str,
    pin: &str,
) -> Result<&'a str, MacroRenderError> {
    macro_model
        .circuit
        .connections
        .iter()
        .find(|connection| connection.from.instance == instance && connection.from.pin == pin)
        .map(|connection| connection.net.as_str())
        .ok_or_else(|| MacroRenderError::MissingConnectedPin {
            macro_name: macro_model.name.clone(),
            instance: instance.to_string(),
            pin: pin.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::load_primitive_catalog;
    use crate::circuit::{Circuit, Connection, Instance, PinRef};
    use crate::macro_model::{
        load_macro_catalog, load_macro_model, MacroModel, MacroPort, MacroPortRole,
    };
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

    #[test]
    fn renders_nested_macro_dependencies_before_parent() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let primitive_catalog =
            load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
                .expect("primitive catalog should load");
        let child =
            load_macro_model(&workspace_root.join("analoglib/macros/current_source/macro.json"))
                .expect("current_source macro should load");
        let parent = parent_macro_with_child();
        let mut macro_catalog = MacroCatalog::new();
        macro_catalog.register(child);
        macro_catalog.register(parent.clone());

        let netlist = render_macro_netlist(&parent, &primitive_catalog, &macro_catalog).unwrap();
        let child_index = netlist.find(".subckt current_source").unwrap();
        let parent_index = netlist.find(".subckt parent_macro").unwrap();

        assert!(child_index < parent_index);
        assert!(netlist.contains("Xxchild OUT 0 BIAS current_source"));
    }

    #[test]
    fn renders_ota_1stage_with_current_source_dependency() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let primitive_catalog =
            load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
                .expect("primitive catalog should load");
        let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros"))
            .expect("macro catalog should load");
        let ota = macro_catalog
            .get("ota_1stage")
            .expect("ota_1stage macro should load");

        let netlist = render_macro_netlist(ota, &primitive_catalog, &macro_catalog).unwrap();

        assert!(netlist.contains(".subckt current_source VOUT VSS VBIAS"));
        assert!(netlist.contains(".subckt ota_1stage VINP VINN VOUT VDD VSS VBIAS"));
        assert!(netlist.contains("Xxdp VINP VINN VOUT N1 IBIAS simplediffpair"));
        assert!(netlist.contains("Xxcm N1 VOUT VDD simplecurrentmirror"));
        assert!(netlist.contains("Xxcs_macro IBIAS VSS VBIAS current_source"));
    }

    #[test]
    fn renders_ota_1stage_small_signal_with_hierarchical_names() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let primitive_catalog =
            load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
                .expect("primitive catalog should load");
        let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros"))
            .expect("macro catalog should load");
        let ota = macro_catalog
            .get("ota_1stage")
            .expect("ota_1stage macro should load");

        let netlist =
            render_macro_small_signal_netlist(ota, &primitive_catalog, &macro_catalog).unwrap();

        assert!(netlist.contains("* Small-signal macro: ota_1stage"));
        assert!(netlist.contains("R_ro__xdp__m1 VOUT IBIAS ro__xdp__m1"));
        assert!(netlist.contains("G_gm__xdp__m1 VOUT IBIAS VINP IBIAS gm__xdp__m1"));
        assert!(netlist.contains("R_ro__xcm__m1 VOUT VDD ro__xcm__m1"));
        assert!(netlist.contains("R_ro__xcs_macro__xcs__m1 IBIAS VSS ro__xcs_macro__xcs__m1"));
        assert!(
            netlist.contains("G_gm__xcs_macro__xcs__m1 IBIAS VSS VBIAS VSS gm__xcs_macro__xcs__m1")
        );
    }

    fn parent_macro_with_child() -> MacroModel {
        let mut circuit = Circuit::new("parent_macro");
        circuit.add_instance(Instance::macro_instance("xchild", "current_source"));
        circuit.connect(Connection::new(PinRef::new("xchild", "VOUT"), "OUT"));
        circuit.connect(Connection::new(PinRef::new("xchild", "VSS"), "0"));
        circuit.connect(Connection::new(PinRef::new("xchild", "VBIAS"), "BIAS"));

        MacroModel::new(
            "parent_macro",
            vec![
                MacroPort::new("OUT", MacroPortRole::Output),
                MacroPort::new("0", MacroPortRole::Ground),
                MacroPort::new("BIAS", MacroPortRole::Bias),
            ],
            circuit,
        )
    }
}
