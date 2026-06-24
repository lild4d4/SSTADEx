pub mod catalog;
pub mod io;
pub mod model;
pub mod validation;

pub use catalog::{load_macro_catalog, MacroCatalog};
pub use io::{load_macro_model, save_macro_model, MacroModelIoError};
pub use model::{MacroMetadata, MacroModel, MacroPort, MacroPortRole, MacroSymbol, MacroSymbolPin};
pub use validation::{validate_macro_catalog, validate_macro_model, MacroValidationError};
