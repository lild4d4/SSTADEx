use serde::{Deserialize, Serialize};

use super::small_signal::SmallSignalModel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub subckt_name: String,
    pub pins: Vec<Pin>,
    pub files: PrimitiveFiles,
    pub ui: PrimitiveUi,
    pub small_signal: Option<SmallSignalModel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pin {
    pub name: String,
    pub role: PinRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PinRole {
    Input,
    Output,
    Bias,
    Supply,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveFiles {
    pub netlist: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveUi {
    pub shape: PrimitiveShape,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrimitiveShape {
    Box,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_primitive_manifest_for_gui_box() {
        let manifest = PrimitiveManifest {
            name: "simplediffpair".to_string(),
            version: "1.0".to_string(),
            description: Some("Simple differential pair".to_string()),
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
            },
            small_signal: Some(SmallSignalModel {
                branches: Vec::new(),
            }),
        };

        assert_eq!(manifest.name, "simplediffpair");
        assert_eq!(manifest.pins.len(), 5);
        assert_eq!(manifest.ui.shape, PrimitiveShape::Box);
    }
}
