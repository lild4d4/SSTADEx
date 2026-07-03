use std::collections::HashMap;
use std::path::{Path, PathBuf};

use libsstadex::catalog::load_primitive_catalog;
use libsstadex::exploration::{
    CandidatePoint, PreparedSpec, PreparedSpecSource, RangeCondition, SpecOutput,
    run_prepared_expression_flow,
};
use libsstadex::primitive::build::{
    PrimitiveBuildEngine, PrimitiveBuildInput, PrimitiveBuildValue, PythonGmidLutBackend,
};

const CURRENT: f64 = 200.0e-6;
const VINP_START: f64 = 0.55;
const VINP_STOP: f64 = 0.75;
const VOUTP_START: f64 = 0.75;
const VOUTP_STOP: f64 = 0.95;
const POINTS: usize = 10;
const VTAIL: f64 = 0.10;

fn main() -> Result<(), String> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("libsstadex should be inside the workspace");
    let primitives_dir = workspace_root.join("analoglib/primitives");
    let python = workspace_root.join(".venv-sstadex/bin/python");
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

    require_file(&python)?;
    for path in lut_files.values() {
        require_file(path)?;
    }

    let catalog = load_primitive_catalog(&primitives_dir).map_err(|error| format!("{error:?}"))?;
    let primitive = catalog
        .get("simplediffpair")
        .ok_or_else(|| "missing simplediffpair primitive".to_string())?;
    let build_input = PrimitiveBuildInput::new(HashMap::from([
        ("current".to_string(), PrimitiveBuildValue::Scalar(CURRENT)),
        (
            "VINP".to_string(),
            PrimitiveBuildValue::Vector(linspace(VINP_START, VINP_STOP, POINTS)),
        ),
        (
            "VOUTP".to_string(),
            PrimitiveBuildValue::Vector(linspace(VOUTP_START, VOUTP_STOP, POINTS)),
        ),
        (
            "VTAIL".to_string(),
            PrimitiveBuildValue::Vector(vec![VTAIL]),
        ),
    ]));

    let engine = PrimitiveBuildEngine::new(
        PythonGmidLutBackend::with_default_helper(python, lut_files).with_timing_output(true),
    );
    let candidate_set = engine
        .build_candidate_set_for_primitive(primitive, "xdp", &build_input)
        .map_err(|error| format!("{error:?}"))?;
    let candidate_columns = CandidatePoint::to_columns(&candidate_set.points);
    let gain_proxy = PreparedSpec::new(
        "gain_proxy",
        RangeCondition::min(0.0),
        SpecOutput::Eval,
        PreparedSpecSource::CandidateExpression {
            expression: "xdp.gm * xdp.Ro".to_string(),
        },
    );
    let table = run_prepared_expression_flow(&[], &[candidate_set.clone()], &[], &[gain_proxy])
        .map_err(|error| format!("{error:?}"))?;

    println!("Primitive: {}", primitive.name);
    println!(
        "Sweep: VINP={} points, VOUTP={} points, cartesian operating points={}",
        POINTS,
        POINTS,
        POINTS * POINTS
    );
    println!(
        "Rows generated with real gmid LUT backend: {}",
        candidate_set.points.len()
    );
    println!("Columns emitted:");
    for column in &candidate_columns {
        println!("  {}", column.name);
    }
    println!();

    for row in 0..table.row_count {
        let length = table.column("xdp.length").unwrap().values[row];
        let width = table.column("xdp.width").unwrap().values[row];
        let gm = table.column("xdp.gm").unwrap().values[row];
        let ro = table.column("xdp.Ro").unwrap().values[row];
        let gain_proxy = table.column("gain_proxy").unwrap().values[row];

        println!(
            "row={row} length={length:.3e} width={width:.3e} gm={gm:.3e} Ro={ro:.3e} gain_proxy={gain_proxy:.3e}"
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

fn require_file(path: &PathBuf) -> Result<(), String> {
    if path.exists() {
        Ok(())
    } else {
        Err(format!("required file does not exist: {}", path.display()))
    }
}
