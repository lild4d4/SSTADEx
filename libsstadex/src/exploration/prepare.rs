use std::fmt::Debug;
use std::path::Path;

use crate::analysis::{
    analyze_macro_testbench_mna_with_mode, analyze_small_signal_netlist_mna,
    transfer_function_expression, TransferFunctionError,
};
use crate::catalog::PrimitiveCatalog;
use crate::circuit::Circuit;
use crate::macro_model::{MacroCatalog, MacroSmallSignalMode};
use crate::mna::mna::{MnaResult, MnaSolveResult};
use crate::netlist::render_testbench_small_signal_netlist;

use super::conditions::RangeCondition;
use super::spec::{ExplorationSpec, SpecOutput, SpecParameter, SpecSource};

#[derive(Debug, Clone, PartialEq)]
pub struct PreparedSpec {
    pub name: String,
    pub condition: RangeCondition,
    pub output: SpecOutput,
    pub source: PreparedSpecSource,
    pub parameter_map: Vec<SpecParameter>,
}

impl PreparedSpec {
    pub fn new(
        name: impl Into<String>,
        condition: RangeCondition,
        output: SpecOutput,
        source: PreparedSpecSource,
    ) -> Self {
        Self {
            name: name.into(),
            condition,
            output,
            source,
            parameter_map: Vec::new(),
        }
    }

    pub fn with_parameter_map(mut self, parameter_map: Vec<SpecParameter>) -> Self {
        self.parameter_map = parameter_map;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedSpecSource {
    CandidateExpression { expression: String },
    TransferFunction { expression: String },
    Composed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecPrepareError {
    UnsupportedSource { source: &'static str },
    TransferFunction(TransferFunctionError),
    MnaAnalysis { reason: String },
    MissingMnaSolution,
}

impl From<TransferFunctionError> for SpecPrepareError {
    fn from(error: TransferFunctionError) -> Self {
        Self::TransferFunction(error)
    }
}

pub fn prepare_candidate_expression_spec(
    spec: &ExplorationSpec,
) -> Result<PreparedSpec, SpecPrepareError> {
    match &spec.source {
        SpecSource::CandidateExpression { expression } => Ok(PreparedSpec::new(
            spec.name.clone(),
            spec.condition,
            spec.output.clone(),
            PreparedSpecSource::CandidateExpression {
                expression: expression.clone(),
            },
        )
        .with_parameter_map(spec.parameter_map.clone())),
        SpecSource::TransferFunction { .. } => Err(SpecPrepareError::UnsupportedSource {
            source: "transfer_function",
        }),
        SpecSource::Composed => Err(SpecPrepareError::UnsupportedSource { source: "composed" }),
    }
}

pub fn prepare_candidate_expression_specs(
    specs: &[ExplorationSpec],
) -> Result<Vec<PreparedSpec>, SpecPrepareError> {
    specs
        .iter()
        .map(prepare_candidate_expression_spec)
        .collect()
}

pub fn prepare_macro_testbench_specs(
    specs: &[ExplorationSpec],
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    output_dir: &Path,
) -> Result<Vec<PreparedSpec>, SpecPrepareError> {
    prepare_macro_testbench_specs_with_mode(
        specs,
        primitive_catalog,
        macro_catalog,
        output_dir,
        MacroSmallSignalMode::CompactWhenAvailable,
    )
}

pub fn prepare_macro_testbench_specs_with_mode(
    specs: &[ExplorationSpec],
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    output_dir: &Path,
    mode: MacroSmallSignalMode,
) -> Result<Vec<PreparedSpec>, SpecPrepareError> {
    specs
        .iter()
        .map(|spec| match &spec.source {
            SpecSource::CandidateExpression { .. } => prepare_candidate_expression_spec(spec),
            SpecSource::TransferFunction { .. } => {
                prepare_transfer_function_spec_for_macro_testbench_with_mode(
                    spec,
                    primitive_catalog,
                    macro_catalog,
                    output_dir,
                    mode,
                )
            }
            SpecSource::Composed => Err(SpecPrepareError::UnsupportedSource { source: "composed" }),
        })
        .collect()
}

pub fn prepare_transfer_function_spec_from_analysis(
    spec: &ExplorationSpec,
    mna: &MnaResult,
    solution: &MnaSolveResult,
) -> Result<PreparedSpec, SpecPrepareError> {
    match &spec.source {
        SpecSource::TransferFunction { input, output, .. } => Ok(PreparedSpec::new(
            spec.name.clone(),
            spec.condition,
            spec.output.clone(),
            PreparedSpecSource::TransferFunction {
                expression: transfer_function_expression(mna, solution, input, output)?,
            },
        )
        .with_parameter_map(spec.parameter_map.clone())),
        SpecSource::CandidateExpression { .. } => Err(SpecPrepareError::UnsupportedSource {
            source: "candidate_expression",
        }),
        SpecSource::Composed => Err(SpecPrepareError::UnsupportedSource { source: "composed" }),
    }
}

pub fn prepare_transfer_function_spec(
    spec: &ExplorationSpec,
    circuit: &Circuit,
    catalog: &PrimitiveCatalog,
    output_dir: &Path,
) -> Result<PreparedSpec, SpecPrepareError> {
    let testbench = match &spec.source {
        SpecSource::TransferFunction { testbench, .. } => testbench,
        SpecSource::CandidateExpression { .. } => {
            return Err(SpecPrepareError::UnsupportedSource {
                source: "candidate_expression",
            });
        }
        SpecSource::Composed => {
            return Err(SpecPrepareError::UnsupportedSource { source: "composed" });
        }
    };

    let small_signal_netlist = render_testbench_small_signal_netlist(circuit, catalog, testbench)
        .map_err(mna_analysis_error)?;
    let analysis =
        analyze_small_signal_netlist_mna(&testbench.name, small_signal_netlist, output_dir, true)
            .map_err(mna_analysis_error)?;
    let solution = analysis
        .solution
        .as_ref()
        .ok_or(SpecPrepareError::MissingMnaSolution)?;

    prepare_transfer_function_spec_from_analysis(spec, &analysis.mna, solution)
}

pub fn prepare_transfer_function_spec_for_macro_testbench(
    spec: &ExplorationSpec,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    output_dir: &Path,
) -> Result<PreparedSpec, SpecPrepareError> {
    prepare_transfer_function_spec_for_macro_testbench_with_mode(
        spec,
        primitive_catalog,
        macro_catalog,
        output_dir,
        MacroSmallSignalMode::CompactWhenAvailable,
    )
}

pub fn prepare_transfer_function_spec_for_macro_testbench_with_mode(
    spec: &ExplorationSpec,
    primitive_catalog: &PrimitiveCatalog,
    macro_catalog: &MacroCatalog,
    output_dir: &Path,
    mode: MacroSmallSignalMode,
) -> Result<PreparedSpec, SpecPrepareError> {
    let testbench = match &spec.source {
        SpecSource::TransferFunction { testbench, .. } => testbench,
        SpecSource::CandidateExpression { .. } => {
            return Err(SpecPrepareError::UnsupportedSource {
                source: "candidate_expression",
            });
        }
        SpecSource::Composed => {
            return Err(SpecPrepareError::UnsupportedSource { source: "composed" });
        }
    };

    let analysis = analyze_macro_testbench_mna_with_mode(
        testbench,
        primitive_catalog,
        macro_catalog,
        output_dir,
        true,
        mode,
    )
    .map_err(mna_analysis_error)?;
    let solution = analysis
        .solution
        .as_ref()
        .ok_or(SpecPrepareError::MissingMnaSolution)?;

    prepare_transfer_function_spec_from_analysis(spec, &analysis.mna, solution)
}

fn mna_analysis_error(error: impl Debug) -> SpecPrepareError {
    SpecPrepareError::MnaAnalysis {
        reason: format!("{error:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exploration::{RangeCondition, SpecOutput, SpecSource, TestbenchSpec};
    use crate::mna::spice_parser::NodeMap;
    use std::collections::HashMap;

    #[test]
    fn prepares_candidate_expression_spec() {
        let spec = ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::CandidateExpression {
                expression: "xdp.gm / (xdp.gds + xcs.gds)".to_string(),
            },
            SpecOutput::Eval,
        );

        assert_eq!(
            prepare_candidate_expression_spec(&spec).unwrap(),
            PreparedSpec::new(
                "gain",
                RangeCondition::min(40.0),
                SpecOutput::Eval,
                PreparedSpecSource::CandidateExpression {
                    expression: "xdp.gm / (xdp.gds + xcs.gds)".to_string(),
                },
            )
        );
    }

    #[test]
    fn reports_transfer_function_as_unsupported_for_candidate_expression_prepare() {
        let spec = ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::TransferFunction {
                testbench: TestbenchSpec::new("gain"),
                input: "VIN".to_string(),
                output: "VOUT".to_string(),
            },
            SpecOutput::Eval,
        );

        assert_eq!(
            prepare_candidate_expression_spec(&spec),
            Err(SpecPrepareError::UnsupportedSource {
                source: "transfer_function",
            })
        );
    }

    #[test]
    fn reports_composed_as_unsupported_for_candidate_expression_prepare() {
        let spec = ExplorationSpec::new(
            "efficiency",
            RangeCondition::min(1.0),
            SpecSource::Composed,
            SpecOutput::Divide {
                numerator: "gain".to_string(),
                denominator: "power".to_string(),
            },
        );

        assert_eq!(
            prepare_candidate_expression_spec(&spec),
            Err(SpecPrepareError::UnsupportedSource { source: "composed" })
        );
    }

    #[test]
    fn prepares_candidate_expression_specs_in_order() {
        let specs = vec![
            ExplorationSpec::new(
                "gain",
                RangeCondition::min(40.0),
                SpecSource::CandidateExpression {
                    expression: "xdp.gm / (xdp.gds + xcs.gds)".to_string(),
                },
                SpecOutput::Eval,
            ),
            ExplorationSpec::new(
                "power",
                RangeCondition::max(1e-3),
                SpecSource::CandidateExpression {
                    expression: "vdd * ibias".to_string(),
                },
                SpecOutput::Eval,
            ),
        ];

        let prepared = prepare_candidate_expression_specs(&specs).unwrap();

        assert_eq!(prepared.len(), 2);
        assert_eq!(prepared[0].name, "gain");
        assert_eq!(prepared[1].name, "power");
        assert_eq!(
            prepared[0].source,
            PreparedSpecSource::CandidateExpression {
                expression: "xdp.gm / (xdp.gds + xcs.gds)".to_string(),
            }
        );
        assert_eq!(
            prepared[1].source,
            PreparedSpecSource::CandidateExpression {
                expression: "vdd * ibias".to_string(),
            }
        );
    }

    #[test]
    fn stops_batch_prepare_on_unsupported_spec() {
        let specs = vec![
            ExplorationSpec::new(
                "gain",
                RangeCondition::min(40.0),
                SpecSource::CandidateExpression {
                    expression: "xdp.gm / (xdp.gds + xcs.gds)".to_string(),
                },
                SpecOutput::Eval,
            ),
            ExplorationSpec::new(
                "tf_gain",
                RangeCondition::min(40.0),
                SpecSource::TransferFunction {
                    testbench: TestbenchSpec::new("gain"),
                    input: "VIN".to_string(),
                    output: "VOUT".to_string(),
                },
                SpecOutput::Eval,
            ),
        ];

        assert_eq!(
            prepare_candidate_expression_specs(&specs),
            Err(SpecPrepareError::UnsupportedSource {
                source: "transfer_function",
            })
        );
    }

    #[test]
    fn prepares_transfer_function_spec_from_analysis() {
        let spec = ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::TransferFunction {
                testbench: TestbenchSpec::new("gain"),
                input: "VIN".to_string(),
                output: "VOUT".to_string(),
            },
            SpecOutput::Eval,
        );
        let mna = synthetic_mna_result();
        let solution = MnaSolveResult {
            solutions: HashMap::from([
                ("v1".to_string(), "gm*ro*vin".to_string()),
                ("v2".to_string(), "vin".to_string()),
            ]),
        };

        assert_eq!(
            prepare_transfer_function_spec_from_analysis(&spec, &mna, &solution).unwrap(),
            PreparedSpec::new(
                "gain",
                RangeCondition::min(40.0),
                SpecOutput::Eval,
                PreparedSpecSource::TransferFunction {
                    expression: "(gm*ro*vin)/(vin)".to_string(),
                },
            )
        );
    }

    #[test]
    fn reports_unsupported_source_when_preparing_transfer_function_spec() {
        let spec = ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::CandidateExpression {
                expression: "xdp.gm * xcm.ro".to_string(),
            },
            SpecOutput::Eval,
        );

        assert_eq!(
            prepare_transfer_function_spec_from_analysis(
                &spec,
                &synthetic_mna_result(),
                &MnaSolveResult {
                    solutions: HashMap::new(),
                },
            ),
            Err(SpecPrepareError::UnsupportedSource {
                source: "candidate_expression",
            })
        );
    }

    #[test]
    fn propagates_transfer_function_prepare_errors() {
        let spec = ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::TransferFunction {
                testbench: TestbenchSpec::new("gain"),
                input: "MISSING".to_string(),
                output: "VOUT".to_string(),
            },
            SpecOutput::Eval,
        );

        assert_eq!(
            prepare_transfer_function_spec_from_analysis(
                &spec,
                &synthetic_mna_result(),
                &MnaSolveResult {
                    solutions: HashMap::new(),
                },
            ),
            Err(SpecPrepareError::TransferFunction(
                TransferFunctionError::MissingInputNode {
                    node: "MISSING".to_string(),
                },
            ))
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
