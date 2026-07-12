use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ResolvedSubmacroConditions {
    pub pre_build_constraints: Vec<SubmacroPreBuildConstraint>,
    pub filters: Vec<ExplorationFilter>,
}

impl ResolvedSubmacroConditions {
    pub fn is_empty(&self) -> bool {
        self.pre_build_constraints.is_empty() && self.filters.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubmacroPreBuildConstraint {
    pub target_column: String,
    pub condition: SubmacroPreBuildCondition,
}

impl SubmacroPreBuildConstraint {
    pub fn allowed_values(target_column: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            target_column: target_column.into(),
            condition: SubmacroPreBuildCondition::AllowedValues(values),
        }
    }

    pub fn range(target_column: impl Into<String>, range: RangeCondition) -> Self {
        Self {
            target_column: target_column.into(),
            condition: SubmacroPreBuildCondition::Range(range),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubmacroPreBuildCondition {
    AllowedValues(Vec<f64>),
    Range(RangeCondition),
}

#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyDfsProject {
    pub top_macro: String,
    pub nodes: HashMap<String, HierarchyDfsNode>,
}

impl HierarchyDfsProject {
    pub fn new(top_macro: impl Into<String>, nodes: Vec<HierarchyDfsNode>) -> Self {
        Self {
            top_macro: top_macro.into(),
            nodes: nodes
                .into_iter()
                .map(|node| (node.macro_name.clone(), node))
                .collect(),
        }
    }

    pub fn node(&self, macro_name: &str) -> Option<&HierarchyDfsNode> {
        self.nodes.get(macro_name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyDfsNode {
    pub macro_name: String,
    pub workspace: MacroExplorationWorkspace,
    pub children: Vec<HierarchyDfsChild>,
    pub compact_outputs: Vec<CompactOutputBinding>,
    pub pre_build_columns: Vec<String>,
}

impl HierarchyDfsNode {
    pub fn new(macro_name: impl Into<String>, workspace: MacroExplorationWorkspace) -> Self {
        let macro_name = macro_name.into();
        Self {
            macro_name,
            workspace,
            children: Vec::new(),
            compact_outputs: Vec::new(),
            pre_build_columns: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyDfsChild {
    pub instance_name: String,
    pub macro_name: String,
}

impl HierarchyDfsChild {
    pub fn new(instance_name: impl Into<String>, macro_name: impl Into<String>) -> Self {
        Self {
            instance_name: instance_name.into(),
            macro_name: macro_name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyDfsEvaluationRequest {
    pub macro_name: String,
    pub event: String,
    pub child_candidate_sets: Vec<CandidateSet>,
    pub inherited_constraints: ResolvedSubmacroConditions,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyDfsRun {
    pub top_table: ExplorationTable,
    pub results_by_macro: HashMap<String, ExplorationTable>,
    pub events: Vec<HierarchyDfsEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HierarchyDfsEvent {
    Enter {
        macro_name: String,
        level: usize,
        pre_build_constraints: usize,
        filters: usize,
    },
    Evaluate {
        macro_name: String,
        event: String,
        rows: usize,
    },
    Descend {
        parent_macro: String,
        instance_name: String,
        child_macro: String,
        level: usize,
        pre_build_constraints: usize,
        filters: usize,
    },
    RefreshParent {
        macro_name: String,
        instance_name: String,
        rows: usize,
    },
    Leave {
        macro_name: String,
        level: usize,
        rows: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HierarchyDfsError {
    MissingNode {
        macro_name: String,
    },
    Cycle {
        macro_name: String,
    },
    Condition(SubmacroConditionError),
    Candidate(SubmacroCandidateError),
    Evaluation {
        macro_name: String,
        event: String,
        message: String,
    },
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

pub fn run_hierarchical_dfs<F>(
    project: &HierarchyDfsProject,
    mut evaluate: F,
) -> Result<HierarchyDfsRun, HierarchyDfsError>
where
    F: FnMut(HierarchyDfsEvaluationRequest) -> Result<ExplorationTable, String>,
{
    let mut state = HierarchyDfsState::default();
    let top_table = run_hierarchical_dfs_node(
        project,
        &project.top_macro,
        ResolvedSubmacroConditions::default(),
        Vec::new(),
        0,
        &mut evaluate,
        &mut state,
    )?;

    Ok(HierarchyDfsRun {
        top_table,
        results_by_macro: state.results_by_macro,
        events: state.events,
    })
}

#[derive(Default)]
struct HierarchyDfsState {
    results_by_macro: HashMap<String, ExplorationTable>,
    events: Vec<HierarchyDfsEvent>,
    visiting: HashSet<String>,
}

fn run_hierarchical_dfs_node<F>(
    project: &HierarchyDfsProject,
    macro_name: &str,
    inherited_constraints: ResolvedSubmacroConditions,
    mut child_candidate_sets: Vec<CandidateSet>,
    level: usize,
    evaluate: &mut F,
    state: &mut HierarchyDfsState,
) -> Result<ExplorationTable, HierarchyDfsError>
where
    F: FnMut(HierarchyDfsEvaluationRequest) -> Result<ExplorationTable, String>,
{
    if !state.visiting.insert(macro_name.to_string()) {
        return Err(HierarchyDfsError::Cycle {
            macro_name: macro_name.to_string(),
        });
    }
    let node = project
        .node(macro_name)
        .ok_or_else(|| HierarchyDfsError::MissingNode {
            macro_name: macro_name.to_string(),
        })?;
    state.events.push(HierarchyDfsEvent::Enter {
        macro_name: macro_name.to_string(),
        level,
        pre_build_constraints: inherited_constraints.pre_build_constraints.len(),
        filters: inherited_constraints.filters.len(),
    });

    let mut current_table = evaluate_hierarchy_dfs_event(
        macro_name,
        "initial",
        child_candidate_sets.clone(),
        inherited_constraints.clone(),
        evaluate,
        state,
    )?;

    if node.children.is_empty() {
        current_table = evaluate_hierarchy_dfs_event(
            macro_name,
            "final",
            child_candidate_sets,
            inherited_constraints,
            evaluate,
            state,
        )?;
        state.events.push(HierarchyDfsEvent::Leave {
            macro_name: macro_name.to_string(),
            level,
            rows: current_table.row_count,
        });
        state
            .results_by_macro
            .insert(macro_name.to_string(), current_table.clone());
        state.visiting.remove(macro_name);
        return Ok(current_table);
    }

    for child in &node.children {
        let child_node =
            project
                .node(&child.macro_name)
                .ok_or_else(|| HierarchyDfsError::MissingNode {
                    macro_name: child.macro_name.clone(),
                })?;
        let child_constraints = resolve_submacro_conditions(
            &child.instance_name,
            &node.workspace.submacro_condition_rules,
            &child_node.workspace,
            &current_table,
            &child_node.pre_build_columns,
        )
        .map_err(HierarchyDfsError::Condition)?;
        state.events.push(HierarchyDfsEvent::Descend {
            parent_macro: macro_name.to_string(),
            instance_name: child.instance_name.clone(),
            child_macro: child.macro_name.clone(),
            level,
            pre_build_constraints: child_constraints.pre_build_constraints.len(),
            filters: child_constraints.filters.len(),
        });

        let child_table = run_hierarchical_dfs_node(
            project,
            &child.macro_name,
            child_constraints,
            Vec::new(),
            level + 1,
            evaluate,
            state,
        )?;
        let child_set = submacro_results_to_candidate_set(
            &child.instance_name,
            &child_node.compact_outputs,
            &child_node.workspace.interface_variables,
            &child_table,
        )
        .map_err(HierarchyDfsError::Candidate)?;
        child_candidate_sets.push(child_set);

        let event = format!(
            "after_child_{}",
            sanitize_hierarchy_event_name(&child.instance_name)
        );
        current_table = evaluate_hierarchy_dfs_event(
            macro_name,
            &event,
            child_candidate_sets.clone(),
            inherited_constraints.clone(),
            evaluate,
            state,
        )?;
        state.events.push(HierarchyDfsEvent::RefreshParent {
            macro_name: macro_name.to_string(),
            instance_name: child.instance_name.clone(),
            rows: current_table.row_count,
        });
    }

    current_table = evaluate_hierarchy_dfs_event(
        macro_name,
        "final",
        child_candidate_sets,
        inherited_constraints,
        evaluate,
        state,
    )?;
    state.events.push(HierarchyDfsEvent::Leave {
        macro_name: macro_name.to_string(),
        level,
        rows: current_table.row_count,
    });
    state
        .results_by_macro
        .insert(macro_name.to_string(), current_table.clone());
    state.visiting.remove(macro_name);
    Ok(current_table)
}

fn evaluate_hierarchy_dfs_event<F>(
    macro_name: &str,
    event: &str,
    child_candidate_sets: Vec<CandidateSet>,
    inherited_constraints: ResolvedSubmacroConditions,
    evaluate: &mut F,
    state: &mut HierarchyDfsState,
) -> Result<ExplorationTable, HierarchyDfsError>
where
    F: FnMut(HierarchyDfsEvaluationRequest) -> Result<ExplorationTable, String>,
{
    let table = evaluate(HierarchyDfsEvaluationRequest {
        macro_name: macro_name.to_string(),
        event: event.to_string(),
        child_candidate_sets,
        inherited_constraints,
    })
    .map_err(|message| HierarchyDfsError::Evaluation {
        macro_name: macro_name.to_string(),
        event: event.to_string(),
        message,
    })?;
    state.events.push(HierarchyDfsEvent::Evaluate {
        macro_name: macro_name.to_string(),
        event: event.to_string(),
        rows: table.row_count,
    });
    Ok(table)
}

pub fn sanitize_hierarchy_event_name(name: &str) -> String {
    let sanitized = name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>();
    if sanitized.is_empty() {
        "event".to_string()
    } else {
        sanitized
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
    Ok(resolve_submacro_conditions(instance, rules, child_workspace, parent_table, &[])?.filters)
}

pub fn resolve_submacro_conditions(
    instance: impl AsRef<str>,
    rules: &[SubmacroConditionRule],
    child_workspace: &MacroExplorationWorkspace,
    parent_table: &ExplorationTable,
    pre_build_columns: &[String],
) -> Result<ResolvedSubmacroConditions, SubmacroConditionError> {
    let instance = instance.as_ref();
    let pre_build_columns = pre_build_columns
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let mut resolved = ResolvedSubmacroConditions::default();

    for rule in rules.iter().filter(|rule| rule.instance == instance) {
        let target_column = child_interface_source_column(child_workspace, &rule.target_column);
        let pre_build = pre_build_columns.contains(target_column);

        match &rule.source {
            SubmacroConditionSource::AllowedValuesFromParent { column } => {
                let source = required_parent_column(parent_table, column)?;
                let values = unique_values(&source.values);
                push_resolved_condition(
                    &mut resolved,
                    pre_build,
                    target_column,
                    SubmacroPreBuildCondition::AllowedValues(values),
                );
            }
            SubmacroConditionSource::RangeFromParent { column } => {
                let source = required_parent_column(parent_table, column)?;
                push_resolved_condition(
                    &mut resolved,
                    pre_build,
                    target_column,
                    SubmacroPreBuildCondition::Range(range_from_values(&source.values)),
                );
            }
            SubmacroConditionSource::FixedRange { min, max } => {
                push_resolved_condition(
                    &mut resolved,
                    pre_build,
                    target_column,
                    SubmacroPreBuildCondition::Range(RangeCondition::new(
                        parse_optional_f64(min)?,
                        parse_optional_f64(max)?,
                    )),
                );
            }
            SubmacroConditionSource::Expression { expression } => {
                let values = evaluate_parent_expression(parent_table, expression)?;
                push_resolved_condition(
                    &mut resolved,
                    pre_build,
                    target_column,
                    SubmacroPreBuildCondition::AllowedValues(unique_values(&values)),
                );
            }
        }
    }

    Ok(resolved)
}

fn push_resolved_condition(
    resolved: &mut ResolvedSubmacroConditions,
    pre_build: bool,
    target_column: &str,
    condition: SubmacroPreBuildCondition,
) {
    if pre_build {
        resolved
            .pre_build_constraints
            .push(SubmacroPreBuildConstraint {
                target_column: target_column.to_string(),
                condition,
            });
        return;
    }

    let filter = match condition {
        SubmacroPreBuildCondition::AllowedValues(values) => ExplorationFilter::allowed_values(
            FilterPhase::CandidatePreEvaluation,
            target_column,
            values,
        ),
        SubmacroPreBuildCondition::Range(range) => {
            ExplorationFilter::new(FilterPhase::CandidatePreEvaluation, target_column, range)
        }
    };
    resolved.filters.push(filter);
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
    fn resolves_allowed_values_as_pre_build_constraint_for_port_voltage_interface() {
        let mut child = MacroExplorationWorkspace::new("current_source");
        child
            .interface_variables
            .push(InterfaceVariable::new("vout", "xcs.voutp"));
        let parent_table = ExplorationTable {
            columns: vec![ExplorationColumn::new("parent_vout", vec![0.6, 0.6])],
            row_count: 2,
        };
        let rules = vec![SubmacroConditionRule::new(
            "xcs_macro",
            "vout",
            SubmacroConditionSource::AllowedValuesFromParent {
                column: "parent_vout".to_string(),
            },
        )];

        let resolved = resolve_submacro_conditions(
            "xcs_macro",
            &rules,
            &child,
            &parent_table,
            &[String::from("xcs.voutp")],
        )
        .unwrap();

        assert_eq!(resolved.filters, Vec::new());
        assert_eq!(
            resolved.pre_build_constraints,
            vec![SubmacroPreBuildConstraint::allowed_values(
                "xcs.voutp",
                vec![0.6],
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

    #[test]
    fn dfs_runner_descends_child_and_refreshes_parent() {
        let mut top_workspace = MacroExplorationWorkspace::new("ota");
        top_workspace
            .submacro_condition_rules
            .push(SubmacroConditionRule::new(
                "xcs_macro",
                "vout",
                SubmacroConditionSource::AllowedValuesFromParent {
                    column: "parent_vout".to_string(),
                },
            ));
        let mut child_workspace = MacroExplorationWorkspace::new("current_source");
        child_workspace
            .interface_variables
            .push(InterfaceVariable::new("vout", "xcs.voutp"));

        let mut top = HierarchyDfsNode::new("ota", top_workspace);
        top.children
            .push(HierarchyDfsChild::new("xcs_macro", "current_source"));
        let mut child = HierarchyDfsNode::new("current_source", child_workspace);
        child
            .compact_outputs
            .push(CompactOutputBinding::new("bias_current", "isource"));
        child.pre_build_columns.push("xcs.voutp".to_string());

        let project = HierarchyDfsProject::new("ota", vec![top, child]);
        let mut requests = Vec::new();
        let run = run_hierarchical_dfs(&project, |request| {
            requests.push((
                request.macro_name.clone(),
                request.event.clone(),
                request.child_candidate_sets.len(),
                request.inherited_constraints.pre_build_constraints.len(),
            ));
            match (request.macro_name.as_str(), request.event.as_str()) {
                ("ota", _) => Ok(ExplorationTable {
                    columns: vec![ExplorationColumn::new("parent_vout", vec![0.6])],
                    row_count: 1,
                }),
                ("current_source", _) => Ok(ExplorationTable {
                    columns: vec![
                        ExplorationColumn::new("bias_current", vec![1.0]),
                        ExplorationColumn::new("xcs.voutp", vec![0.6]),
                    ],
                    row_count: 1,
                }),
                _ => Err("unexpected request".to_string()),
            }
        })
        .unwrap();

        assert_eq!(run.top_table.row_count, 1);
        assert!(run.results_by_macro.contains_key("ota"));
        assert!(run.results_by_macro.contains_key("current_source"));
        assert!(requests.contains(&("current_source".to_string(), "initial".to_string(), 0, 1,)));
        assert!(
            requests.contains(&("ota".to_string(), "after_child_xcs_macro".to_string(), 1, 0,))
        );
        assert!(run.events.iter().any(|event| matches!(
            event,
            HierarchyDfsEvent::RefreshParent {
                macro_name,
                instance_name,
                rows: 1,
            } if macro_name == "ota" && instance_name == "xcs_macro"
        )));
    }

    #[test]
    fn dfs_runner_visits_second_child_after_first_refresh() {
        let top_workspace = MacroExplorationWorkspace::new("top");
        let first_workspace = MacroExplorationWorkspace::new("first");
        let second_workspace = MacroExplorationWorkspace::new("second");

        let mut top = HierarchyDfsNode::new("top", top_workspace);
        top.children.push(HierarchyDfsChild::new("x1", "first"));
        top.children.push(HierarchyDfsChild::new("x2", "second"));
        let mut first = HierarchyDfsNode::new("first", first_workspace);
        first
            .compact_outputs
            .push(CompactOutputBinding::new("bias_current", "isource"));
        let mut second = HierarchyDfsNode::new("second", second_workspace);
        second
            .compact_outputs
            .push(CompactOutputBinding::new("bias_current", "isource"));

        let project = HierarchyDfsProject::new("top", vec![top, first, second]);
        let mut top_candidate_counts = Vec::new();
        run_hierarchical_dfs(&project, |request| {
            if request.macro_name == "top" {
                top_candidate_counts
                    .push((request.event.clone(), request.child_candidate_sets.len()));
            }
            if request.macro_name == "first" || request.macro_name == "second" {
                Ok(ExplorationTable {
                    columns: vec![ExplorationColumn::new("bias_current", vec![1.0])],
                    row_count: 1,
                })
            } else {
                Ok(ExplorationTable {
                    columns: vec![ExplorationColumn::new("parent_vout", vec![0.6])],
                    row_count: 1,
                })
            }
        })
        .unwrap();

        assert_eq!(
            top_candidate_counts,
            vec![
                ("initial".to_string(), 0),
                ("after_child_x1".to_string(), 1),
                ("after_child_x2".to_string(), 2),
                ("final".to_string(), 2),
            ]
        );
    }

    #[test]
    fn dfs_runner_reports_cycles() {
        let mut top = HierarchyDfsNode::new("top", MacroExplorationWorkspace::new("top"));
        top.children.push(HierarchyDfsChild::new("xchild", "child"));
        let mut child = HierarchyDfsNode::new("child", MacroExplorationWorkspace::new("child"));
        child.children.push(HierarchyDfsChild::new("xtop", "top"));
        let project = HierarchyDfsProject::new("top", vec![top, child]);

        let error = run_hierarchical_dfs(&project, |_| {
            Ok(ExplorationTable {
                columns: vec![ExplorationColumn::new("value", vec![1.0])],
                row_count: 1,
            })
        })
        .unwrap_err();

        assert_eq!(
            error,
            HierarchyDfsError::Cycle {
                macro_name: "top".to_string(),
            }
        );
    }
}
