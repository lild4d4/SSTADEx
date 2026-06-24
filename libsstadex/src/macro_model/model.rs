use serde::{Deserialize, Serialize};

use crate::circuit::Circuit;
use crate::primitive::manifest::SymbolPinSide;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacroModel {
    pub name: String,
    pub subckt_name: String,
    pub ports: Vec<MacroPort>,
    pub circuit: Circuit,
    pub symbol: Option<MacroSymbol>,
    pub metadata: MacroMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroPort {
    pub name: String,
    pub role: MacroPortRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroPortRole {
    Input,
    Output,
    Inout,
    Bias,
    Supply,
    Ground,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroMetadata {
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacroSymbol {
    pub pins: Vec<MacroSymbolPin>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacroSymbolPin {
    pub name: String,
    pub side: SymbolPinSide,
    pub offset: f32,
}

impl MacroModel {
    pub fn new(name: impl Into<String>, ports: Vec<MacroPort>, circuit: Circuit) -> Self {
        let name = name.into();

        Self {
            subckt_name: name.clone(),
            name,
            ports,
            circuit,
            symbol: None,
            metadata: MacroMetadata::default(),
        }
    }
}

impl MacroPort {
    pub fn new(name: impl Into<String>, role: MacroPortRole) -> Self {
        Self {
            name: name.into(),
            role,
        }
    }
}

impl Default for MacroMetadata {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            description: None,
        }
    }
}
