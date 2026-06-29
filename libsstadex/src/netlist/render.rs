use crate::catalog::PrimitiveCatalog;
use crate::circuit::{Circuit, CircuitValidationError, validate_circuit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetlistRenderError {
    InvalidCircuit(Vec<CircuitValidationError>),
    UnconnectedPin {
        instance: String,
        pin: String,
    },
    MissingPrimitive {
        primitive: String,
    },
    UnsupportedMacroInstance {
        instance: String,
        macro_name: String,
    },
}

pub fn render_circuit_netlist(
    circuit: &Circuit,
    catalog: &PrimitiveCatalog,
) -> Result<String, NetlistRenderError> {
    let validation_errors = validate_circuit(circuit, catalog);
    if !validation_errors.is_empty() {
        return Err(NetlistRenderError::InvalidCircuit(validation_errors));
    }

    let mut lines = Vec::new();
    lines.push(format!("* Circuit: {}", circuit.name));

    for instance in &circuit.instances {
        let primitive_name =
            instance
                .primitive_name()
                .ok_or_else(|| match instance.macro_name() {
                    Some(macro_name) => NetlistRenderError::UnsupportedMacroInstance {
                        instance: instance.id.clone(),
                        macro_name: macro_name.to_string(),
                    },
                    None => NetlistRenderError::MissingPrimitive {
                        primitive: String::new(),
                    },
                })?;
        let primitive =
            catalog
                .get(primitive_name)
                .ok_or_else(|| NetlistRenderError::MissingPrimitive {
                    primitive: primitive_name.to_string(),
                })?;

        let mut nets = Vec::new();
        for pin in &primitive.pins {
            let net = circuit
                .connections
                .iter()
                .find(|connection| {
                    connection.from.instance == instance.id && connection.from.pin == pin.name
                })
                .map(|connection| connection.net.clone())
                .ok_or_else(|| NetlistRenderError::UnconnectedPin {
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

    Ok(format!("{}\n", lines.join("\n")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::PrimitiveCatalog;
    use crate::circuit::{Circuit, Connection, Instance, PinRef};
    use crate::primitive::manifest::{
        Pin, PinRole, PrimitiveFiles, PrimitiveManifest, PrimitiveShape, PrimitiveUi,
    };

    #[test]
    fn renders_instance_lines_in_pin_order() {
        let catalog = catalog_with_simplediffpair();
        let mut circuit = Circuit::new("ota");

        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINP"), "VINP"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINN"), "VINN"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTP"), "VOUT"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTN"), "N1"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VTAIL"), "IBIAS"));

        let netlist = render_circuit_netlist(&circuit, &catalog).unwrap();

        assert!(netlist.contains("* Circuit: ota"));
        assert!(netlist.contains("Xxdp VINP VINN VOUT N1 IBIAS simplediffpair"));
    }

    #[test]
    fn reports_unconnected_pin() {
        let catalog = catalog_with_simplediffpair();
        let mut circuit = Circuit::new("ota");

        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINP"), "VINP"));

        let error = render_circuit_netlist(&circuit, &catalog).unwrap_err();

        assert_eq!(
            error,
            NetlistRenderError::UnconnectedPin {
                instance: "xdp".to_string(),
                pin: "VINN".to_string(),
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
            pins: vec![
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
            ],
            files: PrimitiveFiles {
                netlist: "netlist/netlist.spice".to_string(),
            },
            ui: PrimitiveUi {
                shape: PrimitiveShape::Box,
                symbol: None,
            },
            small_signal: None,
        });

        catalog
    }
}
