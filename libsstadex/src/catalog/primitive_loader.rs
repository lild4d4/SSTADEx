use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::primitive::manifest::{
    Pin, PinRole, PrimitiveFiles, PrimitiveManifest, PrimitiveShape, PrimitiveUi,
};
use crate::primitive::small_signal::SmallSignalModel;

use super::PrimitiveCatalog;

#[derive(Debug)]
pub enum PrimitiveLoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingPort {
        primitive: String,
        pin: String,
    },
    MissingNetlistFile {
        primitive: String,
    },
}

impl From<std::io::Error> for PrimitiveLoadError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for PrimitiveLoadError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

pub fn load_primitive_manifest(path: &Path) -> Result<PrimitiveManifest, PrimitiveLoadError> {
    let content = fs::read_to_string(path)?;
    let raw: RawPrimitiveManifest = serde_json::from_str(&content)?;
    raw.into_manifest()
}

pub fn load_primitive_catalog(primitives_dir: &Path) -> Result<PrimitiveCatalog, PrimitiveLoadError> {
    let mut catalog = PrimitiveCatalog::new();

    for entry in fs::read_dir(primitives_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let manifest_path = path.join("primitive.json");
        if !manifest_path.exists() {
            continue;
        }

        catalog.register(load_primitive_manifest(&manifest_path)?);
    }

    Ok(catalog)
}

#[derive(Debug, Deserialize)]
struct RawPrimitiveManifest {
    name: String,
    version: Option<String>,
    description: Option<String>,
    subckt_name: Option<String>,
    pin_order: Option<Vec<String>>,
    ports: HashMap<String, RawPort>,
    files: RawPrimitiveFiles,
    small_signal: Option<SmallSignalModel>,
}

impl RawPrimitiveManifest {
    fn into_manifest(self) -> Result<PrimitiveManifest, PrimitiveLoadError> {
        let pin_order = self
            .pin_order
            .unwrap_or_else(|| sorted_port_names(&self.ports));

        let mut pins = Vec::with_capacity(pin_order.len());
        for pin_name in pin_order {
            let port = self
                .ports
                .get(&pin_name)
                .ok_or_else(|| PrimitiveLoadError::MissingPort {
                    primitive: self.name.clone(),
                    pin: pin_name.clone(),
                })?;

            pins.push(Pin {
                name: port.name.clone().unwrap_or(pin_name),
                role: port.role.clone(),
            });
        }

        let netlist = self.files.netlist.ok_or_else(|| {
            PrimitiveLoadError::MissingNetlistFile {
                primitive: self.name.clone(),
            }
        })?;

        Ok(PrimitiveManifest {
            subckt_name: self.subckt_name.unwrap_or_else(|| self.name.clone()),
            name: self.name,
            version: self.version.unwrap_or_else(|| "0.1.0".to_string()),
            description: self.description,
            pins,
            files: PrimitiveFiles { netlist },
            ui: PrimitiveUi {
                shape: PrimitiveShape::Box,
            },
            small_signal: self.small_signal,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawPort {
    name: Option<String>,
    role: PinRole,
}

#[derive(Debug, Deserialize)]
struct RawPrimitiveFiles {
    netlist: Option<String>,
}

fn sorted_port_names(ports: &HashMap<String, RawPort>) -> Vec<String> {
    let mut names = ports.keys().cloned().collect::<Vec<_>>();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn loads_manifest_from_primitive_json() {
        let dir = make_temp_dir("manifest");
        let manifest_path = dir.join("primitive.json");
        write_file(
            &manifest_path,
            r#"{
              "name": "simplediffpair",
              "version": "1.0",
              "description": "Simple differential pair",
              "subckt_name": "simplediffpair",
              "pin_order": ["VINP", "VINN", "VOUTP"],
              "ports": {
                "VINP": { "name": "VINP", "role": "INPUT" },
                "VINN": { "name": "VINN", "role": "INPUT" },
                "VOUTP": { "name": "VOUTP", "role": "OUTPUT" }
              },
              "small_signal": {
                "branches": [
                  {
                    "name": "m1",
                    "vd": "VOUTP",
                    "vg": "VINP",
                    "vs": "VINN"
                  }
                ]
              },
              "files": {
                "netlist": "netlist/netlist.spice"
              }
            }"#,
        );

        let manifest = load_primitive_manifest(&manifest_path).unwrap();

        assert_eq!(manifest.name, "simplediffpair");
        assert_eq!(manifest.pins.len(), 3);
        assert_eq!(manifest.pins[0].name, "VINP");
        assert_eq!(manifest.pins[2].role, PinRole::Output);
        assert_eq!(manifest.files.netlist, "netlist/netlist.spice");
        assert_eq!(
            manifest.small_signal.as_ref().unwrap().branches[0].name,
            "m1"
        );
        assert_eq!(
            manifest.small_signal.as_ref().unwrap().branches[0].vd,
            "VOUTP"
        );

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn loads_catalog_from_primitive_folders() {
        let dir = make_temp_dir("catalog");
        let primitive_dir = dir.join("simplediffpair");
        fs::create_dir_all(&primitive_dir).unwrap();
        write_file(
            &primitive_dir.join("primitive.json"),
            r#"{
              "name": "simplediffpair",
              "version": "1.0",
              "subckt_name": "simplediffpair",
              "pin_order": ["VINP"],
              "ports": {
                "VINP": { "name": "VINP", "role": "INPUT" }
              },
              "files": {
                "netlist": "netlist/netlist.spice"
              }
            }"#,
        );

        let catalog = load_primitive_catalog(&dir).unwrap();

        assert!(catalog.contains("simplediffpair"));
        assert_eq!(catalog.list().len(), 1);

        let _ = fs::remove_dir_all(dir);
    }

    fn make_temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "libsstadex_primitive_loader_{name}_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_file(path: &Path, content: &str) {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}
