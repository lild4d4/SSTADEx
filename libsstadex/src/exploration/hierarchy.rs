use std::collections::HashMap;

use super::{
    candidate_set_from_columns, CandidateAxis, CandidateSet, CompactOutputBinding,
    ExplorationColumn, ExplorationFilter, ExplorationSpec, ExplorationTable, TestbenchSpec,
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
    if bindings.is_empty() {
        return Err(SubmacroCandidateError::EmptyBindings);
    }

    let instance = instance.as_ref();
    let mut columns = Vec::with_capacity(bindings.len());

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

    Ok(candidate_set_from_columns(instance, &columns)?)
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
}
