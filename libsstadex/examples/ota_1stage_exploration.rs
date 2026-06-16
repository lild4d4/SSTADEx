use std::fs;
use std::path::Path;

use libsstadex::catalog::load_primitive_catalog;
use libsstadex::circuit::load_circuit;
use libsstadex::exploration::{
    CandidatePoint, CandidateSet, ExplorationSpec, PreparedSpec, PreparedSpecSource,
    RangeCondition, SpecOutput, SpecParameter, SpecSource, TestbenchElement, TestbenchSpec,
    prepare_transfer_function_spec, run_prepared_expression_flow,
};

const CL: f64 = 1e-12;

fn main() -> Result<(), String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("libsstadex should be inside the workspace");
    let primitives_dir = workspace_root.join("analoglib/primitives");
    let circuit_path = workspace_root.join("libsstadex/examples/circuits/ota_primitives.json");
    let output_dir = std::env::temp_dir().join(format!(
        "libsstadex_ota_1stage_exploration_{}",
        std::process::id()
    ));

    let catalog = load_primitive_catalog(&primitives_dir).map_err(|error| format!("{error:?}"))?;
    let circuit = load_circuit(&circuit_path).map_err(|error| format!("{error:?}"))?;

    let gain_spec = gain_spec();
    let rout_spec = rout_spec();
    let prepared_gain = prepare_transfer_function_spec(&gain_spec, &circuit, &catalog, &output_dir)
        .map_err(|error| format!("{error:?}"))?;
    let prepared_rout = prepare_transfer_function_spec(&rout_spec, &circuit, &catalog, &output_dir)
        .map_err(|error| format!("{error:?}"))?;
    let candidates = vec![
        operating_point(1.0e-3, 1.0e5, 8.0e-4, 1.2e5, 6.0e-4, 1.5e5),
        operating_point(1.5e-3, 8.0e4, 1.0e-3, 1.0e5, 8.0e-4, 1.2e5),
        operating_point(2.0e-3, 6.0e4, 1.2e-3, 9.0e4, 1.0e-3, 1.0e5),
    ];

    let candidate_set = CandidateSet::new("ota_1stage_manual_lut_points", candidates.clone());
    let table = run_prepared_expression_flow(
        &[],
        &[candidate_set],
        &[],
        &[prepared_gain.clone(), prepared_rout.clone()],
    )
    .map_err(|error| format!("{error:?}"))?;

    println!("Prepared gain TF:");
    println!("{}", prepared_expression(&prepared_gain));
    println!();
    println!("Prepared rout TF:");
    println!("{}", prepared_expression(&prepared_rout));
    println!();
    println!("OTA 1-stage exploration rows:");

    for row in 0..table.row_count {
        let gain = table.column("gain_1stage").unwrap().values[row];
        let rout_ratio = table.column("rout_1stage").unwrap().values[row];
        let rout = rout_ratio * 1000.0 / (1.0 - rout_ratio);
        let gm = gain / rout;
        let gain_db = 20.0 * gain.abs().log10();
        let gbw = 1.0 / (2.0 * std::f64::consts::PI * rout.abs() * CL);

        println!(
            "row={row} gain={gain:.6e} gain_db={gain_db:.3} rout={rout:.6e} gm={gm:.6e} gbw={gbw:.6e}"
        );
    }

    let _ = fs::remove_dir_all(output_dir);

    Ok(())
}

fn gain_spec() -> ExplorationSpec {
    let mut spec = ExplorationSpec::new(
        "gain_1stage",
        RangeCondition::min(10_f64.powf(-100.0 / 20.0)),
        SpecSource::TransferFunction {
            testbench: TestbenchSpec::new("ota_1stage_gain")
                .with_element(voltage_source("Vdd", "VDD", "VSS", "0"))
                .with_element(voltage_source("Vss", "VSS", "VSS", "0"))
                .with_element(voltage_source("V_n", "VINN", "VSS", "0"))
                .with_element(voltage_source("V_p", "VINP", "VSS", "vin")),
            input: "VINP".to_string(),
            output: "VOUT".to_string(),
        },
        SpecOutput::Eval,
    );
    spec.parameter_map = ota_parameter_map(vec![("vin", "1")]);
    spec
}

fn rout_spec() -> ExplorationSpec {
    let mut spec = ExplorationSpec::new(
        "rout_1stage",
        RangeCondition::min(1.0e-12),
        SpecSource::TransferFunction {
            testbench: TestbenchSpec::new("ota_1stage_rout")
                .with_element(voltage_source("Vdd", "VDD", "VSS", "0"))
                .with_element(voltage_source("Vss", "VSS", "VSS", "0"))
                .with_element(voltage_source("V_n", "VINN", "VSS", "0"))
                .with_element(voltage_source("V_p", "VINP", "VSS", "0"))
                .with_element(voltage_source("Vr", "VR", "VSS", "vr"))
                .with_element(TestbenchElement::Resistor {
                    name: "Rr".to_string(),
                    n1: "VR".to_string(),
                    n2: "VOUT".to_string(),
                    value: "1000".to_string(),
                }),
            input: "VR".to_string(),
            output: "VOUT".to_string(),
        },
        SpecOutput::Eval,
    );
    spec.parameter_map = ota_parameter_map(vec![("vr", "1")]);
    spec
}

fn ota_parameter_map(extra: Vec<(&str, &str)>) -> Vec<SpecParameter> {
    let mut parameters = vec![
        SpecParameter::new("v6", "0"),
        SpecParameter::new("gm__xdp__m2", "gm__xdp__m1"),
        SpecParameter::new("ro__xdp__m2", "ro__xdp__m1"),
        SpecParameter::new("gm__xcm__m2", "gm__xcm__m1"),
        SpecParameter::new("ro__xcm__m2", "ro__xcm__m1"),
        SpecParameter::new("s", "0"),
    ];

    parameters.extend(
        extra
            .into_iter()
            .map(|(name, value)| SpecParameter::new(name, value)),
    );

    parameters
}

fn voltage_source(name: &str, nplus: &str, nminus: &str, value: &str) -> TestbenchElement {
    TestbenchElement::VoltageSource {
        name: name.to_string(),
        nplus: nplus.to_string(),
        nminus: nminus.to_string(),
        value: value.to_string(),
    }
}

fn operating_point(
    xdp_gm: f64,
    xdp_ro: f64,
    xcm_gm: f64,
    xcm_ro: f64,
    xcs_gm: f64,
    xcs_ro: f64,
) -> CandidatePoint {
    CandidatePoint::new(vec![
        ("gm__xdp__m1".to_string(), xdp_gm),
        ("ro__xdp__m1".to_string(), xdp_ro),
        ("gm__xcm__m1".to_string(), xcm_gm),
        ("ro__xcm__m1".to_string(), xcm_ro),
        ("gm__xcs__m1".to_string(), xcs_gm),
        ("ro__xcs__m1".to_string(), xcs_ro),
        ("gm__xcs__m2".to_string(), xcs_gm),
        ("ro__xcs__m2".to_string(), xcs_ro),
    ])
}

fn prepared_expression(prepared: &PreparedSpec) -> &str {
    match &prepared.source {
        PreparedSpecSource::CandidateExpression { expression }
        | PreparedSpecSource::TransferFunction { expression } => expression,
        PreparedSpecSource::Composed => "",
    }
}
