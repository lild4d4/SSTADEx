use std::fs;
use std::path::Path;

use libsstadex::catalog::load_primitive_catalog;
use libsstadex::circuit::load_circuit;
use libsstadex::exploration::{
    CandidatePoint, CandidateSet, ExplorationSpec, RangeCondition, SpecOutput, SpecSource,
    TestbenchElement, TestbenchSpec, evaluate_candidate_expression, prepare_transfer_function_spec,
    run_prepared_expression_flow,
};

fn main() -> Result<(), String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("libsstadex should be inside the workspace");
    let primitives_dir = workspace_root.join("analoglib/primitives");
    let circuit_path = workspace_root.join("libsstadex/examples/circuits/ota_primitives.json");
    let output_dir = std::env::temp_dir().join(format!(
        "libsstadex_exploration_mna_end_to_end_{}",
        std::process::id()
    ));

    let catalog = load_primitive_catalog(&primitives_dir).map_err(|error| format!("{error:?}"))?;
    let circuit = load_circuit(&circuit_path).map_err(|error| format!("{error:?}"))?;
    let spec = ExplorationSpec::new(
        "gain",
        RangeCondition::min(0.0),
        SpecSource::TransferFunction {
            testbench: TestbenchSpec::new("ota_gain").with_element(
                TestbenchElement::VoltageSource {
                    name: "Vin".to_string(),
                    nplus: "VINP".to_string(),
                    nminus: "VINN".to_string(),
                    value: "vin".to_string(),
                },
            ),
            input: "VINP".to_string(),
            output: "VOUT".to_string(),
        },
        SpecOutput::Eval,
    );

    let prepared = prepare_transfer_function_spec(&spec, &circuit, &catalog, &output_dir)
        .map_err(|error| format!("{error:?}"))?;
    let expression = match &prepared.source {
        libsstadex::exploration::PreparedSpecSource::TransferFunction { expression } => {
            expression.clone()
        }
        _ => unreachable!("expected transfer-function prepared spec"),
    };

    let candidate = CandidatePoint::new(vec![
        ("vin".to_string(), 1.0),
        ("v6".to_string(), 0.0),
        ("gm__xdp__m1".to_string(), 1.0e-3),
        ("ro__xdp__m1".to_string(), 1.0e5),
        ("gm__xdp__m2".to_string(), 1.0e-3),
        ("ro__xdp__m2".to_string(), 1.0e5),
        ("gm__xcm__m1".to_string(), 1.0e-3),
        ("ro__xcm__m1".to_string(), 1.0e5),
        ("gm__xcm__m2".to_string(), 1.0e-3),
        ("ro__xcm__m2".to_string(), 1.0e5),
        ("gm__xcs__m1".to_string(), 1.0e-3),
        ("ro__xcs__m1".to_string(), 1.0e5),
        ("gm__xcs__m2".to_string(), 1.0e-3),
        ("ro__xcs__m2".to_string(), 1.0e5),
    ]);
    let gain_value = evaluate_candidate_expression(&candidate, 0, &expression)
        .map_err(|error| format!("{error:?}"))?;

    let set = CandidateSet::new("operating_points", vec![candidate]);
    let table = run_prepared_expression_flow(&[], &[set], &[], &[prepared])
        .map_err(|error| format!("{error:?}"))?;

    assert_eq!(table.row_count, 1);
    assert!(table.column("gain").is_some());
    assert_eq!(table.column("gain").unwrap().values, vec![gain_value]);
    assert!(gain_value.is_finite());

    println!("Prepared transfer function:");
    println!("{expression}");
    println!("Evaluated gain: {gain_value}");
    println!("Output directory: {}", output_dir.display());

    let _ = fs::remove_dir_all(output_dir);

    Ok(())
}
