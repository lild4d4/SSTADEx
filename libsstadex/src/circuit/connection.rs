use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinRef {
    pub instance: String,
    pub pin: String,
}

impl PinRef {
    pub fn new(instance: impl Into<String>, pin: impl Into<String>) -> Self {
        Self {
            instance: instance.into(),
            pin: pin.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Connection {
    pub from: PinRef,
    pub net: String,
}

impl Connection {
    pub fn new(from: PinRef, net: impl Into<String>) -> Self {
        Self {
            from,
            net: net.into(),
        }
    }
}
