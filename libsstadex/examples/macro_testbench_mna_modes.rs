use std::path::Path;

use libsstadex::analysis::{analyze_macro_testbench_mna, analyze_macro_testbench_mna_with_mode};
use libsstadex::catalog::load_primitive_catalog;
use libsstadex::exploration::{TestbenchElement, TestbenchSpec};
use libsstadex::macro_model::{MacroSmallSignalMode, load_macro_catalog};

fn main() -> Result<(), String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "failed to find workspace root".to_string())?;
    let output_dir = std::env::temp_dir().join(format!(
        "sstadex-macro-testbench-mna-{}",
        std::process::id()
    ));

    let primitive_catalog = load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
        .map_err(|error| format!("{error:?}"))?;
    let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros"))
        .map_err(|error| format!("{error:?}"))?;
    let testbench = TestbenchSpec::new("ota_gain")
        .with_macro_dut("ota_1stage")
        .with_element(TestbenchElement::VoltageSource {
            name: "Vdd".to_string(),
            nplus: "VDD".to_string(),
            nminus: "VSS".to_string(),
            value: "0".to_string(),
        })
        .with_element(TestbenchElement::VoltageSource {
            name: "Vbias".to_string(),
            nplus: "VBIAS".to_string(),
            nminus: "VSS".to_string(),
            value: "0".to_string(),
        })
        .with_element(TestbenchElement::VoltageSource {
            name: "Vin".to_string(),
            nplus: "VINP".to_string(),
            nminus: "VSS".to_string(),
            value: "1".to_string(),
        });

    let compact = analyze_macro_testbench_mna(
        &testbench,
        &primitive_catalog,
        &macro_catalog,
        &output_dir.join("compact"),
        false,
    )
    .map_err(|error| format!("{error:?}"))?;
    let expanded = analyze_macro_testbench_mna_with_mode(
        &testbench,
        &primitive_catalog,
        &macro_catalog,
        &output_dir.join("expand"),
        false,
        MacroSmallSignalMode::Expand,
    )
    .map_err(|error| format!("{error:?}"))?;

    println!("Compact/default SPICE: {}", compact.spice_path.display());
    println!("Compact/default CIR: {}", compact.cir_path.display());
    println!("Expanded SPICE: {}", expanded.spice_path.display());
    println!("Expanded CIR: {}", expanded.cir_path.display());
    println!();
    println!(
        "compact contains gm__xcs_macro: {}",
        compact.small_signal_netlist.contains("gm__xcs_macro")
    );
    println!(
        "compact contains gm__xcs_macro__xcs__m1: {}",
        compact
            .small_signal_netlist
            .contains("gm__xcs_macro__xcs__m1")
    );
    println!(
        "expand contains gm__xcs_macro__xcs__m1: {}",
        expanded
            .small_signal_netlist
            .contains("gm__xcs_macro__xcs__m1")
    );

    Ok(())
}
