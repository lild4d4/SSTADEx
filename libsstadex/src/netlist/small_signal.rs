use crate::catalog::PrimitiveCatalog;
use crate::circuit::{Circuit, CircuitValidationError, validate_circuit};
use crate::exploration::TestbenchSpec;
use crate::netlist::{small_signal_element_name, small_signal_param_name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmallSignalRenderError {
    InvalidCircuit(Vec<CircuitValidationError>),
    MissingPrimitive {
        primitive: String,
    },
    UnsupportedMacroInstance {
        instance: String,
        macro_name: String,
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
        let primitive_name =
            instance
                .primitive_name()
                .ok_or_else(|| match instance.macro_name() {
                    Some(macro_name) => SmallSignalRenderError::UnsupportedMacroInstance {
                        instance: instance.id.clone(),
                        macro_name: macro_name.to_string(),
                    },
                    None => SmallSignalRenderError::MissingPrimitive {
                        primitive: String::new(),
                    },
                })?;
        let primitive = catalog.get(primitive_name).ok_or_else(|| {
            SmallSignalRenderError::MissingPrimitive {
                primitive: primitive_name.to_string(),
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

pub fn render_testbench_small_signal_netlist(
    circuit: &Circuit,
    catalog: &PrimitiveCatalog,
    testbench: &TestbenchSpec,
) -> Result<String, SmallSignalRenderError> {
    let mut netlist = render_small_signal_netlist(circuit, catalog)?;
    let testbench_body = testbench.body_text();

    if !testbench_body.trim().is_empty() {
        if !netlist.ends_with('\n') {
            netlist.push('\n');
        }
        netlist.push_str("\n* testbench ");
        netlist.push_str(&testbench.name);
        netlist.push('\n');
        netlist.push_str(testbench_body.trim());
        netlist.push('\n');
    }

    Ok(netlist)
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
                build: None,
                symbol: None,
            },
            ui: PrimitiveUi {
                shape: PrimitiveShape::Box,
                symbol: None,
            },
            small_signal: None,
            transistor_type: None,
            layout_params: None,
            lut_config: None,
            build: None,
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

    #[test]
    fn renders_small_signal_netlist_with_testbench_body() {
        let catalog = catalog_with_simplediffpair();
        let mut circuit = Circuit::new("ota");

        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINP"), "VINP"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINN"), "VINN"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTP"), "VOUT"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTN"), "N1"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VTAIL"), "IBIAS"));

        let testbench = TestbenchSpec::new("gain")
            .with_element(crate::exploration::TestbenchElement::VoltageSource {
                name: "Vin".to_string(),
                nplus: "VINP".to_string(),
                nminus: "VINN".to_string(),
                value: "ac 1".to_string(),
            })
            .with_extra_body(".ac dec 10 1 1e9");

        let netlist =
            render_testbench_small_signal_netlist(&circuit, &catalog, &testbench).unwrap();

        assert!(netlist.contains("G_gm__xdp__m1 VOUT IBIAS VINP IBIAS gm__xdp__m1"));
        assert!(netlist.contains("* testbench gain"));
        assert!(netlist.contains("Vin VINP VINN ac 1"));
        assert!(netlist.contains(".ac dec 10 1 1e9"));
    }

    #[test]
    fn renders_small_signal_netlist_without_empty_testbench_body() {
        let catalog = catalog_with_simplediffpair();
        let mut circuit = Circuit::new("ota");

        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINP"), "VINP"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINN"), "VINN"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTP"), "VOUT"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTN"), "N1"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VTAIL"), "IBIAS"));

        let netlist =
            render_testbench_small_signal_netlist(&circuit, &catalog, &TestbenchSpec::new("gain"))
                .unwrap();

        assert!(netlist.contains("* Small-signal circuit: ota"));
        assert!(!netlist.contains("* testbench gain"));
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
                build: None,
                symbol: None,
            },
            ui: PrimitiveUi {
                shape: PrimitiveShape::Box,
                symbol: None,
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
            transistor_type: None,
            layout_params: None,
            lut_config: None,
            build: None,
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
