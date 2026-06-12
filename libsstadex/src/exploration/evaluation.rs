use super::candidate::CandidatePoint;
use super::conditions::{RangeCondition, SpecificationResult};
use super::prepare::PreparedSpec;
use super::table::ExplorationColumn;

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateEvaluation {
    pub name: String,
    pub values: Vec<f64>,
}

impl CandidateEvaluation {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            values,
        }
    }

    pub fn with_condition(self, condition: RangeCondition) -> SpecificationResult {
        SpecificationResult::new(self.name, self.values, condition)
    }

    pub fn into_column(self) -> ExplorationColumn {
        ExplorationColumn::new(self.name, self.values)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidateEvaluationError {
    MissingValue { candidate: usize, column: String },
}

pub fn required_candidate_value(
    candidate: &CandidatePoint,
    candidate_index: usize,
    column: &str,
) -> Result<f64, CandidateEvaluationError> {
    candidate
        .get(column)
        .ok_or_else(|| CandidateEvaluationError::MissingValue {
            candidate: candidate_index,
            column: column.to_string(),
        })
}

pub fn evaluate_candidates<F>(
    candidates: &[CandidatePoint],
    name: impl Into<String>,
    evaluator: F,
) -> Result<CandidateEvaluation, CandidateEvaluationError>
where
    F: Fn(usize, &CandidatePoint) -> Result<f64, CandidateEvaluationError>,
{
    let mut values = Vec::with_capacity(candidates.len());

    for (idx, candidate) in candidates.iter().enumerate() {
        values.push(evaluator(idx, candidate)?);
    }

    Ok(CandidateEvaluation::new(name, values))
}

pub fn evaluate_prepared_candidate_spec<F>(
    candidates: &[CandidatePoint],
    prepared: &PreparedSpec,
    evaluator: F,
) -> Result<SpecificationResult, CandidateEvaluationError>
where
    F: Fn(usize, &CandidatePoint) -> Result<f64, CandidateEvaluationError>,
{
    Ok(evaluate_candidates(candidates, prepared.name.clone(), evaluator)?
        .with_condition(prepared.condition))
}

pub fn evaluate_prepared_candidate_specs<F>(
    candidates: &[CandidatePoint],
    prepared_specs: &[PreparedSpec],
    evaluator: F,
) -> Result<Vec<SpecificationResult>, CandidateEvaluationError>
where
    F: Fn(&PreparedSpec, usize, &CandidatePoint) -> Result<f64, CandidateEvaluationError>,
{
    let mut results = Vec::with_capacity(prepared_specs.len());

    for prepared in prepared_specs {
        results.push(evaluate_prepared_candidate_spec(candidates, prepared, |idx, candidate| {
            evaluator(prepared, idx, candidate)
        })?);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exploration::{
        CandidateAxis, CandidatePoint, CandidateSet, assemble_filtered_table,
        build_filtered_candidates, filter_conditions, prepare_candidate_expression_spec,
        shared_node_filter, SpecOutput, SpecSource,
    };

    #[test]
    fn evaluates_candidates_with_custom_function() {
        let candidates = vec![
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 10.0),
                ("xcm.ro".to_string(), 100.0),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 20.0),
                ("xcm.ro".to_string(), 200.0),
            ]),
        ];

        let gain = evaluate_candidates(&candidates, "gain", |idx, candidate| {
            let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
            let ro = required_candidate_value(candidate, idx, "xcm.ro")?;
            Ok(gm * ro)
        })
        .unwrap();

        assert_eq!(gain, CandidateEvaluation::new("gain", vec![1000.0, 4000.0]));
    }

    #[test]
    fn reports_missing_candidate_value() {
        let candidates = vec![CandidatePoint::new(vec![("xdp.gm".to_string(), 10.0)])];

        assert_eq!(
            evaluate_candidates(&candidates, "gain", |idx, candidate| {
                let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
                let ro = required_candidate_value(candidate, idx, "xcm.ro")?;
                Ok(gm * ro)
            }),
            Err(CandidateEvaluationError::MissingValue {
                candidate: 0,
                column: "xcm.ro".to_string(),
            })
        );
    }

    #[test]
    fn converts_candidate_evaluation_to_specification_result() {
        let evaluation = CandidateEvaluation::new("gain", vec![10.0, 50.0, 100.0]);

        assert_eq!(
            evaluation.with_condition(RangeCondition::min(40.0)),
            SpecificationResult::new("gain", vec![10.0, 50.0, 100.0], RangeCondition::min(40.0))
        );
    }

    #[test]
    fn converts_candidate_evaluation_to_column() {
        let evaluation = CandidateEvaluation::new("gain", vec![10.0, 50.0, 100.0]);

        assert_eq!(
            evaluation.into_column(),
            ExplorationColumn::new("gain", vec![10.0, 50.0, 100.0])
        );
    }

    #[test]
    fn filters_candidate_evaluation_with_post_evaluation_condition() {
        let candidates = vec![
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 1.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 5.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 10.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
        ];
        let gain = evaluate_candidates(&candidates, "gain", |idx, candidate| {
            let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
            let ro = required_candidate_value(candidate, idx, "xcm.ro")?;
            Ok(gm * ro)
        })
        .unwrap();
        let gain_spec = gain.with_condition(RangeCondition::min(40.0));

        assert_eq!(filter_conditions(&[gain_spec]).unwrap(), vec![false, true, true]);
    }

    #[test]
    fn evaluates_prepared_candidate_spec() {
        let candidates = vec![
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 5.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 10.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
        ];
        let spec = crate::exploration::ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::CandidateExpression {
                expression: "xdp.gm * xcm.ro".to_string(),
            },
            SpecOutput::Eval,
        );
        let prepared = prepare_candidate_expression_spec(&spec).unwrap();

        let result = evaluate_prepared_candidate_spec(&candidates, &prepared, |idx, candidate| {
            let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
            let ro = required_candidate_value(candidate, idx, "xcm.ro")?;
            Ok(gm * ro)
        })
        .unwrap();

        assert_eq!(
            result,
            SpecificationResult::new("gain", vec![50.0, 100.0], RangeCondition::min(40.0))
        );
    }

    #[test]
    fn reports_missing_value_when_evaluating_prepared_candidate_spec() {
        let candidates = vec![CandidatePoint::new(vec![("xdp.gm".to_string(), 5.0)])];
        let spec = crate::exploration::ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::CandidateExpression {
                expression: "xdp.gm * xcm.ro".to_string(),
            },
            SpecOutput::Eval,
        );
        let prepared = prepare_candidate_expression_spec(&spec).unwrap();

        assert_eq!(
            evaluate_prepared_candidate_spec(&candidates, &prepared, |idx, candidate| {
                let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
                let ro = required_candidate_value(candidate, idx, "xcm.ro")?;
                Ok(gm * ro)
            }),
            Err(CandidateEvaluationError::MissingValue {
                candidate: 0,
                column: "xcm.ro".to_string(),
            })
        );
    }

    #[test]
    fn filters_evaluated_prepared_candidate_spec() {
        let candidates = vec![
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 1.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 5.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 10.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
        ];
        let spec = crate::exploration::ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::CandidateExpression {
                expression: "xdp.gm * xcm.ro".to_string(),
            },
            SpecOutput::Eval,
        );
        let prepared = prepare_candidate_expression_spec(&spec).unwrap();
        let result = evaluate_prepared_candidate_spec(&candidates, &prepared, |idx, candidate| {
            let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
            let ro = required_candidate_value(candidate, idx, "xcm.ro")?;
            Ok(gm * ro)
        })
        .unwrap();

        assert_eq!(filter_conditions(&[result]).unwrap(), vec![false, true, true]);
    }

    #[test]
    fn evaluates_prepared_candidate_specs_in_order() {
        let candidates = vec![
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 5.0),
                ("xcm.ro".to_string(), 10.0),
                ("vdd".to_string(), 1.8),
                ("ibias".to_string(), 1e-4),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 10.0),
                ("xcm.ro".to_string(), 10.0),
                ("vdd".to_string(), 1.8),
                ("ibias".to_string(), 2e-4),
            ]),
        ];
        let specs = vec![
            crate::exploration::ExplorationSpec::new(
                "gain",
                RangeCondition::min(40.0),
                SpecSource::CandidateExpression {
                    expression: "xdp.gm * xcm.ro".to_string(),
                },
                SpecOutput::Eval,
            ),
            crate::exploration::ExplorationSpec::new(
                "power",
                RangeCondition::max(5e-4),
                SpecSource::CandidateExpression {
                    expression: "vdd * ibias".to_string(),
                },
                SpecOutput::Eval,
            ),
        ];
        let prepared = crate::exploration::prepare_candidate_expression_specs(&specs).unwrap();

        let results = evaluate_prepared_candidate_specs(
            &candidates,
            &prepared,
            |prepared, idx, candidate| match prepared.name.as_str() {
                "gain" => {
                    let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
                    let ro = required_candidate_value(candidate, idx, "xcm.ro")?;
                    Ok(gm * ro)
                }
                "power" => {
                    let vdd = required_candidate_value(candidate, idx, "vdd")?;
                    let ibias = required_candidate_value(candidate, idx, "ibias")?;
                    Ok(vdd * ibias)
                }
                _ => unreachable!("unexpected prepared spec"),
            },
        )
        .unwrap();

        assert_eq!(
            results,
            vec![
                SpecificationResult::new("gain", vec![50.0, 100.0], RangeCondition::min(40.0)),
                SpecificationResult::new("power", vec![1.8e-4, 3.6e-4], RangeCondition::max(5e-4)),
            ]
        );
    }

    #[test]
    fn filters_multiple_evaluated_prepared_candidate_specs() {
        let candidates = vec![
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 1.0),
                ("xcm.ro".to_string(), 10.0),
                ("vdd".to_string(), 1.8),
                ("ibias".to_string(), 1e-4),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 5.0),
                ("xcm.ro".to_string(), 10.0),
                ("vdd".to_string(), 1.8),
                ("ibias".to_string(), 1e-4),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 10.0),
                ("xcm.ro".to_string(), 10.0),
                ("vdd".to_string(), 1.8),
                ("ibias".to_string(), 5e-4),
            ]),
        ];
        let specs = vec![
            crate::exploration::ExplorationSpec::new(
                "gain",
                RangeCondition::min(40.0),
                SpecSource::CandidateExpression {
                    expression: "xdp.gm * xcm.ro".to_string(),
                },
                SpecOutput::Eval,
            ),
            crate::exploration::ExplorationSpec::new(
                "power",
                RangeCondition::max(5e-4),
                SpecSource::CandidateExpression {
                    expression: "vdd * ibias".to_string(),
                },
                SpecOutput::Eval,
            ),
        ];
        let prepared = crate::exploration::prepare_candidate_expression_specs(&specs).unwrap();
        let results = evaluate_prepared_candidate_specs(
            &candidates,
            &prepared,
            |prepared, idx, candidate| match prepared.name.as_str() {
                "gain" => {
                    let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
                    let ro = required_candidate_value(candidate, idx, "xcm.ro")?;
                    Ok(gm * ro)
                }
                "power" => {
                    let vdd = required_candidate_value(candidate, idx, "vdd")?;
                    let ibias = required_candidate_value(candidate, idx, "ibias")?;
                    Ok(vdd * ibias)
                }
                _ => unreachable!("unexpected prepared spec"),
            },
        )
        .unwrap();

        assert_eq!(filter_conditions(&results).unwrap(), vec![false, true, false]);
    }

    #[test]
    fn assembles_filtered_table_from_candidates_and_evaluation() {
        let candidates = vec![
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 1.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 5.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
            CandidatePoint::new(vec![
                ("xdp.gm".to_string(), 10.0),
                ("xcm.ro".to_string(), 10.0),
            ]),
        ];
        let gain = evaluate_candidates(&candidates, "gain", |idx, candidate| {
            let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
            let ro = required_candidate_value(candidate, idx, "xcm.ro")?;
            Ok(gm * ro)
        })
        .unwrap();
        let mask = filter_conditions(&[gain.clone().with_condition(RangeCondition::min(40.0))])
            .unwrap();
        let candidate_columns = CandidatePoint::to_columns(&candidates);
        let gain_column = gain.into_column();

        let table =
            assemble_filtered_table(&candidate_columns, &[gain_column], &[], &mask).unwrap();

        assert_eq!(table.row_count, 2);
        assert_eq!(table.column("xdp.gm").unwrap().values, vec![5.0, 10.0]);
        assert_eq!(table.column("xcm.ro").unwrap().values, vec![10.0, 10.0]);
        assert_eq!(table.column("gain").unwrap().values, vec![50.0, 100.0]);
    }

    #[test]
    fn evaluates_candidates_after_shared_node_filtering() {
        let axes = vec![CandidateAxis::new("Ra", vec![1.0, 2.0])];
        let diffpair = CandidateSet::new(
            "diffpair",
            vec![
                CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), 10.0),
                    ("xdp.gds".to_string(), 0.2),
                    ("xdp.vs".to_string(), 0.3),
                ]),
                CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), 100.0),
                    ("xdp.gds".to_string(), 0.4),
                    ("xdp.vs".to_string(), 0.4),
                ]),
            ],
        );
        let current_source = CandidateSet::new(
            "current_source",
            vec![
                CandidatePoint::new(vec![
                    ("xcs.gds".to_string(), 0.02),
                    ("xcs.vs".to_string(), 0.3),
                ]),
                CandidatePoint::new(vec![
                    ("xcs.gds".to_string(), 0.05),
                    ("xcs.vs".to_string(), 0.4),
                ]),
            ],
        );
        let filters = vec![shared_node_filter(vec!["xdp.vs", "xcs.vs"])];
        let candidates =
            build_filtered_candidates(&axes, &[diffpair, current_source], &filters).unwrap();

        let gain = evaluate_candidates(&candidates, "gain", |idx, candidate| {
            let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
            let xdp_gds = required_candidate_value(candidate, idx, "xdp.gds")?;
            let xcs_gds = required_candidate_value(candidate, idx, "xcs.gds")?;
            Ok(gm / (xdp_gds + xcs_gds))
        })
        .unwrap();

        assert_eq!(candidates.len(), 4);
        assert_eq!(
            gain,
            CandidateEvaluation::new(
                "gain",
                vec![
                    10.0 / (0.2 + 0.02),
                    100.0 / (0.4 + 0.05),
                    10.0 / (0.2 + 0.02),
                    100.0 / (0.4 + 0.05),
                ],
            )
        );
    }
}
