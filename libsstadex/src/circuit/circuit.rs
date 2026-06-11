use super::{Connection, Instance};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Circuit {
    pub name: String,
    pub instances: Vec<Instance>,
    pub connections: Vec<Connection>,
}

impl Circuit {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instances: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_instance(&mut self, instance: Instance) {
        self.instances.push(instance);
    }

    pub fn connect(&mut self, connection: Connection) {
        self.connections.push(connection);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::{Connection, Instance, PinRef};

    #[test]
    fn creates_circuit_with_instance_and_connection() {
        let mut circuit = Circuit::new("ota_1stage");

        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VOUTP"), "VOUT"));

        assert_eq!(circuit.name, "ota_1stage");
        assert_eq!(circuit.instances.len(), 1);
        assert_eq!(circuit.connections.len(), 1);
        assert_eq!(circuit.instances[0].id, "xdp");
        assert_eq!(circuit.connections[0].net, "VOUT");
    }
}
