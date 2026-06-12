use super::candidate::CandidatePoint;
use super::conditions::{RangeCondition, SpecificationResult};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exploration::{
        CandidateAxis, CandidatePoint, CandidateSet, assemble_filtered_table,
        build_filtered_candidates, filter_conditions, shared_node_filter,
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
