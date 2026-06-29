use std::path::Path;

use libsstadex::catalog::PrimitiveCatalog;
use libsstadex::catalog::load_primitive_catalog;
use libsstadex::exploration::{
    ExplorationSpec, PreparedSpecSource, RangeCondition, SpecOutput, SpecSource, TestbenchElement,
    TestbenchSpec, prepare_macro_testbench_specs, prepare_macro_testbench_specs_with_mode,
};
use libsstadex::macro_model::{MacroCatalog, MacroSmallSignalMode, load_macro_catalog};

fn main() -> Result<(), String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "failed to find workspace root".to_string())?;
    let output_dir = std::env::temp_dir().join(format!(
        "libsstadex_ota_1stage_macro_prepare_{}",
        std::process::id()
    ));

    let primitive_catalog = load_primitive_catalog(&workspace_root.join("analoglib/primitives"))
        .map_err(|error| format!("{error:?}"))?;
    let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros"))
        .map_err(|error| format!("{error:?}"))?;
    let specs = vec![ExplorationSpec::new(
        "gain",
        RangeCondition::min(0.0),
        SpecSource::TransferFunction {
            testbench: TestbenchSpec::new("ota_gain")
                .with_macro_dut("ota_1stage")
                .with_element(TestbenchElement::VoltageSource {
                    name: "Vdd".to_string(),
                    nplus: "VDD".to_string(),
                    nminus: "VSS".to_string(),
                    value: "0".to_string(),
                })
                .with_element(TestbenchElement::VoltageSource {
                    name: "Vss".to_string(),
                    nplus: "VSS".to_string(),
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
                    value: "vin".to_string(),
                }),
            input: "VINP".to_string(),
            output: "VOUT".to_string(),
        },
        SpecOutput::Eval,
    )];

    print_prepared_expression(
        "CompactWhenAvailable default",
        &specs,
        &primitive_catalog,
        &macro_catalog,
        &output_dir.join("compact"),
        None,
    )?;
    print_prepared_expression(
        "Expand",
        &specs,
        &primitive_catalog,
        &macro_catalog,
        &output_dir.join("expand"),
        Some(MacroSmallSignalMode::Expand),
    )?;

    println!("Output directory: {}", output_dir.display());

    Ok(())
}

fn print_prepared_expression(
    label: &str,
    specs: &[ExplorationSpec],
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    output_dir: &Path,
    mode: Option<MacroSmallSignalMode>,
) -> Result<(), String> {
    let prepared_specs = match mode {
        Some(mode) => prepare_macro_testbench_specs_with_mode(
            specs,
            primitive_catalog,
            macro_catalog,
            output_dir,
            mode,
        ),
        None => prepare_macro_testbench_specs(specs, primitive_catalog, macro_catalog, output_dir),
    }
    .map_err(format_prepare_error)?;
    let expression = prepared_transfer_function_expression(&prepared_specs)?;

    println!("Prepared transfer function for ota_1stage macro ({label}):");
    println!("{expression}");
    println!();

    Ok(())
}

fn prepared_transfer_function_expression(
    prepared_specs: &[libsstadex::exploration::PreparedSpec],
) -> Result<&str, String> {
    let prepared = prepared_specs
        .first()
        .ok_or_else(|| "expected one prepared spec".to_string())?;

    match &prepared.source {
        PreparedSpecSource::TransferFunction { expression } => Ok(expression),
        _ => unreachable!("expected transfer-function prepared spec"),
    }
}

fn format_prepare_error(error: impl std::fmt::Debug) -> String {
    let reason = format!("{error:?}");
    if reason.contains("No module named 'sympy'") {
        format!(
            "{reason}\n\nThis example requires Python package `sympy` in the `python3` environment used by SSTADEx."
        )
    } else {
        reason
    }
}
