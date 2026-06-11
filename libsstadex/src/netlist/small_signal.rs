use crate::catalog::PrimitiveCatalog;
use crate::circuit::{Circuit, CircuitValidationError, validate_circuit};
use crate::netlist::{small_signal_element_name, small_signal_param_name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmallSignalRenderError {
    InvalidCircuit(Vec<CircuitValidationError>),
    MissingPrimitive {
        primitive: String,
    },
    MissingSmallSignalModel {
        primitive: String,
    },
    MissingBranchPin {
        instance: String,
        branch: String,
        pin: String,
    },
}

pub fn render_small_signal_netlist(
    circuit: &Circuit,
    catalog: &PrimitiveCatalog,
) -> Result<String, SmallSignalRenderError> {
    let validation_errors = validate_circuit(circuit, catalog);
    if !validation_errors.is_empty() {
        return Err(SmallSignalRenderError::InvalidCircuit(validation_errors));
    }

    let mut lines = Vec::new();
    lines.push(format!("* Small-signal circuit: {}", circuit.name));

    for instance in &circuit.instances {
        let primitive = catalog.get(&instance.primitive).ok_or_else(|| {
            SmallSignalRenderError::MissingPrimitive {
                primitive: instance.primitive.clone(),
            }
        })?;
        let small_signal = primitive.small_signal.as_ref().ok_or_else(|| {
            SmallSignalRenderError::MissingSmallSignalModel {
                primitive: primitive.name.clone(),
            }
        })?;

        lines.push(String::new());
        lines.push(format!("* instance {} ({})", instance.id, primitive.name));

        for branch in &small_signal.branches {
            let vd = connected_net(circuit, &instance.id, &branch.vd).ok_or_else(|| {
                SmallSignalRenderError::MissingBranchPin {
                    instance: instance.id.clone(),
                    branch: branch.name.clone(),
                    pin: branch.vd.clone(),
                }
            })?;
            let vg = connected_net(circuit, &instance.id, &branch.vg).ok_or_else(|| {
                SmallSignalRenderError::MissingBranchPin {
                    instance: instance.id.clone(),
                    branch: branch.name.clone(),
                    pin: branch.vg.clone(),
                }
            })?;
            let vs = connected_net(circuit, &instance.id, &branch.vs).ok_or_else(|| {
                SmallSignalRenderError::MissingBranchPin {
                    instance: instance.id.clone(),
                    branch: branch.name.clone(),
                    pin: branch.vs.clone(),
                }
            })?;

            let ro_element = small_signal_element_name("R", "ro", &instance.id, &branch.name);
            let ro_param = small_signal_param_name("ro", &instance.id, &branch.name);
            let gm_element = small_signal_element_name("G", "gm", &instance.id, &branch.name);
            let gm_param = small_signal_param_name("gm", &instance.id, &branch.name);

            lines.push(format!("{ro_element} {vd} {vs} {ro_param}"));
            lines.push(format!("{gm_element} {vd} {vs} {vg} {vs} {gm_param}"));
        }
    }

    Ok(format!("{}\n", lines.join("\n")))
}

fn connected_net<'a>(circuit: &'a Circuit, instance: &str, pin: &str) -> Option<&'a str> {
    circuit
        .connections
        .iter()
        .find(|connection| connection.from.instance == instance && connection.from.pin == pin)
        .map(|connection| connection.net.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::PrimitiveCatalog;
    use crate::circuit::{Circuit, Connection, Instance, PinRef};
    use crate::primitive::manifest::{
        Pin, PinRole, PrimitiveFiles, PrimitiveManifest, PrimitiveShape, PrimitiveUi,
    };
    use crate::primitive::small_signal::{SmallSignalBranch, SmallSignalModel};

    #[test]
    fn renders_small_signal_elements_for_branches() {
        let catalog = catalog_with_simplediffpair();
        let mut circuit = Circuit::new("ota");

        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINP"), "VINP"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINN"), "VINN"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTP"), "VOUT"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTN"), "N1"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VTAIL"), "IBIAS"));

        let netlist = render_small_signal_netlist(&circuit, &catalog).unwrap();

        assert!(netlist.contains("* Small-signal circuit: ota"));
        assert!(netlist.contains("R_ro__xdp__m1 VOUT IBIAS ro__xdp__m1"));
        assert!(netlist.contains("G_gm__xdp__m1 VOUT IBIAS VINP IBIAS gm__xdp__m1"));
        assert!(netlist.contains("R_ro__xdp__m2 N1 IBIAS ro__xdp__m2"));
        assert!(netlist.contains("G_gm__xdp__m2 N1 IBIAS VINN IBIAS gm__xdp__m2"));
    }

    #[test]
    fn reports_missing_small_signal_model() {
        let mut catalog = PrimitiveCatalog::new();
        catalog.register(PrimitiveManifest {
            name: "simplediffpair".to_string(),
            version: "1.0".to_string(),
            description: None,
            subckt_name: "simplediffpair".to_string(),
            pins: pins(),
            files: PrimitiveFiles {
                netlist: "netlist/netlist.spice".to_string(),
            },
            ui: PrimitiveUi {
                shape: PrimitiveShape::Box,
            },
            small_signal: None,
        });

        let mut circuit = Circuit::new("ota");
        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINP"), "VINP"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINN"), "VINN"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTP"), "VOUT"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTN"), "N1"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VTAIL"), "IBIAS"));

        let error = render_small_signal_netlist(&circuit, &catalog).unwrap_err();

        assert_eq!(
            error,
            SmallSignalRenderError::MissingSmallSignalModel {
                primitive: "simplediffpair".to_string()
            }
        );
    }

    fn catalog_with_simplediffpair() -> PrimitiveCatalog {
        let mut catalog = PrimitiveCatalog::new();

        catalog.register(PrimitiveManifest {
            name: "simplediffpair".to_string(),
            version: "1.0".to_string(),
            description: None,
            subckt_name: "simplediffpair".to_string(),
            pins: pins(),
            files: PrimitiveFiles {
                netlist: "netlist/netlist.spice".to_string(),
            },
            ui: PrimitiveUi {
                shape: PrimitiveShape::Box,
            },
            small_signal: Some(SmallSignalModel {
                branches: vec![
                    SmallSignalBranch {
                        name: "m1".to_string(),
                        vd: "VOUTP".to_string(),
                        vg: "VINP".to_string(),
                        vs: "VTAIL".to_string(),
                    },
                    SmallSignalBranch {
                        name: "m2".to_string(),
                        vd: "VOUTN".to_string(),
                        vg: "VINN".to_string(),
                        vs: "VTAIL".to_string(),
                    },
                ],
            }),
        });

        catalog
    }

    fn pins() -> Vec<Pin> {
        vec![
            Pin {
                name: "VINP".to_string(),
                role: PinRole::Input,
            },
            Pin {
                name: "VINN".to_string(),
                role: PinRole::Input,
            },
            Pin {
                name: "VOUTP".to_string(),
                role: PinRole::Output,
            },
            Pin {
                name: "VOUTN".to_string(),
                role: PinRole::Output,
            },
            Pin {
                name: "VTAIL".to_string(),
                role: PinRole::Bias,
            },
        ]
    }
}
