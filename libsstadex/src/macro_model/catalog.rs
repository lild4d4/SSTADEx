use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::macro_model::{MacroModel, MacroModelIoError, load_macro_model};

#[derive(Debug, Clone, Default)]
pub struct MacroCatalog {
    macros: HashMap<String, MacroModel>,
}

impl MacroCatalog {
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }

    pub fn register(&mut self, macro_model: MacroModel) {
        self.macros.insert(macro_model.name.clone(), macro_model);
    }

    pub fn get(&self, name: &str) -> Option<&MacroModel> {
        self.macros.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    pub fn list(&self) -> Vec<&MacroModel> {
        let mut macros = self.macros.values().collect::<Vec<_>>();
        macros.sort_by(|left, right| left.name.cmp(&right.name));
        macros
    }
}

pub fn load_macro_catalog(macros_dir: &Path) -> Result<MacroCatalog, MacroModelIoError> {
    let mut catalog = MacroCatalog::new();

    for entry in fs::read_dir(macros_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let manifest_path = path.join("macro.json");
        if !manifest_path.exists() {
            continue;
        }

        let Ok(macro_model) = load_macro_model(&manifest_path) else {
            continue;
        };
        catalog.register(macro_model);
    }

    Ok(catalog)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_declarative_macros_and_ignores_legacy_python_descriptors() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let macros_dir = workspace_root.join("analoglib/macros");

        let catalog = load_macro_catalog(&macros_dir).unwrap();

        assert!(catalog.contains("current_source"));
        assert!(catalog.contains("ota_1stage"));
        assert!(!catalog.contains("current_source_macro"));
        assert!(!catalog.contains("ota_1stage_macro"));
        assert_eq!(catalog.list().len(), 2);
    }
}
