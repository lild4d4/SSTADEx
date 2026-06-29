pub mod catalog;
pub mod io;
pub mod model;
pub mod render;
pub mod validation;

pub use catalog::{MacroCatalog, load_macro_catalog};
pub use io::{MacroModelIoError, load_macro_model, save_macro_model};
pub use model::{
    MacroMetadata, MacroModel, MacroPort, MacroPortRole, MacroSmallSignalElement,
    MacroSmallSignalModel, MacroSymbol, MacroSymbolPin,
};
pub use render::{
    MacroRenderError, MacroSmallSignalMode, render_macro_compact_small_signal_netlist,
    render_macro_netlist, render_macro_small_signal_netlist,
    render_macro_small_signal_netlist_with_mode, render_macro_subckt,
    render_macro_testbench_small_signal_netlist,
    render_macro_testbench_small_signal_netlist_with_mode,
};
pub use validation::{MacroValidationError, validate_macro_catalog, validate_macro_model};
