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
}
