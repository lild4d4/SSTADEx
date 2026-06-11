use std::fs;
use std::path::Path;

use super::Circuit;

#[derive(Debug)]
pub enum CircuitIoError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for CircuitIoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for CircuitIoError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

pub fn load_circuit(path: &Path) -> Result<Circuit, CircuitIoError> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn save_circuit(path: &Path, circuit: &Circuit) -> Result<(), CircuitIoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(circuit)?;
    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::{Connection, Instance, PinRef};

    #[test]
    fn saves_and_loads_circuit_json() {
        let dir = make_temp_dir("roundtrip");
        let path = dir.join("circuit.json");

        let mut circuit = Circuit::new("ota");
        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINP"), "VINP"));

        save_circuit(&path, &circuit).unwrap();
        let loaded = load_circuit(&path).unwrap();

        assert_eq!(loaded, circuit);

        let _ = fs::remove_dir_all(dir);
    }

    fn make_temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "libsstadex_circuit_io_{name}_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }
}
