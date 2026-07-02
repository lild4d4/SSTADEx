use std::collections::HashSet;

use crate::catalog::PrimitiveCatalog;

use super::Circuit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitValidationError {
    DuplicateInstance {
        id: String,
    },
    UnknownPrimitive {
        instance: String,
        primitive: String,
    },
    UnknownMacro {
        instance: String,
        macro_name: String,
    },
    UnknownInstance {
        instance: String,
    },
    UnknownPin {
        instance: String,
        primitive: String,
        pin: String,
    },
}

pub fn validate_circuit(
    circuit: &Circuit,
    catalog: &PrimitiveCatalog,
) -> Vec<CircuitValidationError> {
    let mut errors = Vec::new();
    let mut instance_ids = HashSet::new();

    for instance in &circuit.instances {
        if !instance_ids.insert(instance.id.clone()) {
            errors.push(CircuitValidationError::DuplicateInstance {
                id: instance.id.clone(),
            });
        }

        let Some(primitive_name) = instance.primitive_name() else {
            if let Some(macro_name) = instance.macro_name() {
                errors.push(CircuitValidationError::UnknownMacro {
                    instance: instance.id.clone(),
                    macro_name: macro_name.to_string(),
                });
            }
            continue;
        };

        if !catalog.contains(primitive_name) {
            errors.push(CircuitValidationError::UnknownPrimitive {
                instance: instance.id.clone(),
                primitive: primitive_name.to_string(),
            });
        }
    }

    for connection in &circuit.connections {
        let Some(instance) = circuit
            .instances
            .iter()
            .find(|instance| instance.id == connection.from.instance)
        else {
            errors.push(CircuitValidationError::UnknownInstance {
                instance: connection.from.instance.clone(),
            });
            continue;
        };

        let Some(primitive_name) = instance.primitive_name() else {
            continue;
        };
        let Some(primitive) = catalog.get(primitive_name) else {
            continue;
        };

        let pin_exists = primitive
            .pins
            .iter()
            .any(|pin| pin.name == connection.from.pin);

        if !pin_exists {
            errors.push(CircuitValidationError::UnknownPin {
                instance: instance.id.clone(),
                primitive: primitive_name.to_string(),
                pin: connection.from.pin.clone(),
            });
        }
    }

    errors
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
    fn validates_valid_circuit() {
        let catalog = catalog_with_simplediffpair();
        let mut circuit = Circuit::new("ota");

        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINP"), "VINP"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTP"), "VOUT"));

        assert!(validate_circuit(&circuit, &catalog).is_empty());
    }

    #[test]
    fn reports_circuit_validation_errors() {
        let catalog = catalog_with_simplediffpair();
        let mut circuit = Circuit::new("ota");

        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.add_instance(Instance::new("xdp", "unknownprimitive"));
        circuit.connect(Connection::new(PinRef::new("xmissing", "VINP"), "VINP"));
        circuit.connect(Connection::new(PinRef::new("xdp", "NOPE"), "VOUT"));

        let errors = validate_circuit(&circuit, &catalog);

        assert!(errors.contains(&CircuitValidationError::DuplicateInstance {
            id: "xdp".to_string(),
        }));
        assert!(errors.contains(&CircuitValidationError::UnknownPrimitive {
            instance: "xdp".to_string(),
            primitive: "unknownprimitive".to_string(),
        }));
        assert!(errors.contains(&CircuitValidationError::UnknownInstance {
            instance: "xmissing".to_string(),
        }));
        assert!(errors.contains(&CircuitValidationError::UnknownPin {
            instance: "xdp".to_string(),
            primitive: "simplediffpair".to_string(),
            pin: "NOPE".to_string(),
        }));
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
                    name: "VOUTP".to_string(),
                    role: PinRole::Output,
                },
            ],
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

        catalog
    }
}
