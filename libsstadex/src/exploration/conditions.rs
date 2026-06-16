use super::table::ExplorationColumn;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeCondition {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl RangeCondition {
    pub fn new(min: Option<f64>, max: Option<f64>) -> Self {
        Self { min, max }
    }

    pub fn min(value: f64) -> Self {
        Self {
            min: Some(value),
            max: None,
        }
    }

    pub fn max(value: f64) -> Self {
        Self {
            min: None,
            max: Some(value),
        }
    }

    pub fn contains_abs(self, value: f64) -> bool {
        let value = if value.is_finite() { value.abs() } else { 0.0 };

        if let Some(min) = self.min {
            if value <= min {
                return false;
            }
        }

        if let Some(max) = self.max {
            if value >= max {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterPhase {
    AxisPreEvaluation,
    CandidatePreEvaluation,
    PostEvaluation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplorationFilter {
    Range {
        phase: FilterPhase,
        column: String,
        condition: RangeCondition,
    },
    EqualColumns {
        phase: FilterPhase,
        columns: Vec<String>,
    },
}

impl ExplorationFilter {
    pub fn new(phase: FilterPhase, column: impl Into<String>, condition: RangeCondition) -> Self {
        Self::Range {
            phase,
            column: column.into(),
            condition,
        }
    }

    pub fn equal_columns(phase: FilterPhase, columns: Vec<impl Into<String>>) -> Self {
        Self::EqualColumns {
            phase,
            columns: columns.into_iter().map(Into::into).collect(),
        }
    }

    pub fn phase(&self) -> FilterPhase {
        match self {
            Self::Range { phase, .. } => *phase,
            Self::EqualColumns { phase, .. } => *phase,
        }
    }

    pub fn column(&self) -> Option<&str> {
        match self {
            Self::Range { column, .. } => Some(column),
            Self::EqualColumns { .. } => None,
        }
    }

    pub fn columns(&self) -> &[String] {
        match self {
            Self::Range { column, .. } => std::slice::from_ref(column),
            Self::EqualColumns { columns, .. } => columns,
        }
    }

    pub fn condition(&self) -> Option<RangeCondition> {
        match self {
            Self::Range { condition, .. } => Some(*condition),
            Self::EqualColumns { .. } => None,
        }
    }

    pub fn accepts_value(&self, value: f64) -> bool {
        match self {
            Self::Range { condition, .. } => condition.contains_abs(value),
            Self::EqualColumns { .. } => true,
        }
    }
}

pub fn shared_node_filter(columns: Vec<impl Into<String>>) -> ExplorationFilter {
    ExplorationFilter::equal_columns(FilterPhase::CandidatePreEvaluation, columns)
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecificationResult {
    pub name: String,
    pub values: Vec<f64>,
    pub condition: RangeCondition,
}

impl SpecificationResult {
    pub fn new(name: impl Into<String>, values: Vec<f64>, condition: RangeCondition) -> Self {
        Self {
            name: name.into(),
            values,
            condition,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterConditionsError {
    EmptySpecifications,
    LengthMismatch {
        specification: String,
        expected: usize,
        actual: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterKnownColumnsError {
    EmptyColumns,
    MissingColumn {
        column: String,
    },
    ColumnLengthMismatch {
        column: String,
        expected: usize,
        actual: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterEqualColumnsError {
    EmptyColumns,
    MissingColumn {
        column: String,
    },
    ColumnLengthMismatch {
        column: String,
        expected: usize,
        actual: usize,
    },
}

pub fn filter_conditions(
    specifications: &[SpecificationResult],
) -> Result<Vec<bool>, FilterConditionsError> {
    let Some(first) = specifications.first() else {
        return Err(FilterConditionsError::EmptySpecifications);
    };

    let expected = first.values.len();
    let mut mask = vec![true; expected];

    for specification in specifications {
        let actual = specification.values.len();
        if actual != expected {
            return Err(FilterConditionsError::LengthMismatch {
                specification: specification.name.clone(),
                expected,
                actual,
            });
        }

        for (is_kept, value) in mask.iter_mut().zip(&specification.values) {
            *is_kept &= specification.condition.contains_abs(*value);
        }
    }

    Ok(mask)
}

pub fn filter_known_columns(
    columns: &[ExplorationColumn],
    filters: &[ExplorationFilter],
) -> Result<Vec<bool>, FilterKnownColumnsError> {
    let Some(first) = columns.first() else {
        return Err(FilterKnownColumnsError::EmptyColumns);
    };

    let expected = first.values.len();
    let mut mask = vec![true; expected];

    for column in columns {
        let actual = column.values.len();
        if actual != expected {
            return Err(FilterKnownColumnsError::ColumnLengthMismatch {
                column: column.name.clone(),
                expected,
                actual,
            });
        }
    }

    for filter in filters
        .iter()
        .filter(|filter| filter.phase() == FilterPhase::AxisPreEvaluation)
        .filter(|filter| matches!(filter, ExplorationFilter::Range { .. }))
    {
        let column = columns
            .iter()
            .find(|column| Some(column.name.as_str()) == filter.column())
            .ok_or_else(|| FilterKnownColumnsError::MissingColumn {
                column: filter
                    .column()
                    .unwrap_or("<not-a-range-filter>")
                    .to_string(),
            })?;

        for (is_kept, value) in mask.iter_mut().zip(&column.values) {
            *is_kept &= filter.accepts_value(*value);
        }
    }

    Ok(mask)
}

pub fn filter_equal_columns(
    columns: &[ExplorationColumn],
    filters: &[ExplorationFilter],
    phase: FilterPhase,
) -> Result<Vec<bool>, FilterEqualColumnsError> {
    let Some(first) = columns.first() else {
        return Err(FilterEqualColumnsError::EmptyColumns);
    };

    let expected = first.values.len();
    let mut mask = vec![true; expected];

    for column in columns {
        let actual = column.values.len();
        if actual != expected {
            return Err(FilterEqualColumnsError::ColumnLengthMismatch {
                column: column.name.clone(),
                expected,
                actual,
            });
        }
    }

    for filter in filters
        .iter()
        .filter(|filter| filter.phase() == phase)
        .filter_map(|filter| match filter {
            ExplorationFilter::EqualColumns { columns, .. } => Some(columns),
            ExplorationFilter::Range { .. } => None,
        })
    {
        if filter.len() < 2 {
            continue;
        }

        let reference = column_by_name(columns, &filter[0])?;
        for column_name in &filter[1..] {
            let column = column_by_name(columns, column_name)?;

            for ((is_kept, reference_value), value) in
                mask.iter_mut().zip(&reference.values).zip(&column.values)
            {
                *is_kept &= reference_value == value;
            }
        }
    }

    Ok(mask)
}

fn column_by_name<'a>(
    columns: &'a [ExplorationColumn],
    name: &str,
) -> Result<&'a ExplorationColumn, FilterEqualColumnsError> {
    columns
        .iter()
        .find(|column| column.name == name)
        .ok_or_else(|| FilterEqualColumnsError::MissingColumn {
            column: name.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_values_with_max_condition() {
        let specs = vec![SpecificationResult::new(
            "gain",
            vec![1.0, 2.0, 3.0],
            RangeCondition::max(2.5),
        )];

        assert_eq!(filter_conditions(&specs).unwrap(), vec![true, true, false]);
    }

    #[test]
    fn filters_values_with_min_condition() {
        let specs = vec![SpecificationResult::new(
            "gbw",
            vec![1.0, 2.0, 3.0],
            RangeCondition::min(1.5),
        )];

        assert_eq!(filter_conditions(&specs).unwrap(), vec![false, true, true]);
    }

    #[test]
    fn combines_multiple_specification_masks() {
        let specs = vec![
            SpecificationResult::new("gain", vec![1.0, 3.0, 5.0], RangeCondition::min(2.0)),
            SpecificationResult::new("area", vec![9.0, 8.0, 7.0], RangeCondition::max(8.5)),
        ];

        assert_eq!(filter_conditions(&specs).unwrap(), vec![false, true, true]);
    }

    #[test]
    fn filters_absolute_values_for_python_compatibility() {
        let specs = vec![SpecificationResult::new(
            "gain",
            vec![-1.0, -3.0, 4.0],
            RangeCondition::min(2.0),
        )];

        assert_eq!(filter_conditions(&specs).unwrap(), vec![false, true, true]);
    }

    #[test]
    fn reports_length_mismatch() {
        let specs = vec![
            SpecificationResult::new("gain", vec![1.0, 2.0], RangeCondition::min(0.0)),
            SpecificationResult::new("area", vec![1.0], RangeCondition::max(2.0)),
        ];

        assert_eq!(
            filter_conditions(&specs),
            Err(FilterConditionsError::LengthMismatch {
                specification: "area".to_string(),
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn creates_exploration_filter_with_phase() {
        let filter = ExplorationFilter::new(
            FilterPhase::AxisPreEvaluation,
            "W_diff",
            RangeCondition::min(1e-6),
        );

        assert_eq!(filter.phase(), FilterPhase::AxisPreEvaluation);
        assert_eq!(filter.column(), Some("W_diff"));
        assert_eq!(filter.condition(), Some(RangeCondition::min(1e-6)));
    }

    #[test]
    fn exploration_filter_evaluates_range_condition() {
        let filter = ExplorationFilter::new(
            FilterPhase::PostEvaluation,
            "gain",
            RangeCondition::max(10.0),
        );

        assert!(filter.accepts_value(3.0));
        assert!(!filter.accepts_value(12.0));
    }

    #[test]
    fn creates_equal_columns_filter() {
        let filter = ExplorationFilter::equal_columns(
            FilterPhase::CandidatePreEvaluation,
            vec!["vs_diff", "vs_cs_1stage"],
        );

        assert_eq!(filter.phase(), FilterPhase::CandidatePreEvaluation);
        assert_eq!(filter.column(), None);
        assert_eq!(
            filter.columns(),
            &["vs_diff".to_string(), "vs_cs_1stage".to_string()]
        );
        assert_eq!(filter.condition(), None);
    }

    #[test]
    fn creates_shared_node_filter() {
        let filter = shared_node_filter(vec!["xdp.vs", "xcs.vs"]);

        assert_eq!(filter.phase(), FilterPhase::CandidatePreEvaluation);
        assert_eq!(filter.column(), None);
        assert_eq!(
            filter.columns(),
            &["xdp.vs".to_string(), "xcs.vs".to_string()]
        );
        assert_eq!(filter.condition(), None);
    }

    #[test]
    fn filters_known_columns_with_pre_evaluation_filters() {
        let columns = vec![
            ExplorationColumn::new("W_diff", vec![0.5, 2.0, 3.0]),
            ExplorationColumn::new("L_diff", vec![1.0, 1.0, 2.0]),
        ];
        let filters = vec![ExplorationFilter::new(
            FilterPhase::AxisPreEvaluation,
            "W_diff",
            RangeCondition::min(1.0),
        )];

        assert_eq!(
            filter_known_columns(&columns, &filters).unwrap(),
            vec![false, true, true]
        );
    }

    #[test]
    fn ignores_non_pre_evaluation_filters_for_known_columns() {
        let columns = vec![ExplorationColumn::new("gain", vec![1.0, 2.0, 3.0])];
        let filters = vec![ExplorationFilter::new(
            FilterPhase::PostEvaluation,
            "gain",
            RangeCondition::min(2.0),
        )];

        assert_eq!(
            filter_known_columns(&columns, &filters).unwrap(),
            vec![true, true, true]
        );
    }

    #[test]
    fn reports_missing_column_for_known_column_filter() {
        let columns = vec![ExplorationColumn::new("W_diff", vec![1.0])];
        let filters = vec![ExplorationFilter::new(
            FilterPhase::AxisPreEvaluation,
            "L_diff",
            RangeCondition::min(1.0),
        )];

        assert_eq!(
            filter_known_columns(&columns, &filters),
            Err(FilterKnownColumnsError::MissingColumn {
                column: "L_diff".to_string(),
            })
        );
    }

    #[test]
    fn filters_equal_columns_for_candidate_pre_evaluation() {
        let columns = vec![
            ExplorationColumn::new("vs_diff", vec![0.2, 0.3, 0.4]),
            ExplorationColumn::new("vs_cs", vec![0.1, 0.3, 0.4]),
        ];
        let filters = vec![ExplorationFilter::equal_columns(
            FilterPhase::CandidatePreEvaluation,
            vec!["vs_diff", "vs_cs"],
        )];

        assert_eq!(
            filter_equal_columns(&columns, &filters, FilterPhase::CandidatePreEvaluation).unwrap(),
            vec![false, true, true]
        );
    }

    #[test]
    fn filters_equal_columns_with_three_column_group() {
        let columns = vec![
            ExplorationColumn::new("a", vec![1.0, 1.0, 2.0]),
            ExplorationColumn::new("b", vec![1.0, 2.0, 2.0]),
            ExplorationColumn::new("c", vec![1.0, 1.0, 2.0]),
        ];
        let filters = vec![ExplorationFilter::equal_columns(
            FilterPhase::CandidatePreEvaluation,
            vec!["a", "b", "c"],
        )];

        assert_eq!(
            filter_equal_columns(&columns, &filters, FilterPhase::CandidatePreEvaluation).unwrap(),
            vec![true, false, true]
        );
    }

    #[test]
    fn ignores_equal_column_filters_from_other_phases() {
        let columns = vec![
            ExplorationColumn::new("a", vec![1.0, 2.0]),
            ExplorationColumn::new("b", vec![2.0, 2.0]),
        ];
        let filters = vec![ExplorationFilter::equal_columns(
            FilterPhase::PostEvaluation,
            vec!["a", "b"],
        )];

        assert_eq!(
            filter_equal_columns(&columns, &filters, FilterPhase::CandidatePreEvaluation).unwrap(),
            vec![true, true]
        );
    }

    #[test]
    fn reports_missing_column_for_equal_column_filter() {
        let columns = vec![ExplorationColumn::new("a", vec![1.0])];
        let filters = vec![ExplorationFilter::equal_columns(
            FilterPhase::CandidatePreEvaluation,
            vec!["a", "missing"],
        )];

        assert_eq!(
            filter_equal_columns(&columns, &filters, FilterPhase::CandidatePreEvaluation),
            Err(FilterEqualColumnsError::MissingColumn {
                column: "missing".to_string(),
            })
        );
    }
}
