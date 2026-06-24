use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    #[serde(flatten)]
    pub block: BlockRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockRef {
    LegacyPrimitive { primitive: String },
    Block { block: BlockKind },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockKind {
    Primitive { name: String },
    Macro { name: String },
}

impl Instance {
    pub fn new(id: impl Into<String>, primitive: impl Into<String>) -> Self {
        Self::primitive(id, primitive)
    }

    pub fn primitive(id: impl Into<String>, primitive: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            block: BlockRef::Block {
                block: BlockKind::Primitive {
                    name: primitive.into(),
                },
            },
        }
    }

    pub fn macro_instance(id: impl Into<String>, macro_name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            block: BlockRef::Block {
                block: BlockKind::Macro {
                    name: macro_name.into(),
                },
            },
        }
    }

    pub fn primitive_name(&self) -> Option<&str> {
        match &self.block {
            BlockRef::LegacyPrimitive { primitive } => Some(primitive),
            BlockRef::Block {
                block: BlockKind::Primitive { name },
            } => Some(name),
            BlockRef::Block {
                block: BlockKind::Macro { .. },
            } => None,
        }
    }

    pub fn macro_name(&self) -> Option<&str> {
        match &self.block {
            BlockRef::LegacyPrimitive { .. } => None,
            BlockRef::Block {
                block: BlockKind::Primitive { .. },
            } => None,
            BlockRef::Block {
                block: BlockKind::Macro { name },
            } => Some(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_legacy_primitive_and_new_macro_instance_json() {
        let legacy = serde_json::from_str::<Instance>(
            r#"{
                "id": "xdp",
                "primitive": "simplediffpair"
            }"#,
        )
        .unwrap();
        let macro_instance = serde_json::from_str::<Instance>(
            r#"{
                "id": "xcs",
                "block": { "type": "macro", "name": "current_source" }
            }"#,
        )
        .unwrap();

        assert_eq!(legacy.primitive_name(), Some("simplediffpair"));
        assert_eq!(legacy.macro_name(), None);
        assert_eq!(macro_instance.primitive_name(), None);
        assert_eq!(macro_instance.macro_name(), Some("current_source"));
    }
}
