use super::conditions::RangeCondition;
use super::spec::{ExplorationSpec, SpecOutput, SpecSource};

#[derive(Debug, Clone, PartialEq)]
pub struct PreparedSpec {
    pub name: String,
    pub condition: RangeCondition,
    pub output: SpecOutput,
    pub source: PreparedSpecSource,
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
        }
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
        )),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exploration::{
        RangeCondition, SpecOutput, SpecSource, TestbenchSpec,
    };

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
}
