use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub primitive: String,
}

impl Instance {
    pub fn new(id: impl Into<String>, primitive: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            primitive: primitive.into(),
        }
    }
}
