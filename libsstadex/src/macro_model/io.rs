use std::fs;
use std::path::Path;

use crate::macro_model::MacroModel;

#[derive(Debug)]
pub enum MacroModelIoError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for MacroModelIoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for MacroModelIoError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

pub fn load_macro_model(path: &Path) -> Result<MacroModel, MacroModelIoError> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn save_macro_model(path: &Path, macro_model: &MacroModel) -> Result<(), MacroModelIoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(macro_model)?;
    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_current_source_macro_json() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let path = workspace_root.join("analoglib/macros/current_source/macro.json");

        let macro_model = load_macro_model(&path).unwrap();

        assert_eq!(macro_model.name, "current_source");
        assert_eq!(macro_model.subckt_name, "current_source");
        assert_eq!(macro_model.ports.len(), 3);
        assert_eq!(macro_model.circuit.instances.len(), 1);
        assert_eq!(
            macro_model.circuit.instances[0].primitive_name(),
            Some("simplecurrentsource")
        );
        assert_eq!(macro_model.circuit.connections.len(), 3);
    }
}
