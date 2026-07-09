use super::candidate::CandidatePoint;
use super::evaluation::{CandidateEvaluationError, evaluate_candidate_expression};
use super::table::{ExplorationColumn, ExplorationTable, ExplorationTableError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedColumnSpec {
    pub name: String,
    pub expression: String,
}

impl DerivedColumnSpec {
    pub fn new(name: impl Into<String>, expression: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            expression: expression.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedColumnError {
    EmptyName,
    DuplicateColumn { column: String },
    Evaluation(CandidateEvaluationError),
    Table(ExplorationTableError),
}

impl From<CandidateEvaluationError> for DerivedColumnError {
    fn from(error: CandidateEvaluationError) -> Self {
        Self::Evaluation(error)
    }
}

impl From<ExplorationTableError> for DerivedColumnError {
    fn from(error: ExplorationTableError) -> Self {
        Self::Table(error)
    }
}

pub fn apply_derived_columns(
    table: &mut ExplorationTable,
    derived: &[DerivedColumnSpec],
) -> Result<(), DerivedColumnError> {
    for spec in derived {
        let name = spec.name.trim();
        if name.is_empty() {
            return Err(DerivedColumnError::EmptyName);
        }
        if table.column(name).is_some() {
            return Err(DerivedColumnError::DuplicateColumn {
                column: name.to_string(),
            });
        }

        validate_table_columns(table)?;

        let mut values = Vec::with_capacity(table.row_count);
        for row in 0..table.row_count {
            let candidate = table_row_candidate(table, row);
            values.push(evaluate_candidate_expression(
                &candidate,
                row,
                &spec.expression,
            )?);
        }

        table
            .columns
            .push(ExplorationColumn::new(name.to_string(), values));
    }

    Ok(())
}

fn validate_table_columns(table: &ExplorationTable) -> Result<(), ExplorationTableError> {
    for column in &table.columns {
        if column.values.len() != table.row_count {
            return Err(ExplorationTableError::ColumnLengthMismatch {
                column: column.name.clone(),
                expected: table.row_count,
                actual: column.values.len(),
            });
        }
    }

    Ok(())
}

fn table_row_candidate(table: &ExplorationTable, row: usize) -> CandidatePoint {
    CandidatePoint::new(
        table
            .columns
            .iter()
            .map(|column| (column.name.clone(), column.values[row]))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table_with_gain() -> ExplorationTable {
        ExplorationTable {
            row_count: 2,
            columns: vec![ExplorationColumn::new("gain", vec![-10.0, 100.0])],
        }
    }

    #[test]
    fn adds_gain_db_column() {
        let mut table = table_with_gain();

        apply_derived_columns(
            &mut table,
            &[DerivedColumnSpec::new("gain_db", "20 * log10(abs(gain))")],
        )
        .unwrap();

        assert_eq!(table.column("gain_db").unwrap().values, vec![20.0, 40.0]);
    }

    #[test]
    fn adds_sign_changed_column() {
        let mut table = table_with_gain();

        apply_derived_columns(&mut table, &[DerivedColumnSpec::new("gain_pos", "-gain")]).unwrap();

        assert_eq!(table.column("gain_pos").unwrap().values, vec![10.0, -100.0]);
    }

    #[test]
    fn derived_columns_can_use_previous_derived_columns() {
        let mut table = table_with_gain();

        apply_derived_columns(
            &mut table,
            &[
                DerivedColumnSpec::new("gain_abs", "abs(gain)"),
                DerivedColumnSpec::new("gain_db", "20 * log10(gain_abs)"),
            ],
        )
        .unwrap();

        assert_eq!(table.column("gain_abs").unwrap().values, vec![10.0, 100.0]);
        assert_eq!(table.column("gain_db").unwrap().values, vec![20.0, 40.0]);
    }

    #[test]
    fn reports_missing_source_column() {
        let mut table = table_with_gain();

        let error = apply_derived_columns(&mut table, &[DerivedColumnSpec::new("missing", "gm")])
            .unwrap_err();

        assert!(matches!(
            error,
            DerivedColumnError::Evaluation(CandidateEvaluationError::Expression {
                reason,
                ..
            }) if reason.contains("gm")
        ));
    }

    #[test]
    fn reports_duplicate_column() {
        let mut table = table_with_gain();

        let error = apply_derived_columns(&mut table, &[DerivedColumnSpec::new("gain", "-gain")])
            .unwrap_err();

        assert_eq!(
            error,
            DerivedColumnError::DuplicateColumn {
                column: "gain".to_string(),
            }
        );
    }
}
