use super::conditions::{
    ExplorationFilter, FilterEqualColumnsError, FilterPhase, filter_equal_columns,
};
use super::table::ExplorationColumn;

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateAxis {
    pub name: String,
    pub values: Vec<f64>,
}

impl CandidateAxis {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            values,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidatePoint {
    pub values: Vec<(String, f64)>,
}

impl CandidatePoint {
    pub fn new(values: Vec<(String, f64)>) -> Self {
        Self { values }
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.values
            .iter()
            .find_map(|(key, value)| (key == name).then_some(*value))
    }

    pub fn to_columns(points: &[CandidatePoint]) -> Vec<ExplorationColumn> {
        let Some(first) = points.first() else {
            return Vec::new();
        };

        first
            .values
            .iter()
            .map(|(name, _)| ExplorationColumn {
                name: name.clone(),
                values: points
                    .iter()
                    .filter_map(|point| point.get(name))
                    .collect::<Vec<_>>(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateSet {
    pub name: String,
    pub points: Vec<CandidatePoint>,
}

impl CandidateSet {
    pub fn new(name: impl Into<String>, points: Vec<CandidatePoint>) -> Self {
        Self {
            name: name.into(),
            points,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidateGenerationError {
    EmptyAxis { axis: String },
    EmptySet { set: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidateSetBuildError {
    EmptyColumns,
    ColumnLengthMismatch {
        column: String,
        expected: usize,
        actual: usize,
    },
}

pub fn candidate_set_from_columns(
    name: impl Into<String>,
    columns: &[ExplorationColumn],
) -> Result<CandidateSet, CandidateSetBuildError> {
    let Some(first) = columns.first() else {
        return Err(CandidateSetBuildError::EmptyColumns);
    };

    let row_count = first.values.len();

    for column in columns {
        let actual = column.values.len();
        if actual != row_count {
            return Err(CandidateSetBuildError::ColumnLengthMismatch {
                column: column.name.clone(),
                expected: row_count,
                actual,
            });
        }
    }

    let mut points = Vec::with_capacity(row_count);
    for row_idx in 0..row_count {
        let values = columns
            .iter()
            .map(|column| (column.name.clone(), column.values[row_idx]))
            .collect();
        points.push(CandidatePoint::new(values));
    }

    Ok(CandidateSet::new(name, points))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AxisFilterError {
    EmptyAxis { axis: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidateFilterError {
    EqualColumns(FilterEqualColumnsError),
    MissingColumn { column: String },
}

impl From<FilterEqualColumnsError> for CandidateFilterError {
    fn from(error: FilterEqualColumnsError) -> Self {
        Self::EqualColumns(error)
    }
}

pub fn filter_candidate_axes(
    axes: &[CandidateAxis],
    filters: &[ExplorationFilter],
) -> Result<Vec<CandidateAxis>, AxisFilterError> {
    let mut filtered_axes = Vec::with_capacity(axes.len());

    for axis in axes {
        let mut values = axis.values.clone();

        for filter in filters.iter().filter(|filter| {
            filter.phase() == FilterPhase::AxisPreEvaluation
                && filter.column() == Some(axis.name.as_str())
        }) {
            values.retain(|value| filter.accepts_value(*value));
        }

        if values.is_empty() {
            return Err(AxisFilterError::EmptyAxis {
                axis: axis.name.clone(),
            });
        }

        filtered_axes.push(CandidateAxis {
            name: axis.name.clone(),
            values,
        });
    }

    Ok(filtered_axes)
}

pub fn filter_candidate_points(
    candidates: &[CandidatePoint],
    filters: &[ExplorationFilter],
) -> Result<Vec<CandidatePoint>, CandidateFilterError> {
    if candidates.is_empty() {
        return Ok(Vec::new());
    }

    let columns = CandidatePoint::to_columns(candidates);
    let range_filters = filters
        .iter()
        .filter_map(|filter| match filter {
            ExplorationFilter::Range {
                phase,
                column,
                condition,
            } if *phase == FilterPhase::CandidatePreEvaluation => {
                Some(ExplorationFilter::new(*phase, column.clone(), *condition))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut mask = vec![true; candidates.len()];

    if !range_filters.is_empty() {
        let range_mask = filter_candidate_range_columns(&columns, &range_filters)?;
        for (is_kept, range_is_kept) in mask.iter_mut().zip(range_mask) {
            *is_kept &= range_is_kept;
        }
    }

    let equality_mask =
        filter_equal_columns(&columns, filters, FilterPhase::CandidatePreEvaluation)?;
    for (is_kept, equality_is_kept) in mask.iter_mut().zip(equality_mask) {
        *is_kept &= equality_is_kept;
    }

    Ok(candidates
        .iter()
        .zip(mask)
        .filter_map(|(candidate, is_kept)| is_kept.then_some(candidate.clone()))
        .collect())
}

fn filter_candidate_range_columns(
    columns: &[ExplorationColumn],
    filters: &[ExplorationFilter],
) -> Result<Vec<bool>, CandidateFilterError> {
    let row_count = columns.first().map(|column| column.values.len()).unwrap_or(0);
    let mut mask = vec![true; row_count];

    for filter in filters {
        let Some(column_name) = filter.column() else {
            continue;
        };
        let column = columns
            .iter()
            .find(|column| column.name == column_name)
            .ok_or_else(|| CandidateFilterError::MissingColumn {
                column: column_name.to_string(),
            })?;

        for (is_kept, value) in mask.iter_mut().zip(&column.values) {
            *is_kept &= filter.accepts_value(*value);
        }
    }

    Ok(mask)
}

pub fn generate_candidate_grid(
    axes: &[CandidateAxis],
) -> Result<Vec<CandidatePoint>, CandidateGenerationError> {
    let mut candidates = vec![CandidatePoint::new(Vec::new())];

    for axis in axes {
        if axis.values.is_empty() {
            return Err(CandidateGenerationError::EmptyAxis {
                axis: axis.name.clone(),
            });
        }

        let mut next_candidates = Vec::with_capacity(candidates.len() * axis.values.len());

        for candidate in &candidates {
            for value in &axis.values {
                let mut values = candidate.values.clone();
                values.push((axis.name.clone(), *value));
                next_candidates.push(CandidatePoint::new(values));
            }
        }

        candidates = next_candidates;
    }

    Ok(candidates)
}

pub fn generate_candidate_combinations(
    sets: &[CandidateSet],
) -> Result<Vec<CandidatePoint>, CandidateGenerationError> {
    let mut candidates = vec![CandidatePoint::new(Vec::new())];

    for set in sets {
        if set.points.is_empty() {
            return Err(CandidateGenerationError::EmptySet {
                set: set.name.clone(),
            });
        }

        let mut next_candidates = Vec::with_capacity(candidates.len() * set.points.len());

        for candidate in &candidates {
            for point in &set.points {
                let mut values = candidate.values.clone();
                values.extend(point.values.clone());
                next_candidates.push(CandidatePoint::new(values));
            }
        }

        candidates = next_candidates;
    }

    Ok(candidates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_candidate_axis() {
        let axis = CandidateAxis::new("W_diff", vec![1.0, 2.0]);

        assert_eq!(axis.name, "W_diff");
        assert_eq!(axis.values, vec![1.0, 2.0]);
    }

    #[test]
    fn gets_candidate_point_value_by_name() {
        let point = CandidatePoint::new(vec![
            ("W_diff".to_string(), 1.0),
            ("L_diff".to_string(), 2.0),
        ]);

        assert_eq!(point.get("W_diff"), Some(1.0));
        assert_eq!(point.get("missing"), None);
    }

    #[test]
    fn converts_candidate_points_to_columns() {
        let points = vec![
            CandidatePoint::new(vec![("W".to_string(), 1.0), ("L".to_string(), 3.0)]),
            CandidatePoint::new(vec![("W".to_string(), 2.0), ("L".to_string(), 4.0)]),
        ];

        let columns = CandidatePoint::to_columns(&points);

        assert_eq!(
            columns,
            vec![
                ExplorationColumn::new("W", vec![1.0, 2.0]),
                ExplorationColumn::new("L", vec![3.0, 4.0]),
            ]
        );
    }

    #[test]
    fn converts_empty_candidate_points_to_empty_columns() {
        assert!(CandidatePoint::to_columns(&[]).is_empty());
    }

    #[test]
    fn generates_candidate_grid_from_axes() {
        let axes = vec![
            CandidateAxis::new("W", vec![1.0, 2.0]),
            CandidateAxis::new("L", vec![3.0, 4.0]),
        ];

        let candidates = generate_candidate_grid(&axes).unwrap();

        assert_eq!(
            candidates,
            vec![
                CandidatePoint::new(vec![("W".to_string(), 1.0), ("L".to_string(), 3.0)]),
                CandidatePoint::new(vec![("W".to_string(), 1.0), ("L".to_string(), 4.0)]),
                CandidatePoint::new(vec![("W".to_string(), 2.0), ("L".to_string(), 3.0)]),
                CandidatePoint::new(vec![("W".to_string(), 2.0), ("L".to_string(), 4.0)]),
            ]
        );
    }

    #[test]
    fn generates_single_empty_candidate_for_no_axes() {
        assert_eq!(
            generate_candidate_grid(&[]).unwrap(),
            vec![CandidatePoint::new(Vec::new())]
        );
    }

    #[test]
    fn reports_empty_axis() {
        let axes = vec![CandidateAxis::new("W", Vec::new())];

        assert_eq!(
            generate_candidate_grid(&axes),
            Err(CandidateGenerationError::EmptyAxis {
                axis: "W".to_string(),
            })
        );
    }

    #[test]
    fn creates_candidate_set() {
        let set = CandidateSet::new(
            "xdp",
            vec![CandidatePoint::new(vec![("gm".to_string(), 1.0)])],
        );

        assert_eq!(set.name, "xdp");
        assert_eq!(set.points.len(), 1);
    }

    #[test]
    fn generates_candidate_combinations_from_sets() {
        let xdp = CandidateSet::new(
            "xdp",
            vec![
                CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), 1.0),
                    ("xdp.ro".to_string(), 10.0),
                ]),
                CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), 2.0),
                    ("xdp.ro".to_string(), 20.0),
                ]),
            ],
        );
        let xcm = CandidateSet::new(
            "xcm",
            vec![CandidatePoint::new(vec![("xcm.gm".to_string(), 3.0)])],
        );

        let candidates = generate_candidate_combinations(&[xdp, xcm]).unwrap();

        assert_eq!(
            candidates,
            vec![
                CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), 1.0),
                    ("xdp.ro".to_string(), 10.0),
                    ("xcm.gm".to_string(), 3.0),
                ]),
                CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), 2.0),
                    ("xdp.ro".to_string(), 20.0),
                    ("xcm.gm".to_string(), 3.0),
                ]),
            ]
        );
    }

    #[test]
    fn preserves_values_within_each_candidate_set_point() {
        let primitive = CandidateSet::new(
            "primitive",
            vec![
                CandidatePoint::new(vec![("gm".to_string(), 1.0), ("ro".to_string(), 10.0)]),
                CandidatePoint::new(vec![("gm".to_string(), 2.0), ("ro".to_string(), 20.0)]),
            ],
        );

        let candidates = generate_candidate_combinations(&[primitive]).unwrap();

        assert_eq!(candidates[0].get("gm"), Some(1.0));
        assert_eq!(candidates[0].get("ro"), Some(10.0));
        assert_eq!(candidates[1].get("gm"), Some(2.0));
        assert_eq!(candidates[1].get("ro"), Some(20.0));
    }

    #[test]
    fn reports_empty_candidate_set() {
        let sets = vec![CandidateSet::new("xdp", Vec::new())];

        assert_eq!(
            generate_candidate_combinations(&sets),
            Err(CandidateGenerationError::EmptySet {
                set: "xdp".to_string(),
            })
        );
    }

    #[test]
    fn builds_candidate_set_from_columns() {
        let columns = vec![
            ExplorationColumn::new("gm", vec![1.0, 2.0]),
            ExplorationColumn::new("ro", vec![10.0, 20.0]),
        ];

        let set = candidate_set_from_columns("xdp", &columns).unwrap();

        assert_eq!(
            set,
            CandidateSet::new(
                "xdp",
                vec![
                    CandidatePoint::new(vec![("gm".to_string(), 1.0), ("ro".to_string(), 10.0)]),
                    CandidatePoint::new(vec![("gm".to_string(), 2.0), ("ro".to_string(), 20.0)]),
                ],
            )
        );
    }

    #[test]
    fn reports_empty_columns_when_building_candidate_set() {
        assert_eq!(
            candidate_set_from_columns("xdp", &[]),
            Err(CandidateSetBuildError::EmptyColumns)
        );
    }

    #[test]
    fn reports_column_length_mismatch_when_building_candidate_set() {
        let columns = vec![
            ExplorationColumn::new("gm", vec![1.0, 2.0]),
            ExplorationColumn::new("ro", vec![10.0]),
        ];

        assert_eq!(
            candidate_set_from_columns("xdp", &columns),
            Err(CandidateSetBuildError::ColumnLengthMismatch {
                column: "ro".to_string(),
                expected: 2,
                actual: 1,
            })
        );
    }

    #[test]
    fn filters_candidate_axes_before_grid_generation() {
        let axes = vec![
            CandidateAxis::new("W", vec![0.5, 1.0, 2.0]),
            CandidateAxis::new("L", vec![1.0, 2.0]),
        ];
        let filters = vec![ExplorationFilter::new(
            FilterPhase::AxisPreEvaluation,
            "W",
            super::super::conditions::RangeCondition::min(0.75),
        )];

        assert_eq!(
            filter_candidate_axes(&axes, &filters).unwrap(),
            vec![
                CandidateAxis::new("W", vec![1.0, 2.0]),
                CandidateAxis::new("L", vec![1.0, 2.0]),
            ]
        );
    }

    #[test]
    fn ignores_non_axis_filters_when_filtering_candidate_axes() {
        let axes = vec![CandidateAxis::new("W", vec![0.5, 1.0, 2.0])];
        let filters = vec![ExplorationFilter::new(
            FilterPhase::CandidatePreEvaluation,
            "W",
            super::super::conditions::RangeCondition::min(0.75),
        )];

        assert_eq!(filter_candidate_axes(&axes, &filters).unwrap(), axes);
    }

    #[test]
    fn reports_axis_filter_that_removes_all_values() {
        let axes = vec![CandidateAxis::new("W", vec![0.5])];
        let filters = vec![ExplorationFilter::new(
            FilterPhase::AxisPreEvaluation,
            "W",
            super::super::conditions::RangeCondition::min(0.75),
        )];

        assert_eq!(
            filter_candidate_axes(&axes, &filters),
            Err(AxisFilterError::EmptyAxis {
                axis: "W".to_string(),
            })
        );
    }

    #[test]
    fn filters_candidate_points_with_candidate_range_filter() {
        let candidates = vec![
            CandidatePoint::new(vec![("W".to_string(), 0.5)]),
            CandidatePoint::new(vec![("W".to_string(), 1.0)]),
            CandidatePoint::new(vec![("W".to_string(), 2.0)]),
        ];
        let filters = vec![ExplorationFilter::new(
            FilterPhase::CandidatePreEvaluation,
            "W",
            super::super::conditions::RangeCondition::min(0.75),
        )];

        assert_eq!(
            filter_candidate_points(&candidates, &filters).unwrap(),
            vec![
                CandidatePoint::new(vec![("W".to_string(), 1.0)]),
                CandidatePoint::new(vec![("W".to_string(), 2.0)]),
            ]
        );
    }

    #[test]
    fn filters_candidate_points_with_equal_columns_filter() {
        let candidates = vec![
            CandidatePoint::new(vec![("vs_diff".to_string(), 0.2), ("vs_cs".to_string(), 0.1)]),
            CandidatePoint::new(vec![("vs_diff".to_string(), 0.3), ("vs_cs".to_string(), 0.3)]),
            CandidatePoint::new(vec![("vs_diff".to_string(), 0.4), ("vs_cs".to_string(), 0.4)]),
        ];
        let filters = vec![ExplorationFilter::equal_columns(
            FilterPhase::CandidatePreEvaluation,
            vec!["vs_diff", "vs_cs"],
        )];

        assert_eq!(
            filter_candidate_points(&candidates, &filters).unwrap(),
            vec![
                CandidatePoint::new(vec![
                    ("vs_diff".to_string(), 0.3),
                    ("vs_cs".to_string(), 0.3),
                ]),
                CandidatePoint::new(vec![
                    ("vs_diff".to_string(), 0.4),
                    ("vs_cs".to_string(), 0.4),
                ]),
            ]
        );
    }

    #[test]
    fn combines_candidate_range_and_equal_columns_filters() {
        let candidates = vec![
            CandidatePoint::new(vec![("W".to_string(), 0.5), ("a".to_string(), 1.0), ("b".to_string(), 1.0)]),
            CandidatePoint::new(vec![("W".to_string(), 2.0), ("a".to_string(), 1.0), ("b".to_string(), 2.0)]),
            CandidatePoint::new(vec![("W".to_string(), 2.0), ("a".to_string(), 3.0), ("b".to_string(), 3.0)]),
        ];
        let filters = vec![
            ExplorationFilter::new(
                FilterPhase::CandidatePreEvaluation,
                "W",
                super::super::conditions::RangeCondition::min(1.0),
            ),
            ExplorationFilter::equal_columns(FilterPhase::CandidatePreEvaluation, vec!["a", "b"]),
        ];

        assert_eq!(
            filter_candidate_points(&candidates, &filters).unwrap(),
            vec![CandidatePoint::new(vec![
                ("W".to_string(), 2.0),
                ("a".to_string(), 3.0),
                ("b".to_string(), 3.0),
            ])]
        );
    }
}
