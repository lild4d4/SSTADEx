use std::fs;
use std::path::Path;

use libsstadex::catalog::load_primitive_catalog;
use libsstadex::macro_model::{
    MacroSmallSignalMode, load_macro_catalog, render_macro_small_signal_netlist,
    render_macro_small_signal_netlist_with_mode,
};

fn main() -> Result<(), String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "failed to find workspace root".to_string())?;
    let output_dir =
        std::env::temp_dir().join(format!("sstadex-macro-modes-{}", std::process::id()));

    let primitive_catalog = load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
        .map_err(|error| format!("{error:?}"))?;
    let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros"))
        .map_err(|error| format!("{error:?}"))?;
    let ota = macro_catalog
        .get("ota_1stage")
        .ok_or_else(|| "failed to load ota_1stage macro".to_string())?;

    let default_netlist =
        render_macro_small_signal_netlist(ota, &primitive_catalog, &macro_catalog)
            .map_err(|error| format!("{error:?}"))?;
    let expanded_netlist = render_macro_small_signal_netlist_with_mode(
        ota,
        &primitive_catalog,
        &macro_catalog,
        MacroSmallSignalMode::Expand,
    )
    .map_err(|error| format!("{error:?}"))?;

    fs::create_dir_all(&output_dir).map_err(|error| format!("{error:?}"))?;
    let default_path = output_dir.join("ota_1stage_default.spice");
    let expanded_path = output_dir.join("ota_1stage_expand.spice");
    fs::write(&default_path, &default_netlist).map_err(|error| format!("{error:?}"))?;
    fs::write(&expanded_path, &expanded_netlist).map_err(|error| format!("{error:?}"))?;

    println!("Default small-signal netlist: {}", default_path.display());
    println!("Expanded small-signal netlist: {}", expanded_path.display());
    println!();
    println!(
        "default contains gm__xcs_macro: {}",
        default_netlist.contains("gm__xcs_macro")
    );
    println!(
        "default contains gm__xcs_macro__xcs__m1: {}",
        default_netlist.contains("gm__xcs_macro__xcs__m1")
    );
    println!(
        "expand contains gm__xcs_macro__xcs__m1: {}",
        expanded_netlist.contains("gm__xcs_macro__xcs__m1")
    );

    Ok(())
}
