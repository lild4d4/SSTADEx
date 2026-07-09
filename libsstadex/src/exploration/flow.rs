use super::candidate::{
    CandidateAxis, CandidatePipelineError, CandidatePoint, CandidateSet, build_filtered_candidates,
};
use super::conditions::{
    ExplorationFilter, FilterConditionsError, SpecificationResult, filter_conditions,
};
use super::evaluation::{
    CandidateEvaluationError, evaluate_prepared_candidate_specs, evaluate_prepared_expression_specs,
};
use super::prepare::{PreparedSpec, SpecPrepareError, prepare_candidate_expression_specs};
use super::spec::ExplorationSpec;
use super::table::{
    ExplorationColumn, ExplorationTable, ExplorationTableError, assemble_filtered_table,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplorationFlowError {
    CandidatePipeline(CandidatePipelineError),
    SpecPrepare(SpecPrepareError),
    Evaluation(CandidateEvaluationError),
    FilterConditions(FilterConditionsError),
    Table(ExplorationTableError),
}

impl From<CandidatePipelineError> for ExplorationFlowError {
    fn from(error: CandidatePipelineError) -> Self {
        Self::CandidatePipeline(error)
    }
}

impl From<SpecPrepareError> for ExplorationFlowError {
    fn from(error: SpecPrepareError) -> Self {
        Self::SpecPrepare(error)
    }
}

impl From<CandidateEvaluationError> for ExplorationFlowError {
    fn from(error: CandidateEvaluationError) -> Self {
        Self::Evaluation(error)
    }
}

impl From<FilterConditionsError> for ExplorationFlowError {
    fn from(error: FilterConditionsError) -> Self {
        Self::FilterConditions(error)
    }
}

impl From<ExplorationTableError> for ExplorationFlowError {
    fn from(error: ExplorationTableError) -> Self {
        Self::Table(error)
    }
}

pub fn run_candidate_expression_flow<F>(
    axes: &[CandidateAxis],
    sets: &[CandidateSet],
    filters: &[ExplorationFilter],
    specs: &[ExplorationSpec],
    evaluator: F,
) -> Result<ExplorationTable, ExplorationFlowError>
where
    F: Fn(&PreparedSpec, usize, &CandidatePoint) -> Result<f64, CandidateEvaluationError>,
{
    let candidates = build_filtered_candidates(axes, sets, filters)?;
    let prepared_specs = prepare_candidate_expression_specs(specs)?;
    let specification_results =
        evaluate_prepared_candidate_specs(&candidates, &prepared_specs, evaluator)?;
    let mask = filter_conditions(&specification_results)?;
    let candidate_columns = CandidatePoint::to_columns(&candidates);
    let specification_columns = specification_results_to_columns(specification_results);

    Ok(assemble_filtered_table(
        &candidate_columns,
        &specification_columns,
        &[],
        &mask,
    )?)
}

pub fn run_prepared_expression_flow(
    axes: &[CandidateAxis],
    sets: &[CandidateSet],
    filters: &[ExplorationFilter],
    prepared_specs: &[PreparedSpec],
) -> Result<ExplorationTable, ExplorationFlowError> {
    let candidates = build_filtered_candidates(axes, sets, filters)?;
    let specification_results = evaluate_prepared_expression_specs(&candidates, prepared_specs)?;
    let mask = filter_conditions(&specification_results)?;
    let candidate_columns = CandidatePoint::to_columns(&candidates);
    let specification_columns = specification_results_to_columns(specification_results);

    Ok(assemble_filtered_table(
        &candidate_columns,
        &specification_columns,
        &[],
        &mask,
    )?)
}

fn specification_results_to_columns(
    specification_results: Vec<SpecificationResult>,
) -> Vec<ExplorationColumn> {
    specification_results
        .into_iter()
        .map(|result| ExplorationColumn::new(result.name, result.values))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exploration::{
        CandidateAxis, CandidatePoint, CandidateSet, RangeCondition, SpecOutput, SpecPrepareError,
        SpecSource, TestbenchSpec, required_candidate_value, shared_node_filter,
    };

    #[test]
    fn runs_candidate_expression_flow_to_filtered_table() {
        let axes = vec![CandidateAxis::new("Ra", vec![1.0, 2.0])];
        let diffpair = CandidateSet::new(
            "diffpair",
            vec![
                CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), 1.0),
                    ("xdp.gds".to_string(), 0.2),
                    ("xdp.vs".to_string(), 0.3),
                ]),
                CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), 10.0),
                    ("xdp.gds".to_string(), 0.2),
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
                    ("vdd".to_string(), 1.8),
                    ("ibias".to_string(), 1e-4),
                ]),
                CandidatePoint::new(vec![
                    ("xcs.gds".to_string(), 0.05),
                    ("xcs.vs".to_string(), 0.4),
                    ("vdd".to_string(), 1.8),
                    ("ibias".to_string(), 5e-4),
                ]),
            ],
        );
        let filters = vec![shared_node_filter(vec!["xdp.vs", "xcs.vs"])];
        let specs = vec![
            ExplorationSpec::new(
                "gain",
                RangeCondition::min(4.0),
                SpecSource::CandidateExpression {
                    expression: "xdp.gm / (xdp.gds + xcs.gds)".to_string(),
                },
                SpecOutput::Eval,
            ),
            ExplorationSpec::new(
                "power",
                RangeCondition::max(5e-4),
                SpecSource::CandidateExpression {
                    expression: "vdd * ibias".to_string(),
                },
                SpecOutput::Eval,
            ),
        ];

        let table = run_candidate_expression_flow(
            &axes,
            &[diffpair, current_source],
            &filters,
            &specs,
            |prepared, idx, candidate| match prepared.name.as_str() {
                "gain" => {
                    let gm = required_candidate_value(candidate, idx, "xdp.gm")?;
                    let xdp_gds = required_candidate_value(candidate, idx, "xdp.gds")?;
                    let xcs_gds = required_candidate_value(candidate, idx, "xcs.gds")?;
                    Ok(gm / (xdp_gds + xcs_gds))
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

        assert_eq!(table.row_count, 2);
        assert_eq!(table.column("Ra").unwrap().values, vec![1.0, 2.0]);
        assert_eq!(table.column("xdp.gm").unwrap().values, vec![1.0, 1.0]);
        assert_eq!(table.column("xdp.vs").unwrap().values, vec![0.3, 0.3]);
        assert_eq!(table.column("xcs.vs").unwrap().values, vec![0.3, 0.3]);
        assert_eq!(
            table.column("gain").unwrap().values,
            vec![1.0 / (0.2 + 0.02), 1.0 / (0.2 + 0.02)]
        );
        assert_eq!(table.column("power").unwrap().values, vec![1.8e-4, 1.8e-4]);
    }

    #[test]
    fn reports_unsupported_transfer_function_spec() {
        let axes = vec![CandidateAxis::new("Ra", vec![1.0])];
        let specs = vec![ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::TransferFunction {
                testbench: TestbenchSpec::new("gain"),
                input: "VIN".to_string(),
                output: "VOUT".to_string(),
            },
            SpecOutput::Eval,
        )];

        assert_eq!(
            run_candidate_expression_flow(
                &axes,
                &[],
                &[],
                &specs,
                |_prepared, _idx, _candidate| {
                    unreachable!("transfer function specs should fail before evaluation")
                }
            ),
            Err(ExplorationFlowError::SpecPrepare(
                SpecPrepareError::UnsupportedSource {
                    source: "transfer_function",
                },
            ))
        );
    }

    #[test]
    fn runs_prepared_transfer_function_expression_flow() {
        let axes = vec![CandidateAxis::new("Ra", vec![1.0, 2.0])];
        let primitive = CandidateSet::new(
            "xdp",
            vec![
                CandidatePoint::new(vec![
                    ("gm__xdp__m1".to_string(), 10.0),
                    ("ro__xdp__m1".to_string(), 100.0),
                    ("vin".to_string(), 1.0),
                ]),
                CandidatePoint::new(vec![
                    ("gm__xdp__m1".to_string(), 20.0),
                    ("ro__xdp__m1".to_string(), 100.0),
                    ("vin".to_string(), 1.0),
                ]),
            ],
        );
        let prepared = vec![PreparedSpec::new(
            "gain",
            RangeCondition::min(1500.0),
            SpecOutput::Eval,
            crate::exploration::PreparedSpecSource::TransferFunction {
                expression: "(gm__xdp__m1*ro__xdp__m1*vin)/(vin)".to_string(),
            },
        )];

        let table = run_prepared_expression_flow(&axes, &[primitive], &[], &prepared).unwrap();

        assert_eq!(table.row_count, 2);
        assert_eq!(table.column("Ra").unwrap().values, vec![1.0, 2.0]);
        assert_eq!(
            table.column("gm__xdp__m1").unwrap().values,
            vec![20.0, 20.0],
        );
        assert_eq!(table.column("gain").unwrap().values, vec![2000.0, 2000.0]);
    }
}
