pub mod catalog;
pub mod io;
pub mod model;
pub mod render;
pub mod validation;

pub use catalog::{load_macro_catalog, MacroCatalog};
pub use io::{load_macro_model, save_macro_model, MacroModelIoError};
pub use model::{MacroMetadata, MacroModel, MacroPort, MacroPortRole, MacroSymbol, MacroSymbolPin};
pub use render::{
    render_macro_netlist, render_macro_small_signal_netlist, render_macro_subckt, MacroRenderError,
};
pub use validation::{validate_macro_catalog, validate_macro_model, MacroValidationError};
