use std::fs;
use std::path::{Path, PathBuf};

use crate::catalog::PrimitiveCatalog;
use crate::circuit::Circuit;
use crate::exploration::TestbenchSpec;
use crate::macro_model::{
    MacroCatalog, MacroModel, MacroRenderError, MacroSmallSignalMode,
    render_macro_small_signal_netlist_with_mode,
    render_macro_testbench_small_signal_netlist_with_mode,
};
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
    MacroRender(MacroRenderError),
    Mna(MnaError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferFunctionError {
    MissingInputNode { node: String },
    MissingOutputNode { node: String },
    MissingInputSolution { variable: String },
    MissingOutputSolution { variable: String },
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

    analyze_small_signal_netlist_mna(&circuit.name, small_signal_netlist, output_dir, solve)
}

pub fn analyze_macro_mna(
    macro_model: &MacroModel,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    output_dir: &Path,
    solve: bool,
) -> Result<CircuitMnaAnalysis, CircuitMnaAnalysisError> {
    analyze_macro_mna_with_mode(
        macro_model,
        primitive_catalog,
        macro_catalog,
        output_dir,
        solve,
        MacroSmallSignalMode::CompactWhenAvailable,
    )
}

pub fn analyze_macro_mna_with_mode(
    macro_model: &MacroModel,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    output_dir: &Path,
    solve: bool,
    mode: MacroSmallSignalMode,
) -> Result<CircuitMnaAnalysis, CircuitMnaAnalysisError> {
    let small_signal_netlist = render_macro_small_signal_netlist_with_mode(
        macro_model,
        primitive_catalog,
        macro_catalog,
        mode,
    )
    .map_err(CircuitMnaAnalysisError::MacroRender)?;

    analyze_small_signal_netlist_mna(&macro_model.name, small_signal_netlist, output_dir, solve)
}

pub fn analyze_macro_testbench_mna(
    testbench: &TestbenchSpec,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    output_dir: &Path,
    solve: bool,
) -> Result<CircuitMnaAnalysis, CircuitMnaAnalysisError> {
    analyze_macro_testbench_mna_with_mode(
        testbench,
        primitive_catalog,
        macro_catalog,
        output_dir,
        solve,
        MacroSmallSignalMode::CompactWhenAvailable,
    )
}

pub fn analyze_macro_testbench_mna_with_mode(
    testbench: &TestbenchSpec,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    output_dir: &Path,
    solve: bool,
    mode: MacroSmallSignalMode,
) -> Result<CircuitMnaAnalysis, CircuitMnaAnalysisError> {
    let small_signal_netlist = render_macro_testbench_small_signal_netlist_with_mode(
        testbench,
        primitive_catalog,
        macro_catalog,
        mode,
    )
    .map_err(CircuitMnaAnalysisError::MacroRender)?;

    analyze_small_signal_netlist_mna(&testbench.name, small_signal_netlist, output_dir, solve)
}

pub fn analyze_small_signal_netlist_mna(
    design_name: &str,
    small_signal_netlist: String,
    output_dir: &Path,
    solve: bool,
) -> Result<CircuitMnaAnalysis, CircuitMnaAnalysisError> {
    fs::create_dir_all(output_dir)?;

    let spice_path = output_dir.join(format!("{design_name}.spice"));
    let cir_path = output_dir.join(format!("{design_name}.cir"));
    fs::write(&spice_path, &small_signal_netlist)?;

    let mna = mna(output_dir, output_dir, design_name).map_err(CircuitMnaAnalysisError::Mna)?;
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

pub fn transfer_function_expression(
    mna: &MnaResult,
    solution: &MnaSolveResult,
    input_node: &str,
    output_node: &str,
) -> Result<String, TransferFunctionError> {
    let input_variable = mna.variable_for_node_name(input_node).ok_or_else(|| {
        TransferFunctionError::MissingInputNode {
            node: input_node.to_string(),
        }
    })?;
    let output_variable = mna.variable_for_node_name(output_node).ok_or_else(|| {
        TransferFunctionError::MissingOutputNode {
            node: output_node.to_string(),
        }
    })?;

    let input_expression = solution.solutions.get(&input_variable).ok_or_else(|| {
        TransferFunctionError::MissingInputSolution {
            variable: input_variable.clone(),
        }
    })?;
    let output_expression = solution.solutions.get(&output_variable).ok_or_else(|| {
        TransferFunctionError::MissingOutputSolution {
            variable: output_variable.clone(),
        }
    })?;

    Ok(format!("({output_expression})/({input_expression})"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::CircuitMnaOutput;
    use crate::catalog::load_primitive_catalog;
    use crate::circuit::{Connection, Instance, PinRef, load_circuit};
    use crate::exploration::TestbenchElement;
    use crate::macro_model::load_macro_catalog;
    use crate::mna::spice_parser::NodeMap;
    use std::collections::HashMap;

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
        assert_eq!(
            analysis.mna.variable_for_node_name("VOUT").as_deref(),
            Some("v1")
        );
        assert_eq!(analysis.mna.node_name_for_variable("v3"), Some("VINP"));
        assert_eq!(
            analysis
                .mna
                .node_variables()
                .into_iter()
                .map(|node| (node.variable, node.node_name))
                .collect::<Vec<_>>(),
            vec![
                ("v1".to_string(), "VOUT".to_string()),
                ("v2".to_string(), "IBIAS".to_string()),
                ("v3".to_string(), "VINP".to_string()),
                ("v4".to_string(), "N1".to_string()),
                ("v5".to_string(), "VINN".to_string()),
                ("v6".to_string(), "VDD".to_string()),
            ]
        );

        let output = CircuitMnaOutput::from_analysis(&analysis);
        let vout = output
            .nodes
            .iter()
            .find(|node| node.name == "VOUT")
            .expect("VOUT node should exist");
        assert_eq!(vout.number, 1);
        assert_eq!(output.variables[0].variable, "v1");
        assert_eq!(output.variables[0].node_name, "VOUT");
        assert_eq!(output.solution, None);
        assert!(
            output
                .equations
                .iter()
                .any(|equation| equation.text.contains("gm__xdp__m1"))
        );

        fs::remove_dir_all(output_dir).unwrap();
    }

    #[test]
    fn analyzes_macro_mna_without_solving() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let output_dir =
            std::env::temp_dir().join(format!("sstadex-macro-mna-test-{}", std::process::id()));

        let primitive_catalog =
            load_primitive_catalog(&workspace_root.join("analoglib/primitives")).unwrap();
        let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros")).unwrap();
        let ota = macro_catalog.get("ota_1stage").unwrap();

        let analysis =
            analyze_macro_mna(ota, &primitive_catalog, &macro_catalog, &output_dir, false).unwrap();

        assert!(analysis.spice_path.exists());
        assert!(analysis.cir_path.exists());
        assert!(analysis.solution.is_none());
        assert!(
            analysis
                .small_signal_netlist
                .contains("I_isource__xcs_macro IBIAS VSS isource__xcs_macro")
        );
        assert!(
            analysis
                .mna
                .z
                .iter()
                .any(|expr| expr.to_string().contains("isource__xcs_macro"))
        );

        let expanded_output_dir = std::env::temp_dir().join(format!(
            "sstadex-macro-mna-expand-test-{}",
            std::process::id()
        ));
        let expanded_analysis = analyze_macro_mna_with_mode(
            ota,
            &primitive_catalog,
            &macro_catalog,
            &expanded_output_dir,
            false,
            MacroSmallSignalMode::Expand,
        )
        .unwrap();
        assert!(
            expanded_analysis
                .small_signal_netlist
                .contains("G_gm__xcs_macro__xcs__m1 IBIAS VSS VBIAS VSS gm__xcs_macro__xcs__m1")
        );

        fs::remove_dir_all(output_dir).unwrap();
        fs::remove_dir_all(expanded_output_dir).unwrap();
    }

    #[test]
    fn analyzes_macro_testbench_mna_without_solving() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("libsstadex should be inside the workspace");
        let output_dir = std::env::temp_dir().join(format!(
            "sstadex-macro-testbench-mna-test-{}",
            std::process::id()
        ));

        let primitive_catalog =
            load_primitive_catalog(&workspace_root.join("analoglib/primitives")).unwrap();
        let macro_catalog = load_macro_catalog(&workspace_root.join("analoglib/macros")).unwrap();
        let testbench = TestbenchSpec::new("ota_gain")
            .with_macro_dut("ota_1stage")
            .with_element(TestbenchElement::VoltageSource {
                name: "Vdd".to_string(),
                nplus: "VDD".to_string(),
                nminus: "VSS".to_string(),
                value: "0".to_string(),
            })
            .with_element(TestbenchElement::VoltageSource {
                name: "Vin".to_string(),
                nplus: "VINP".to_string(),
                nminus: "VSS".to_string(),
                value: "1".to_string(),
            });

        let analysis = analyze_macro_testbench_mna(
            &testbench,
            &primitive_catalog,
            &macro_catalog,
            &output_dir,
            false,
        )
        .unwrap();

        assert!(analysis.spice_path.exists());
        assert!(analysis.cir_path.exists());
        assert!(analysis.solution.is_none());
        assert!(
            analysis
                .small_signal_netlist
                .contains("* testbench ota_gain")
        );
        assert!(analysis.small_signal_netlist.contains("Vin VINP VSS 1"));
        assert!(
            analysis
                .mna
                .z
                .iter()
                .any(|expr| expr.to_string().contains("isource__xcs_macro"))
        );

        let expanded_output_dir = std::env::temp_dir().join(format!(
            "sstadex-macro-testbench-mna-expand-test-{}",
            std::process::id()
        ));
        let expanded_analysis = analyze_macro_testbench_mna_with_mode(
            &testbench,
            &primitive_catalog,
            &macro_catalog,
            &expanded_output_dir,
            false,
            MacroSmallSignalMode::Expand,
        )
        .unwrap();
        assert!(
            expanded_analysis
                .small_signal_netlist
                .contains("G_gm__xcs_macro__xcs__m1 IBIAS VSS VBIAS VSS gm__xcs_macro__xcs__m1")
        );

        fs::remove_dir_all(output_dir).unwrap();
        fs::remove_dir_all(expanded_output_dir).unwrap();
    }

    #[test]
    fn analyzes_prerendered_small_signal_netlist_without_solving() {
        let output_dir = std::env::temp_dir().join(format!(
            "sstadex-prerendered-mna-test-{}",
            std::process::id()
        ));
        let netlist = [
            "* prerendered small-signal netlist",
            "Vin VIN vss 1",
            "R1 VOUT vss ro",
            "G1 VOUT vss VIN vss gm",
            "",
        ]
        .join("\n");

        let analysis =
            analyze_small_signal_netlist_mna("prerendered", netlist, &output_dir, false).unwrap();

        assert!(analysis.spice_path.exists());
        assert!(analysis.cir_path.exists());
        assert!(analysis.solution.is_none());
        assert_eq!(
            analysis.mna.variable_for_node_name("VIN").as_deref(),
            Some("v1")
        );
        assert_eq!(
            analysis.mna.variable_for_node_name("VOUT").as_deref(),
            Some("v2")
        );
        assert!(
            analysis
                .mna
                .a
                .iter()
                .flatten()
                .any(|expr| expr.to_string().contains("gm"))
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

    #[test]
    fn extracts_transfer_function_expression_from_mna_solution() {
        let mna = synthetic_mna_result();
        let solution = MnaSolveResult {
            solutions: HashMap::from([
                ("v1".to_string(), "gm*ro*vin".to_string()),
                ("v2".to_string(), "vin".to_string()),
            ]),
        };

        assert_eq!(
            transfer_function_expression(&mna, &solution, "VIN", "VOUT").unwrap(),
            "(gm*ro*vin)/(vin)"
        );
    }

    #[test]
    fn reports_missing_transfer_function_node() {
        let mna = synthetic_mna_result();
        let solution = MnaSolveResult {
            solutions: HashMap::new(),
        };

        assert_eq!(
            transfer_function_expression(&mna, &solution, "MISSING", "VOUT"),
            Err(TransferFunctionError::MissingInputNode {
                node: "MISSING".to_string(),
            })
        );
        assert_eq!(
            transfer_function_expression(&mna, &solution, "VIN", "MISSING"),
            Err(TransferFunctionError::MissingOutputNode {
                node: "MISSING".to_string(),
            })
        );
    }

    #[test]
    fn reports_missing_transfer_function_solution() {
        let mna = synthetic_mna_result();
        let solution = MnaSolveResult {
            solutions: HashMap::from([("v2".to_string(), "vin".to_string())]),
        };

        assert_eq!(
            transfer_function_expression(&mna, &solution, "VIN", "VOUT"),
            Err(TransferFunctionError::MissingOutputSolution {
                variable: "v1".to_string(),
            })
        );
    }

    fn synthetic_mna_result() -> MnaResult {
        MnaResult {
            report: String::new(),
            a: Vec::new(),
            x: Vec::new(),
            z: Vec::new(),
            nodes: NodeMap {
                nodes: HashMap::from([
                    ("0".to_string(), 0),
                    ("VOUT".to_string(), 1),
                    ("VIN".to_string(), 2),
                ]),
            },
        }
    }
}
