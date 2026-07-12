use std::collections::HashMap;

use super::{
    CandidateAxis, CandidatePoint, CandidateSet, CompactOutputBinding, ExplorationColumn,
    ExplorationFilter, ExplorationSpec, ExplorationTable, FilterPhase, RangeCondition,
    TestbenchSpec, candidate_column_name, candidate_set_from_columns,
    evaluate_candidate_expression,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MacroExplorationWorkspace {
    pub macro_name: String,
    pub testbenches: Vec<TestbenchSpec>,
    pub specs: Vec<ExplorationSpec>,
    pub candidates: HierarchicalCandidateInput,
    pub outputs: Vec<ExplorationOutput>,
    pub interface_variables: Vec<InterfaceVariable>,
    pub submacro_condition_rules: Vec<SubmacroConditionRule>,
    pub last_results: Option<ExplorationTable>,
}

impl MacroExplorationWorkspace {
    pub fn new(macro_name: impl Into<String>) -> Self {
        Self {
            macro_name: macro_name.into(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HierarchicalExplorationProject {
    pub workspaces: HashMap<String, MacroExplorationWorkspace>,
}

impl HierarchicalExplorationProject {
    pub fn new(workspaces: Vec<MacroExplorationWorkspace>) -> Self {
        Self {
            workspaces: workspaces
                .into_iter()
                .map(|workspace| (workspace.macro_name.clone(), workspace))
                .collect(),
        }
    }

    pub fn workspace(&self, macro_name: &str) -> Option<&MacroExplorationWorkspace> {
        self.workspaces.get(macro_name)
    }

    pub fn workspace_mut(&mut self, macro_name: &str) -> Option<&mut MacroExplorationWorkspace> {
        self.workspaces.get_mut(macro_name)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HierarchicalCandidateInput {
    pub axes: Vec<CandidateAxis>,
    pub sets: Vec<CandidateSet>,
    pub filters: Vec<ExplorationFilter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplorationOutput {
    pub name: String,
    pub source_column: String,
}

impl ExplorationOutput {
    pub fn new(name: impl Into<String>, source_column: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source_column: source_column.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceVariable {
    pub name: String,
    pub source_column: String,
}

impl InterfaceVariable {
    pub fn new(name: impl Into<String>, source_column: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source_column: source_column.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmacroConditionRule {
    pub instance: String,
    pub target_column: String,
    pub source: SubmacroConditionSource,
}

impl SubmacroConditionRule {
    pub fn new(
        instance: impl Into<String>,
        target_column: impl Into<String>,
        source: SubmacroConditionSource,
    ) -> Self {
        Self {
            instance: instance.into(),
            target_column: target_column.into(),
            source,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmacroConditionSource {
    AllowedValuesFromParent {
        column: String,
    },
    RangeFromParent {
        column: String,
    },
    FixedRange {
        min: Option<String>,
        max: Option<String>,
    },
    Expression {
        expression: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmacroCandidateError {
    EmptyBindings,
    MissingColumn { column: String },
    CandidateSetBuild(super::CandidateSetBuildError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmacroConditionError {
    MissingParentColumn { column: String },
    InvalidNumber { value: String },
    CandidateEvaluation(super::CandidateEvaluationError),
}

impl From<super::CandidateEvaluationError> for SubmacroConditionError {
    fn from(error: super::CandidateEvaluationError) -> Self {
        Self::CandidateEvaluation(error)
    }
}

impl From<super::CandidateSetBuildError> for SubmacroCandidateError {
    fn from(error: super::CandidateSetBuildError) -> Self {
        Self::CandidateSetBuild(error)
    }
}

pub fn compact_parameter_column_name(
    compact_parameter: impl AsRef<str>,
    instance: impl AsRef<str>,
) -> String {
    format!(
        "{}__{}",
        compact_parameter.as_ref().trim().replace('-', "_"),
        instance.as_ref().trim().replace('-', "_")
    )
}

pub fn submacro_results_to_compact_candidate_set(
    instance: impl AsRef<str>,
    bindings: &[CompactOutputBinding],
    table: &ExplorationTable,
) -> Result<CandidateSet, SubmacroCandidateError> {
    submacro_results_to_candidate_set(instance, bindings, &[], table)
}

pub fn submacro_results_to_candidate_set(
    instance: impl AsRef<str>,
    bindings: &[CompactOutputBinding],
    interface_variables: &[InterfaceVariable],
    table: &ExplorationTable,
) -> Result<CandidateSet, SubmacroCandidateError> {
    if bindings.is_empty() {
        return Err(SubmacroCandidateError::EmptyBindings);
    }

    let instance = instance.as_ref();
    let mut columns = Vec::with_capacity(bindings.len() + table.columns.len());

    for binding in bindings {
        let source = table.column(&binding.source_column).ok_or_else(|| {
            SubmacroCandidateError::MissingColumn {
                column: binding.source_column.clone(),
            }
        })?;
        columns.push(ExplorationColumn::new(
            compact_parameter_column_name(&binding.compact_parameter, instance),
            source.values.clone(),
        ));
    }

    columns.extend(submacro_interface_columns(
        instance,
        interface_variables,
        table,
    )?);
    columns.extend(submacro_sizing_columns(instance, table));

    Ok(candidate_set_from_columns(instance, &columns)?)
}

fn submacro_interface_columns(
    instance: &str,
    interface_variables: &[InterfaceVariable],
    table: &ExplorationTable,
) -> Result<Vec<ExplorationColumn>, SubmacroCandidateError> {
    let mut columns = Vec::with_capacity(interface_variables.len());

    for variable in interface_variables {
        let source = table.column(&variable.source_column).ok_or_else(|| {
            SubmacroCandidateError::MissingColumn {
                column: variable.source_column.clone(),
            }
        })?;
        columns.push(ExplorationColumn::new(
            candidate_column_name(instance, &variable.name),
            source.values.clone(),
        ));
    }

    Ok(columns)
}

fn submacro_sizing_columns(instance: &str, table: &ExplorationTable) -> Vec<ExplorationColumn> {
    table
        .columns
        .iter()
        .filter(|column| is_sizing_column(&column.name))
        .map(|column| {
            ExplorationColumn::new(
                candidate_column_name(instance, &column.name),
                column.values.clone(),
            )
        })
        .collect()
}

fn is_sizing_column(name: &str) -> bool {
    let local_name = name.rsplit('.').next().unwrap_or(name);
    local_name.starts_with("width_")
        || local_name.starts_with("width__")
        || local_name.starts_with("length_")
        || local_name.starts_with("length__")
}

pub fn derive_submacro_condition_filters(
    instance: impl AsRef<str>,
    rules: &[SubmacroConditionRule],
    child_workspace: &MacroExplorationWorkspace,
    parent_table: &ExplorationTable,
) -> Result<Vec<ExplorationFilter>, SubmacroConditionError> {
    let instance = instance.as_ref();
    let mut filters = Vec::new();

    for rule in rules.iter().filter(|rule| rule.instance == instance) {
        let target_column = child_interface_source_column(child_workspace, &rule.target_column);

        match &rule.source {
            SubmacroConditionSource::AllowedValuesFromParent { column } => {
                let source = required_parent_column(parent_table, column)?;
                filters.push(ExplorationFilter::allowed_values(
                    FilterPhase::CandidatePreEvaluation,
                    target_column,
                    unique_values(&source.values),
                ));
            }
            SubmacroConditionSource::RangeFromParent { column } => {
                let source = required_parent_column(parent_table, column)?;
                filters.push(ExplorationFilter::new(
                    FilterPhase::CandidatePreEvaluation,
                    target_column,
                    range_from_values(&source.values),
                ));
            }
            SubmacroConditionSource::FixedRange { min, max } => {
                filters.push(ExplorationFilter::new(
                    FilterPhase::CandidatePreEvaluation,
                    target_column,
                    RangeCondition::new(parse_optional_f64(min)?, parse_optional_f64(max)?),
                ));
            }
            SubmacroConditionSource::Expression { expression } => {
                let values = evaluate_parent_expression(parent_table, expression)?;
                filters.push(ExplorationFilter::allowed_values(
                    FilterPhase::CandidatePreEvaluation,
                    target_column,
                    unique_values(&values),
                ));
            }
        }
    }

    Ok(filters)
}

fn child_interface_source_column<'a>(
    child_workspace: &'a MacroExplorationWorkspace,
    target_column: &'a str,
) -> &'a str {
    child_workspace
        .interface_variables
        .iter()
        .find(|variable| variable.name == target_column)
        .map(|variable| variable.source_column.as_str())
        .unwrap_or(target_column)
}

fn required_parent_column<'a>(
    table: &'a ExplorationTable,
    column: &str,
) -> Result<&'a ExplorationColumn, SubmacroConditionError> {
    table
        .column(column)
        .ok_or_else(|| SubmacroConditionError::MissingParentColumn {
            column: column.to_string(),
        })
}

fn unique_values(values: &[f64]) -> Vec<f64> {
    let mut output = Vec::new();
    for value in values {
        if !output.contains(value) {
            output.push(*value);
        }
    }
    output
}

fn range_from_values(values: &[f64]) -> RangeCondition {
    let mut min = None;
    let mut max = None;

    for value in values {
        min = Some(min.map_or(*value, |current: f64| current.min(*value)));
        max = Some(max.map_or(*value, |current: f64| current.max(*value)));
    }

    RangeCondition::new(min, max)
}

fn parse_optional_f64(value: &Option<String>) -> Result<Option<f64>, SubmacroConditionError> {
    let Some(value) = value else {
        return Ok(None);
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    trimmed
        .parse::<f64>()
        .map(Some)
        .map_err(|_| SubmacroConditionError::InvalidNumber {
            value: value.clone(),
        })
}

fn evaluate_parent_expression(
    table: &ExplorationTable,
    expression: &str,
) -> Result<Vec<f64>, SubmacroConditionError> {
    let mut values = Vec::with_capacity(table.row_count);

    for row in 0..table.row_count {
        let candidate = CandidatePoint::new(
            table
                .columns
                .iter()
                .map(|column| (column.name.clone(), column.values[row]))
                .collect(),
        );
        values.push(evaluate_candidate_expression(&candidate, row, expression)?);
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_macro_exploration_workspaces_by_macro_name() {
        let project = HierarchicalExplorationProject::new(vec![
            MacroExplorationWorkspace::new("ota_1stage"),
            MacroExplorationWorkspace::new("current_source"),
        ]);

        assert!(project.workspace("ota_1stage").is_some());
        assert!(project.workspace("current_source").is_some());
        assert!(project.workspace("missing").is_none());
    }

    #[test]
    fn keeps_exploration_metadata_outside_macro_model() {
        let mut workspace = MacroExplorationWorkspace::new("current_source");
        workspace
            .outputs
            .push(ExplorationOutput::new("bias_current", "isource"));
        workspace
            .interface_variables
            .push(InterfaceVariable::new("vout", "xcs.voutp"));
        workspace
            .submacro_condition_rules
            .push(SubmacroConditionRule::new(
                "xcs_macro",
                "vout",
                SubmacroConditionSource::AllowedValuesFromParent {
                    column: "IBIAS".to_string(),
                },
            ));

        assert_eq!(workspace.macro_name, "current_source");
        assert_eq!(workspace.outputs[0].name, "bias_current");
        assert_eq!(workspace.interface_variables[0].source_column, "xcs.voutp");
        assert_eq!(workspace.submacro_condition_rules[0].instance, "xcs_macro");
    }

    #[test]
    fn maps_submacro_results_to_compact_candidate_set() {
        let table = ExplorationTable {
            columns: vec![
                ExplorationColumn::new("bias_current", vec![1.0, 2.0]),
                ExplorationColumn::new("xcs.width_m1", vec![3.0, 4.0]),
                ExplorationColumn::new("xcs.length__m1", vec![0.15, 0.2]),
                ExplorationColumn::new("gain", vec![10.0, 20.0]),
            ],
            row_count: 2,
        };
        let bindings = vec![CompactOutputBinding::new("bias_current", "isource")];

        let candidate_set =
            submacro_results_to_compact_candidate_set("xcs_macro", &bindings, &table).unwrap();

        assert_eq!(candidate_set.name, "xcs_macro");
        assert_eq!(candidate_set.points[0].get("isource__xcs_macro"), Some(1.0));
        assert_eq!(candidate_set.points[1].get("isource__xcs_macro"), Some(2.0));
        assert_eq!(
            candidate_set.points[0].get("xcs_macro.xcs.width_m1"),
            Some(3.0)
        );
        assert_eq!(
            candidate_set.points[1].get("xcs_macro.xcs.length__m1"),
            Some(0.2)
        );
        assert_eq!(candidate_set.points[0].get("xcs_macro.gain"), None);
    }

    #[test]
    fn maps_submacro_results_without_sizing_columns() {
        let table = ExplorationTable {
            columns: vec![ExplorationColumn::new("bias_current", vec![1.0])],
            row_count: 1,
        };
        let bindings = vec![CompactOutputBinding::new("bias_current", "isource")];

        let candidate_set =
            submacro_results_to_compact_candidate_set("xcs_macro", &bindings, &table).unwrap();

        assert_eq!(candidate_set.points[0].values.len(), 1);
        assert_eq!(candidate_set.points[0].get("isource__xcs_macro"), Some(1.0));
    }

    #[test]
    fn maps_submacro_interface_variables_to_public_instance_columns() {
        let table = ExplorationTable {
            columns: vec![
                ExplorationColumn::new("bias_current", vec![1.0, 2.0]),
                ExplorationColumn::new("xcs.voutp", vec![0.8, 0.9]),
            ],
            row_count: 2,
        };
        let bindings = vec![CompactOutputBinding::new("bias_current", "isource")];
        let interfaces = vec![InterfaceVariable::new("vout", "xcs.voutp")];

        let candidate_set =
            submacro_results_to_candidate_set("xcs_macro", &bindings, &interfaces, &table).unwrap();

        assert_eq!(candidate_set.points[0].get("xcs_macro.vout"), Some(0.8));
        assert_eq!(candidate_set.points[1].get("xcs_macro.vout"), Some(0.9));
    }

    #[test]
    fn reports_missing_compact_output_source_column() {
        let table = ExplorationTable {
            columns: vec![ExplorationColumn::new("gain", vec![10.0])],
            row_count: 1,
        };
        let bindings = vec![CompactOutputBinding::new("bias_current", "isource")];

        let error =
            submacro_results_to_compact_candidate_set("xcs_macro", &bindings, &table).unwrap_err();

        assert_eq!(
            error,
            SubmacroCandidateError::MissingColumn {
                column: "bias_current".to_string(),
            }
        );
    }

    #[test]
    fn derives_allowed_values_filter_for_submacro_interface_variable() {
        let mut child = MacroExplorationWorkspace::new("current_source");
        child
            .interface_variables
            .push(InterfaceVariable::new("vout", "xcs.voutp"));
        let parent_table = ExplorationTable {
            columns: vec![ExplorationColumn::new("parent_vout", vec![0.8, 0.9, 0.8])],
            row_count: 3,
        };
        let rules = vec![SubmacroConditionRule::new(
            "xcs_macro",
            "vout",
            SubmacroConditionSource::AllowedValuesFromParent {
                column: "parent_vout".to_string(),
            },
        )];

        let filters =
            derive_submacro_condition_filters("xcs_macro", &rules, &child, &parent_table).unwrap();

        assert_eq!(
            filters,
            vec![ExplorationFilter::allowed_values(
                FilterPhase::CandidatePreEvaluation,
                "xcs.voutp",
                vec![0.8, 0.9],
            )]
        );
    }

    #[test]
    fn derives_range_filter_from_parent_column() {
        let child = MacroExplorationWorkspace::new("current_source");
        let parent_table = ExplorationTable {
            columns: vec![ExplorationColumn::new("vout_window", vec![0.7, 0.9])],
            row_count: 2,
        };
        let rules = vec![SubmacroConditionRule::new(
            "xcs_macro",
            "xcs.voutp",
            SubmacroConditionSource::RangeFromParent {
                column: "vout_window".to_string(),
            },
        )];

        let filters =
            derive_submacro_condition_filters("xcs_macro", &rules, &child, &parent_table).unwrap();

        assert_eq!(
            filters,
            vec![ExplorationFilter::new(
                FilterPhase::CandidatePreEvaluation,
                "xcs.voutp",
                RangeCondition::new(Some(0.7), Some(0.9)),
            )]
        );
    }

    #[test]
    fn derives_allowed_values_filter_from_parent_expression() {
        let child = MacroExplorationWorkspace::new("current_source");
        let parent_table = ExplorationTable {
            columns: vec![ExplorationColumn::new("vout", vec![0.8, 0.9])],
            row_count: 2,
        };
        let rules = vec![SubmacroConditionRule::new(
            "xcs_macro",
            "xcs.voutp",
            SubmacroConditionSource::Expression {
                expression: "vout + 0.1".to_string(),
            },
        )];

        let filters =
            derive_submacro_condition_filters("xcs_macro", &rules, &child, &parent_table).unwrap();

        assert_eq!(
            filters,
            vec![ExplorationFilter::allowed_values(
                FilterPhase::CandidatePreEvaluation,
                "xcs.voutp",
                vec![0.9, 1.0],
            )]
        );
    }

    #[test]
    fn derives_fixed_range_filter() {
        let child = MacroExplorationWorkspace::new("current_source");
        let parent_table = ExplorationTable {
            columns: Vec::new(),
            row_count: 0,
        };
        let rules = vec![SubmacroConditionRule::new(
            "xcs_macro",
            "xcs.voutp",
            SubmacroConditionSource::FixedRange {
                min: Some("0.2".to_string()),
                max: Some("1.0".to_string()),
            },
        )];

        let filters =
            derive_submacro_condition_filters("xcs_macro", &rules, &child, &parent_table).unwrap();

        assert_eq!(
            filters,
            vec![ExplorationFilter::new(
                FilterPhase::CandidatePreEvaluation,
                "xcs.voutp",
                RangeCondition::new(Some(0.2), Some(1.0)),
            )]
        );
    }
}
