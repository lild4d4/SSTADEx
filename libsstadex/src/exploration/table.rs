#[derive(Debug, Clone, PartialEq)]
pub struct ExplorationColumn {
    pub name: String,
    pub values: Vec<f64>,
}

impl ExplorationColumn {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            values,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExplorationTable {
    pub columns: Vec<ExplorationColumn>,
    pub row_count: usize,
}

impl ExplorationTable {
    pub fn column(&self, name: &str) -> Option<&ExplorationColumn> {
        self.columns.iter().find(|column| column.name == name)
    }

    pub fn add_sum_column(
        &mut self,
        name: impl Into<String>,
        source_columns: &[&str],
    ) -> Result<(), ExplorationTableError> {
        let name = name.into();
        let mut values = vec![0.0; self.row_count];

        for source_column in source_columns {
            let column =
                self.column(source_column)
                    .ok_or_else(|| ExplorationTableError::MissingColumn {
                        column: (*source_column).to_string(),
                    })?;

            if column.values.len() != self.row_count {
                return Err(ExplorationTableError::ColumnLengthMismatch {
                    column: column.name.clone(),
                    expected: self.row_count,
                    actual: column.values.len(),
                });
            }

            for (total, value) in values.iter_mut().zip(&column.values) {
                *total += value;
            }
        }

        self.columns.push(ExplorationColumn { name, values });
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplorationTableError {
    MissingColumn {
        column: String,
    },
    MaskLengthMismatch {
        column: String,
        mask_len: usize,
        values_len: usize,
    },
    ColumnLengthMismatch {
        column: String,
        expected: usize,
        actual: usize,
    },
}

pub fn assemble_filtered_table(
    parameter_axes: &[ExplorationColumn],
    specification_results: &[ExplorationColumn],
    output_results: &[ExplorationColumn],
    mask: &[bool],
) -> Result<ExplorationTable, ExplorationTableError> {
    let kept_rows = mask.iter().filter(|is_kept| **is_kept).count();
    let mut columns = Vec::new();

    for column in parameter_axes
        .iter()
        .chain(specification_results)
        .chain(output_results)
    {
        columns.push(filter_column(column, mask, kept_rows)?);
    }

    Ok(ExplorationTable {
        columns,
        row_count: kept_rows,
    })
}

fn filter_column(
    column: &ExplorationColumn,
    mask: &[bool],
    expected_filtered_len: usize,
) -> Result<ExplorationColumn, ExplorationTableError> {
    if column.values.len() != mask.len() {
        return Err(ExplorationTableError::MaskLengthMismatch {
            column: column.name.clone(),
            mask_len: mask.len(),
            values_len: column.values.len(),
        });
    }

    let values = column
        .values
        .iter()
        .zip(mask)
        .filter_map(|(value, is_kept)| is_kept.then_some(*value))
        .collect::<Vec<_>>();

    if values.len() != expected_filtered_len {
        return Err(ExplorationTableError::ColumnLengthMismatch {
            column: column.name.clone(),
            expected: expected_filtered_len,
            actual: values.len(),
        });
    }

    Ok(ExplorationColumn {
        name: column.name.clone(),
        values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembles_filtered_table_from_columns() {
        let parameter_axes = vec![
            ExplorationColumn::new("w", vec![1.0, 2.0, 3.0]),
            ExplorationColumn::new("l", vec![4.0, 5.0, 6.0]),
        ];
        let specification_results = vec![ExplorationColumn::new("gain", vec![10.0, 20.0, 30.0])];
        let output_results = vec![ExplorationColumn::new("gm", vec![0.1, 0.2, 0.3])];
        let mask = vec![true, false, true];

        let table = assemble_filtered_table(
            &parameter_axes,
            &specification_results,
            &output_results,
            &mask,
        )
        .unwrap();

        assert_eq!(table.row_count, 2);
        assert_eq!(table.column("w").unwrap().values, vec![1.0, 3.0]);
        assert_eq!(table.column("l").unwrap().values, vec![4.0, 6.0]);
        assert_eq!(table.column("gain").unwrap().values, vec![10.0, 30.0]);
        assert_eq!(table.column("gm").unwrap().values, vec![0.1, 0.3]);
    }

    #[test]
    fn reports_column_mask_length_mismatch() {
        let parameter_axes = vec![ExplorationColumn::new("w", vec![1.0, 2.0])];
        let mask = vec![true, false, true];

        assert_eq!(
            assemble_filtered_table(&parameter_axes, &[], &[], &mask),
            Err(ExplorationTableError::MaskLengthMismatch {
                column: "w".to_string(),
                mask_len: 3,
                values_len: 2,
            })
        );
    }

    #[test]
    fn allows_empty_column_groups() {
        let mask = vec![true, false, true];
        let table = assemble_filtered_table(&[], &[], &[], &mask).unwrap();

        assert_eq!(table.row_count, 2);
        assert!(table.columns.is_empty());
    }

    #[test]
    fn adds_sum_column_for_area() {
        let mut table = ExplorationTable {
            row_count: 3,
            columns: vec![
                ExplorationColumn::new("w1", vec![1.0, 2.0, 3.0]),
                ExplorationColumn::new("w2", vec![4.0, 5.0, 6.0]),
            ],
        };

        table.add_sum_column("area", &["w1", "w2"]).unwrap();

        assert_eq!(table.column("area").unwrap().values, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn reports_missing_source_column_for_sum_column() {
        let mut table = ExplorationTable {
            row_count: 1,
            columns: vec![ExplorationColumn::new("w1", vec![1.0])],
        };

        assert_eq!(
            table.add_sum_column("area", &["w1", "w2"]),
            Err(ExplorationTableError::MissingColumn {
                column: "w2".to_string(),
            })
        );
    }

    #[test]
    fn adds_zero_sum_column_when_no_sources_are_given() {
        let mut table = ExplorationTable {
            row_count: 2,
            columns: Vec::new(),
        };

        table.add_sum_column("area", &[]).unwrap();

        assert_eq!(table.column("area").unwrap().values, vec![0.0, 0.0]);
    }
}
