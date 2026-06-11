use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmallSignalModel {
    pub branches: Vec<SmallSignalBranch>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmallSignalBranch {
    pub name: String,
    pub vd: String,
    pub vg: String,
    pub vs: String,
}
