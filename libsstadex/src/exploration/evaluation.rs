use super::candidate::CandidatePoint;
use super::conditions::{RangeCondition, SpecificationResult};
use super::prepare::{PreparedSpec, PreparedSpecSource};
use super::spec::SpecParameter;
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
    MissingValue {
        candidate: usize,
        column: String,
    },
    UnsupportedPreparedSource {
        source: &'static str,
    },
    Expression {
        candidate: usize,
        expression: String,
        reason: String,
    },
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
    Ok(
        evaluate_candidates(candidates, prepared.name.clone(), evaluator)?
            .with_condition(prepared.condition),
    )
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
        results.push(evaluate_prepared_candidate_spec(
            candidates,
            prepared,
            |idx, candidate| evaluator(prepared, idx, candidate),
        )?);
    }

    Ok(results)
}

pub fn evaluate_prepared_expression_spec(
    candidates: &[CandidatePoint],
    prepared: &PreparedSpec,
) -> Result<SpecificationResult, CandidateEvaluationError> {
    let expression = prepared_expression(prepared)?;

    evaluate_prepared_candidate_spec(candidates, prepared, |idx, candidate| {
        evaluate_candidate_expression_with_parameters(
            candidate,
            idx,
            expression,
            &prepared.parameter_map,
        )
    })
}

pub fn evaluate_prepared_expression_specs(
    candidates: &[CandidatePoint],
    prepared_specs: &[PreparedSpec],
) -> Result<Vec<SpecificationResult>, CandidateEvaluationError> {
    let mut results = Vec::with_capacity(prepared_specs.len());

    for prepared in prepared_specs {
        results.push(evaluate_prepared_expression_spec(candidates, prepared)?);
    }

    Ok(results)
}

pub fn evaluate_candidate_expression(
    candidate: &CandidatePoint,
    candidate_index: usize,
    expression: &str,
) -> Result<f64, CandidateEvaluationError> {
    evaluate_candidate_expression_with_parameters(candidate, candidate_index, expression, &[])
}

pub fn evaluate_candidate_expression_with_parameters(
    candidate: &CandidatePoint,
    candidate_index: usize,
    expression: &str,
    parameters: &[SpecParameter],
) -> Result<f64, CandidateEvaluationError> {
    ExpressionParser::new(expression, candidate, candidate_index, parameters)
        .parse()
        .map_err(|reason| CandidateEvaluationError::Expression {
            candidate: candidate_index,
            expression: expression.to_string(),
            reason,
        })
}

fn prepared_expression(prepared: &PreparedSpec) -> Result<&str, CandidateEvaluationError> {
    match &prepared.source {
        PreparedSpecSource::CandidateExpression { expression }
        | PreparedSpecSource::TransferFunction { expression } => Ok(expression),
        PreparedSpecSource::Composed => {
            Err(CandidateEvaluationError::UnsupportedPreparedSource { source: "composed" })
        }
    }
}

struct ExpressionParser<'a> {
    expression: &'a str,
    bytes: &'a [u8],
    pos: usize,
    candidate: &'a CandidatePoint,
    candidate_index: usize,
    parameters: &'a [SpecParameter],
    resolving: Vec<String>,
}

impl<'a> ExpressionParser<'a> {
    fn new(
        expression: &'a str,
        candidate: &'a CandidatePoint,
        candidate_index: usize,
        parameters: &'a [SpecParameter],
    ) -> Self {
        Self {
            expression,
            bytes: expression.as_bytes(),
            pos: 0,
            candidate,
            candidate_index,
            parameters,
            resolving: Vec::new(),
        }
    }

    fn parse(mut self) -> Result<f64, String> {
        let value = self.parse_expression()?;
        self.skip_whitespace();

        if self.pos != self.bytes.len() {
            return Err(format!("unexpected token at byte {}", self.pos));
        }

        Ok(value)
    }

    fn parse_expression(&mut self) -> Result<f64, String> {
        let mut value = self.parse_term()?;

        loop {
            self.skip_whitespace();
            if self.consume(b'+') {
                value += self.parse_term()?;
            } else if self.consume(b'-') {
                value -= self.parse_term()?;
            } else {
                break;
            }
        }

        Ok(value)
    }

    fn parse_term(&mut self) -> Result<f64, String> {
        let mut value = self.parse_factor()?;

        loop {
            self.skip_whitespace();
            if self.consume(b'*') {
                value *= self.parse_factor()?;
            } else if self.consume(b'/') {
                value /= self.parse_factor()?;
            } else {
                break;
            }
        }

        Ok(value)
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        self.skip_whitespace();

        if self.consume(b'-') {
            return Ok(-self.parse_factor()?);
        }

        if self.consume(b'+') {
            return self.parse_factor();
        }

        if self.consume(b'(') {
            let value = self.parse_expression()?;
            self.skip_whitespace();
            if !self.consume(b')') {
                return Err("missing closing ')'".to_string());
            }
            return Ok(value);
        }

        if self.peek().is_some_and(is_number_start) {
            return self.parse_number();
        }

        if self.peek().is_some_and(is_identifier_start) {
            return self.parse_symbol();
        }

        Err(format!("expected factor at byte {}", self.pos))
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.pos;

        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            self.pos += 1;
        }

        if self.consume(b'.') {
            while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                self.pos += 1;
            }
        }

        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;

            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }

            let exponent_start = self.pos;
            while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                self.pos += 1;
            }

            if self.pos == exponent_start {
                return Err("missing exponent digits".to_string());
            }
        }

        self.expression[start..self.pos]
            .parse::<f64>()
            .map_err(|error| format!("invalid number: {error}"))
    }

    fn parse_symbol(&mut self) -> Result<f64, String> {
        let start = self.pos;

        while self.peek().is_some_and(is_identifier_body) {
            self.pos += 1;
        }

        let symbol = &self.expression[start..self.pos];
        self.resolve_symbol(symbol)
    }

    fn resolve_symbol(&self, symbol: &str) -> Result<f64, String> {
        if let Some(value) = self.candidate.get(symbol) {
            return Ok(value);
        }

        if self.resolving.iter().any(|name| name == symbol) {
            return Err(format!("cyclic parameter map while resolving '{symbol}'"));
        }

        let Some(parameter) = self
            .parameters
            .iter()
            .find(|parameter| parameter.name == symbol)
        else {
            return required_candidate_value(self.candidate, self.candidate_index, symbol)
                .map_err(|error| format!("{error:?}"));
        };

        let mut resolving = self.resolving.clone();
        resolving.push(symbol.to_string());

        ExpressionParser {
            expression: &parameter.value,
            bytes: parameter.value.as_bytes(),
            pos: 0,
            candidate: self.candidate,
            candidate_index: self.candidate_index,
            parameters: self.parameters,
            resolving,
        }
        .parse()
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_whitespace()) {
            self.pos += 1;
        }
    }

    fn consume(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }
}

fn is_number_start(byte: u8) -> bool {
    byte.is_ascii_digit() || byte == b'.'
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_body(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exploration::{
        CandidateAxis, CandidatePoint, CandidateSet, SpecOutput, SpecParameter, SpecSource,
        assemble_filtered_table, build_filtered_candidates, filter_conditions,
        prepare_candidate_expression_spec, shared_node_filter,
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

        assert_eq!(
            filter_conditions(&[gain_spec]).unwrap(),
            vec![false, true, true]
        );
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

        assert_eq!(
            filter_conditions(&[result]).unwrap(),
            vec![false, true, true]
        );
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

        assert_eq!(
            filter_conditions(&results).unwrap(),
            vec![false, true, false]
        );
    }

    #[test]
    fn evaluates_candidate_expression_string() {
        let candidate = CandidatePoint::new(vec![
            ("xdp.gm".to_string(), 10.0),
            ("xdp.gds".to_string(), 0.2),
            ("xcs.gds".to_string(), 0.05),
        ]);

        assert_eq!(
            evaluate_candidate_expression(&candidate, 0, "xdp.gm / (xdp.gds + xcs.gds) + 1e-3",)
                .unwrap(),
            10.0 / (0.2 + 0.05) + 1e-3,
        );
    }

    #[test]
    fn evaluates_candidate_expression_with_parameter_map() {
        let candidate = CandidatePoint::new(vec![
            ("gm__xdp__m1".to_string(), 10.0),
            ("ro__xdp__m1".to_string(), 100.0),
        ]);
        let parameters = vec![
            SpecParameter::new("gm__xdp__m2", "gm__xdp__m1"),
            SpecParameter::new("ro__xdp__m2", "ro__xdp__m1"),
            SpecParameter::new("vin", "1"),
        ];

        assert_eq!(
            evaluate_candidate_expression_with_parameters(
                &candidate,
                0,
                "(gm__xdp__m2 * ro__xdp__m2) / vin",
                &parameters,
            )
            .unwrap(),
            1000.0,
        );
    }

    #[test]
    fn reports_cyclic_parameter_map() {
        let candidate = CandidatePoint::new(Vec::new());
        let parameters = vec![
            SpecParameter::new("a", "b + 1"),
            SpecParameter::new("b", "a + 1"),
        ];

        assert_eq!(
            evaluate_candidate_expression_with_parameters(&candidate, 0, "a", &parameters),
            Err(CandidateEvaluationError::Expression {
                candidate: 0,
                expression: "a".to_string(),
                reason: "cyclic parameter map while resolving 'a'".to_string(),
            })
        );
    }

    #[test]
    fn evaluates_prepared_expression_specs_without_external_evaluator() {
        let candidates = vec![
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
        ];
        let prepared = vec![crate::exploration::PreparedSpec::new(
            "gain",
            RangeCondition::min(1500.0),
            SpecOutput::Eval,
            crate::exploration::PreparedSpecSource::TransferFunction {
                expression: "(gm__xdp__m1*ro__xdp__m1*vin)/(vin)".to_string(),
            },
        )];

        let results = evaluate_prepared_expression_specs(&candidates, &prepared).unwrap();

        assert_eq!(
            results,
            vec![SpecificationResult::new(
                "gain",
                vec![1000.0, 2000.0],
                RangeCondition::min(1500.0),
            )]
        );
        assert_eq!(filter_conditions(&results).unwrap(), vec![false, true]);
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
        let mask =
            filter_conditions(&[gain.clone().with_condition(RangeCondition::min(40.0))]).unwrap();
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
