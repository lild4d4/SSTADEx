use std::path::Path;

use libsstadex::catalog::load_primitive_catalog;
use libsstadex::macro_model::{load_macro_catalog, render_macro_netlist};

fn main() -> Result<(), String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "failed to find workspace root".to_string())?;

    let primitive_catalog = load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
        .map_err(|error| format!("{error:?}"))?;
    let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros"))
        .map_err(|error| format!("{error:?}"))?;

    let ota = macro_catalog
        .get("ota_1stage")
        .ok_or_else(|| "missing ota_1stage macro".to_string())?;
    let netlist = render_macro_netlist(ota, &primitive_catalog, &macro_catalog)
        .map_err(|error| format!("{error:?}"))?;

    println!("{netlist}");

    Ok(())
}
