use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::primitive::build::PrimitiveBuildSpec;
use crate::primitive::manifest::{
    Pin, PinRole, PrimitiveFiles, PrimitiveManifest, PrimitiveShape, PrimitiveSymbol, PrimitiveUi,
    SymbolPin,
};
use crate::primitive::small_signal::SmallSignalModel;

use super::PrimitiveCatalog;

#[derive(Debug)]
pub enum PrimitiveLoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
    ExternalBuildJson {
        path: std::path::PathBuf,
        source: serde_json::Error,
    },
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
    raw.into_manifest(path.parent().unwrap_or_else(|| Path::new(".")))
}

pub fn load_primitive_catalog(
    primitives_dir: &Path,
) -> Result<PrimitiveCatalog, PrimitiveLoadError> {
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
    transistor_type: Option<String>,
    pin_order: Option<Vec<String>>,
    ports: HashMap<String, RawPort>,
    layout_params: Option<serde_json::Value>,
    lut_config: Option<serde_json::Value>,
    files: RawPrimitiveFiles,
    ui: Option<RawPrimitiveUi>,
    small_signal: Option<SmallSignalModel>,
    build: Option<PrimitiveBuildSpec>,
}

impl RawPrimitiveManifest {
    fn into_manifest(self, base_dir: &Path) -> Result<PrimitiveManifest, PrimitiveLoadError> {
        let pin_order = self
            .pin_order
            .unwrap_or_else(|| sorted_port_names(&self.ports));

        let mut pins = Vec::with_capacity(pin_order.len());
        for pin_name in pin_order {
            let port =
                self.ports
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

        let netlist = self
            .files
            .netlist
            .ok_or_else(|| PrimitiveLoadError::MissingNetlistFile {
                primitive: self.name.clone(),
            })?;

        let files = PrimitiveFiles {
            netlist,
            build: self.files.build.clone(),
            symbol: self.files.symbol.clone(),
        };
        let build = match self.build {
            Some(build) => Some(build),
            None => load_external_build_spec(base_dir, files.build.as_deref())?,
        };

        Ok(PrimitiveManifest {
            subckt_name: self.subckt_name.unwrap_or_else(|| self.name.clone()),
            name: self.name,
            version: self.version.unwrap_or_else(|| "0.1.0".to_string()),
            description: self.description,
            pins,
            files,
            ui: PrimitiveUi {
                shape: PrimitiveShape::Box,
                symbol: self.ui.and_then(RawPrimitiveUi::into_symbol),
            },
            small_signal: self.small_signal,
            transistor_type: self.transistor_type,
            layout_params: self.layout_params,
            lut_config: self.lut_config,
            build,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawPrimitiveUi {
    symbol: Option<RawPrimitiveSymbol>,
}

impl RawPrimitiveUi {
    fn into_symbol(self) -> Option<PrimitiveSymbol> {
        self.symbol.map(|symbol| PrimitiveSymbol {
            pins: symbol
                .pins
                .into_iter()
                .map(|pin| SymbolPin {
                    name: pin.name,
                    side: pin.side,
                    offset: pin.offset,
                })
                .collect(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawPrimitiveSymbol {
    pins: Vec<RawSymbolPin>,
}

#[derive(Debug, Deserialize)]
struct RawSymbolPin {
    name: String,
    side: crate::primitive::manifest::SymbolPinSide,
    offset: f32,
}

#[derive(Debug, Deserialize)]
struct RawPort {
    name: Option<String>,
    role: PinRole,
}

#[derive(Debug, Deserialize)]
struct RawPrimitiveFiles {
    netlist: Option<String>,
    build: Option<String>,
    symbol: Option<String>,
}

fn load_external_build_spec(
    base_dir: &Path,
    build_file: Option<&str>,
) -> Result<Option<PrimitiveBuildSpec>, PrimitiveLoadError> {
    let Some(build_file) = build_file else {
        return Ok(None);
    };

    if !build_file.ends_with(".json") {
        return Ok(None);
    }

    let path = base_dir.join(build_file);
    let content = fs::read_to_string(&path)?;
    serde_json::from_str(&content)
        .map(Some)
        .map_err(|source| PrimitiveLoadError::ExternalBuildJson { path, source })
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

    #[test]
    fn loads_inline_declarative_build_spec() {
        let dir = make_temp_dir("inline_build");
        let manifest_path = dir.join("primitive.json");
        write_file(
            &manifest_path,
            r#"{
              "name": "simple",
              "pin_order": ["VIN"],
              "ports": {
                "VIN": { "name": "VIN", "role": "INPUT" }
              },
              "files": {
                "netlist": "netlist/netlist.spice"
              },
              "build": {
                "inputs": [
                  { "name": "current", "kind": "scalar", "required": true }
                ],
                "columns": [
                  { "name": "gm", "expr": "current * 10" }
                ]
              }
            }"#,
        );

        let manifest = load_primitive_manifest(&manifest_path).unwrap();

        let build = manifest.build.unwrap();
        assert_eq!(build.inputs[0].name, "current");
        assert_eq!(build.columns[0].expr, "current * 10");

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn loads_external_declarative_build_json() {
        let dir = make_temp_dir("external_build");
        let manifest_path = dir.join("primitive.json");
        write_file(
            &manifest_path,
            r#"{
              "name": "simple",
              "pin_order": ["VIN"],
              "ports": {
                "VIN": { "name": "VIN", "role": "INPUT" }
              },
              "files": {
                "build": "build.json",
                "netlist": "netlist/netlist.spice"
              }
            }"#,
        );
        write_file(
            &dir.join("build.json"),
            r#"{
              "inputs": [
                { "name": "current", "kind": "scalar", "required": true }
              ],
              "columns": [
                { "name": "gm", "expr": "current * 10" }
              ]
            }"#,
        );

        let manifest = load_primitive_manifest(&manifest_path).unwrap();

        assert_eq!(manifest.files.build.as_deref(), Some("build.json"));
        assert_eq!(manifest.build.unwrap().columns[0].name, "gm");

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn keeps_legacy_build_py_as_metadata_without_loading_build_spec() {
        let dir = make_temp_dir("legacy_build_py");
        let manifest_path = dir.join("primitive.json");
        write_file(
            &manifest_path,
            r#"{
              "name": "simple",
              "pin_order": ["VIN"],
              "ports": {
                "VIN": { "name": "VIN", "role": "INPUT" }
              },
              "files": {
                "build": "build.py",
                "netlist": "netlist/netlist.spice"
              }
            }"#,
        );
        write_file(&dir.join("build.py"), "raise RuntimeError('must not run')");

        let manifest = load_primitive_manifest(&manifest_path).unwrap();

        assert_eq!(manifest.files.build.as_deref(), Some("build.py"));
        assert!(manifest.build.is_none());

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn loads_analoglib_declarative_build_specs() {
        let primitives_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analoglib/primitives");

        let catalog = load_primitive_catalog(&primitives_dir).unwrap();

        for primitive_name in [
            "simplediffpair",
            "simplecurrentsource",
            "simplecurrentmirror",
            "simplecommonsource",
        ] {
            let primitive = catalog.get(primitive_name).unwrap();
            assert_eq!(primitive.files.build.as_deref(), Some("build.json"));
            assert!(primitive.build.is_some());
            assert!(primitive.lut_config.is_some());
        }
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
