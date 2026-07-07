use std::collections::HashMap;

use super::{
    CandidateAxis, CandidateSet, ExplorationFilter, ExplorationSpec, ExplorationTable,
    TestbenchSpec,
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
}
