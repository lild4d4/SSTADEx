use std::fs;
use std::path::{Path, PathBuf};

use crate::catalog::PrimitiveCatalog;
use crate::circuit::Circuit;
use crate::mna::mna::{MnaError, MnaResult, MnaSolveResult, mna, mna_solve};
use crate::netlist::{SmallSignalRenderError, render_small_signal_netlist};

#[derive(Debug)]
pub struct CircuitMnaAnalysis {
    pub small_signal_netlist: String,
    pub spice_path: PathBuf,
    pub cir_path: PathBuf,
    pub mna: MnaResult,
    pub solution: Option<MnaSolveResult>,
}

#[derive(Debug)]
pub enum CircuitMnaAnalysisError {
    Io(std::io::Error),
    SmallSignalRender(SmallSignalRenderError),
    Mna(MnaError),
}

impl From<std::io::Error> for CircuitMnaAnalysisError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn analyze_circuit_mna(
    circuit: &Circuit,
    catalog: &PrimitiveCatalog,
    output_dir: &Path,
    solve: bool,
) -> Result<CircuitMnaAnalysis, CircuitMnaAnalysisError> {
    let small_signal_netlist = render_small_signal_netlist(circuit, catalog)
        .map_err(CircuitMnaAnalysisError::SmallSignalRender)?;

    fs::create_dir_all(output_dir)?;

    let spice_path = output_dir.join(format!("{}.spice", circuit.name));
    let cir_path = output_dir.join(format!("{}.cir", circuit.name));
    fs::write(&spice_path, &small_signal_netlist)?;

    let mna = mna(output_dir, output_dir, &circuit.name).map_err(CircuitMnaAnalysisError::Mna)?;
    let solution = if solve {
        Some(mna_solve(&mna.a, &mna.x, &mna.z).map_err(CircuitMnaAnalysisError::Mna)?)
    } else {
        None
    };

    Ok(CircuitMnaAnalysis {
        small_signal_netlist,
        spice_path,
        cir_path,
        mna,
        solution,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::load_primitive_catalog;
    use crate::circuit::{Connection, Instance, PinRef, load_circuit};

    #[test]
    fn analyzes_circuit_mna_without_solving() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let primitives_dir = workspace_root.join("analoglib/primitives");
        let circuit_path = workspace_root.join("libsstadex/examples/circuits/ota_primitives.json");
        let output_dir =
            std::env::temp_dir().join(format!("sstadex-circuit-mna-test-{}", std::process::id()));

        let catalog = load_primitive_catalog(&primitives_dir).unwrap();
        let circuit = load_circuit(&circuit_path).unwrap();

        let analysis = analyze_circuit_mna(&circuit, &catalog, &output_dir, false).unwrap();

        assert!(analysis.spice_path.exists());
        assert!(analysis.cir_path.exists());
        assert!(analysis.solution.is_none());
        assert!(
            analysis
                .small_signal_netlist
                .contains("G_gm__xdp__m1 VOUT IBIAS VINP IBIAS gm__xdp__m1")
        );
        assert!(
            analysis
                .mna
                .a
                .iter()
                .flatten()
                .any(|expr| expr.to_string().contains("gm__xdp__m1"))
        );

        fs::remove_dir_all(output_dir).unwrap();
    }

    #[test]
    fn reports_small_signal_render_errors() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let primitives_dir = workspace_root.join("analoglib/primitives");
        let output_dir = std::env::temp_dir().join(format!(
            "sstadex-circuit-mna-error-test-{}",
            std::process::id()
        ));

        let catalog = load_primitive_catalog(&primitives_dir).unwrap();
        let mut circuit = Circuit::new("invalid");
        circuit.add_instance(Instance::new("xdp", "simplediffpair"));
        circuit.connect(Connection::new(PinRef::new("xdp", "VINP"), "VINP"));

        let error = analyze_circuit_mna(&circuit, &catalog, &output_dir, false).unwrap_err();

        assert!(matches!(
            error,
            CircuitMnaAnalysisError::SmallSignalRender(_)
        ));
    }
}
