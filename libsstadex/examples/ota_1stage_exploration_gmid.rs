use std::{collections::HashMap, path::Path};

use libsstadex::{
    catalog::load_primitive_catalog,
    circuit::load_circuit,
    exploration::{
        CandidatePoint, ExplorationSpec, PreparedSpec, PreparedSpecSource, RangeCondition,
        SpecOutput, SpecParameter, SpecSource, TestbenchElement, TestbenchSpec,
        build_filtered_candidates, prepare_transfer_function_spec, run_prepared_expression_flow,
        shared_node_filter,
    },
    primitive::build::{
        PrimitiveBuildEngine, PrimitiveBuildInput, PrimitiveBuildValue, PythonGmidLutBackend,
    },
};

const VDD: f64 = 1.2;
const VIN: f64 = 0.9;
const VOUT: f64 = 1.0;

const CURRENT: f64 = 20.0e-6;
const VINP_START: f64 = 0.55;
const VINP_STOP: f64 = 0.75;
const VOUTP_START: f64 = 0.75;
const VOUTP_STOP: f64 = 0.95;
const POINTS: usize = 5;
const VTAIL: f64 = 0.60;

fn main() -> Result<(), String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("libsstadex should be inside the workspace");
    let primitives_dir = workspace_root.join("analoglib/primitives");
    let lut_files = HashMap::from([
        (
            "nmos".to_string(),
            workspace_root.join("LUTs/ihp-sg13g2/lv_5w_nmos.npz"),
        ),
        (
            "pmos".to_string(),
            workspace_root.join("LUTs/ihp-sg13g2/lv_5w_pmos.npz"),
        ),
    ]);
    let python = workspace_root.join(".venv-sstadex/bin/python");
    let circuit_path = workspace_root.join("libsstadex/examples/circuits/ota_primitives.json");
    let output_dir = std::env::temp_dir().join(format!(
        "libsstadex_ota_1stage_exploration_gmid_{}",
        std::process::id()
    ));

    let catalog = load_primitive_catalog(&primitives_dir).map_err(|error| format!("{error:?}"))?;
    let simplediffpair = catalog
        .get("simplediffpair")
        .ok_or_else(|| "missing simplediffpair primitive".to_string())?;
    let simplecurrentmirror = catalog
        .get("simplecurrentmirror")
        .ok_or_else(|| "missing simplediffpair primitive".to_string())?;

    let simplediffpair_input = PrimitiveBuildInput::new(HashMap::from([
        ("current".to_string(), PrimitiveBuildValue::Scalar(CURRENT)),
        ("VINP".to_string(), PrimitiveBuildValue::Scalar(VIN)),
        ("VOUTP".to_string(), PrimitiveBuildValue::Scalar(VOUT)),
        (
            "VTAIL".to_string(),
            PrimitiveBuildValue::Vector(vec![VTAIL]),
        ),
    ]));

    let simplecurrentmirror_input = PrimitiveBuildInput::new(HashMap::from([
        ("current".to_string(), PrimitiveBuildValue::Scalar(CURRENT)),
        ("VINP".to_string(), PrimitiveBuildValue::Scalar(VOUT)),
        ("VOUTP".to_string(), PrimitiveBuildValue::Scalar(VOUT)),
        ("VDD".to_string(), PrimitiveBuildValue::Scalar(VDD)),
    ]));

    let engine =
        PrimitiveBuildEngine::new(PythonGmidLutBackend::with_default_helper(python, lut_files));

    let simplediffpair_candidate_set = engine
        .build_candidate_set_for_primitive(simplediffpair, "xdp", &simplediffpair_input)
        .map_err(|error| format!("{error:?}"))?;
    let simplecurrentmirror_candidate_set = engine
        .build_candidate_set_for_primitive(simplecurrentmirror, "xcm", &simplecurrentmirror_input)
        .map_err(|error| format!("{error:?}"))?;

    let circuit = load_circuit(&circuit_path).map_err(|error| format!("{error:?}"))?;
    let gain_spec = gain_spec();
    let prepared_gain = prepare_transfer_function_spec(&gain_spec, &circuit, &catalog, &output_dir)
        .map_err(|error| format!("{error:?}"))?;

    //let filters = vec![
    //    shared_node_filter(vec!["xdp.voutp", "xcm.voutp"]),
    //    shared_node_filter(vec!["xdp.voutp", "xcm.vinp"]),
    //];

    let table = run_prepared_expression_flow(
        &[],
        &[
            simplediffpair_candidate_set,
            simplecurrentmirror_candidate_set,
        ],
        &[],
        &[prepared_gain.clone()],
    )
    .map_err(|error| format!("{error:?}"))?;

    println!("Prepared gain TF:");
    println!("{}", prepared_expression(&prepared_gain));
    println!();
    println!("OTA 1-stage exploration rows:");

    for row in 0..table.row_count {
        let gain = table.column("gain_1stage").unwrap().values[row];
        let gain_db = 20.0 * gain.abs().log10();
        let xdp_vinp = table.column("xdp.vinp").unwrap().values[row];
        let xdp_voutp = table.column("xdp.voutp").unwrap().values[row];
        let xcm_voutp = table.column("xcm.voutp").unwrap().values[row];
        let xcm_vinp = table.column("xcm.vinp").unwrap().values[row];
        //let gbw = 1.0 / (2.0 * std::f64::consts::PI * rout.abs() * CL);

        println!(
            "row={row} xdp.vinp={xdp_vinp:.3} xdp.voutp={xdp_voutp:.3} xcm.voutp={xcm_voutp:.3} xcm.vinp={xcm_vinp:.3} gain={gain:.6e} gain_db={gain_db:.3}"
        );
    }

    Ok(())
}

fn linspace(start: f64, stop: f64, points: usize) -> Vec<f64> {
    match points {
        0 => Vec::new(),
        1 => vec![start],
        _ => {
            let step = (stop - start) / (points - 1) as f64;
            (0..points).map(|idx| start + step * idx as f64).collect()
        }
    }
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
                .with_element(voltage_source("V_p", "VINP", "VSS", "vin"))
                .with_element(current_source("Ibias", "IBIAS", "VSS", "0")),
            input: "VINP".to_string(),
            output: "VOUT".to_string(),
        },
        SpecOutput::Eval,
    );
    spec.parameter_map = ota_parameter_map(vec![("vin", "1")]);
    spec
}

fn ota_parameter_map(extra: Vec<(&str, &str)>) -> Vec<SpecParameter> {
    let mut parameters = vec![
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

fn current_source(name: &str, nplus: &str, nminus: &str, value: &str) -> TestbenchElement {
    TestbenchElement::CurrentSource {
        name: name.to_string(),
        nplus: nplus.to_string(),
        nminus: nminus.to_string(),
        value: value.to_string(),
    }
}

fn prepared_expression(prepared: &PreparedSpec) -> &str {
    match &prepared.source {
        PreparedSpecSource::CandidateExpression { expression }
        | PreparedSpecSource::TransferFunction { expression } => expression,
        PreparedSpecSource::Composed => "",
    }
}
