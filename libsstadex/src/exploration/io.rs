use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::candidate::{CandidateAxis, CandidatePoint, CandidateSet};
use super::conditions::RangeCondition;
use super::conditions::{ExplorationFilter, FilterPhase};
use super::spec::{
    CircuitView, CompactOutputBinding, DutRef, ExplorationSpec, FrequencySweep, SpecOutput,
    SpecParameter, SpecSource, SpecVariable, TestbenchElement, TestbenchSpec,
};

#[derive(Debug)]
pub enum ExplorationIoError {
    Io(std::io::Error),
    Json(serde_json::Error),
    DuplicateTestbench { name: String },
    MissingTestbench { name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExplorationCandidateInput {
    pub axes: Vec<CandidateAxis>,
    pub sets: Vec<CandidateSet>,
    pub filters: Vec<ExplorationFilter>,
}

impl From<std::io::Error> for ExplorationIoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for ExplorationIoError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

pub fn load_testbenches(path: &Path) -> Result<Vec<TestbenchSpec>, ExplorationIoError> {
    let content = fs::read_to_string(path)?;
    let document: TestbenchDocument = serde_json::from_str(&content)?;

    Ok(document.into_specs())
}

pub fn save_testbenches(
    path: &Path,
    testbenches: &[TestbenchSpec],
) -> Result<(), ExplorationIoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let document = TestbenchDocument::from_specs(testbenches);
    fs::write(path, serde_json::to_string_pretty(&document)?)?;
    Ok(())
}

pub fn load_exploration_specs(
    path: &Path,
    testbenches: &[TestbenchSpec],
) -> Result<Vec<ExplorationSpec>, ExplorationIoError> {
    let content = fs::read_to_string(path)?;
    let document: ExplorationSpecDocument = serde_json::from_str(&content)?;
    document.into_specs(testbenches)
}

pub fn save_exploration_specs(
    path: &Path,
    specs: &[ExplorationSpec],
) -> Result<(), ExplorationIoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let document = ExplorationSpecDocument::from_specs(specs);
    fs::write(path, serde_json::to_string_pretty(&document)?)?;
    Ok(())
}

pub fn load_exploration_candidates(
    path: &Path,
) -> Result<ExplorationCandidateInput, ExplorationIoError> {
    let content = fs::read_to_string(path)?;
    let document: CandidateInputDocument = serde_json::from_str(&content)?;

    Ok(document.into_input())
}

pub fn save_exploration_candidates(
    path: &Path,
    input: &ExplorationCandidateInput,
) -> Result<(), ExplorationIoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let document = CandidateInputDocument::from_input(input);
    fs::write(path, serde_json::to_string_pretty(&document)?)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TestbenchDocument {
    testbenches: Vec<RawTestbenchSpec>,
}

impl TestbenchDocument {
    fn into_specs(self) -> Vec<TestbenchSpec> {
        self.testbenches
            .into_iter()
            .map(RawTestbenchSpec::into_spec)
            .collect()
    }

    fn from_specs(testbenches: &[TestbenchSpec]) -> Self {
        Self {
            testbenches: testbenches
                .iter()
                .map(RawTestbenchSpec::from_spec)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RawTestbenchSpec {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dut: Option<RawDutRef>,
    #[serde(default)]
    view: RawCircuitView,
    #[serde(default)]
    elements: Vec<RawTestbenchElement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    extra_body: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    compact_outputs: Vec<RawCompactOutputBinding>,
}

impl RawTestbenchSpec {
    fn into_spec(self) -> TestbenchSpec {
        TestbenchSpec {
            name: self.name,
            dut: self.dut.map(RawDutRef::into_dut_ref),
            view: self.view.into_view(),
            elements: self
                .elements
                .into_iter()
                .map(RawTestbenchElement::into_element)
                .collect(),
            extra_body: self.extra_body,
            compact_outputs: self
                .compact_outputs
                .into_iter()
                .map(RawCompactOutputBinding::into_binding)
                .collect(),
        }
    }

    fn from_spec(spec: &TestbenchSpec) -> Self {
        Self {
            name: spec.name.clone(),
            dut: spec.dut.as_ref().map(RawDutRef::from_dut_ref),
            view: RawCircuitView::from_view(spec.view),
            elements: spec
                .elements
                .iter()
                .map(RawTestbenchElement::from_element)
                .collect(),
            extra_body: spec.extra_body.clone(),
            compact_outputs: spec
                .compact_outputs
                .iter()
                .map(RawCompactOutputBinding::from_binding)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RawCompactOutputBinding {
    source_column: String,
    compact_parameter: String,
}

impl RawCompactOutputBinding {
    fn into_binding(self) -> CompactOutputBinding {
        CompactOutputBinding {
            source_column: self.source_column,
            compact_parameter: self.compact_parameter,
        }
    }

    fn from_binding(binding: &CompactOutputBinding) -> Self {
        Self {
            source_column: binding.source_column.clone(),
            compact_parameter: binding.compact_parameter.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawDutRef {
    Macro { name: String },
    Circuit { name: String },
}

impl RawDutRef {
    fn into_dut_ref(self) -> DutRef {
        match self {
            Self::Macro { name } => DutRef::Macro(name),
            Self::Circuit { name } => DutRef::Circuit(name),
        }
    }

    fn from_dut_ref(dut: &DutRef) -> Self {
        match dut {
            DutRef::Macro(name) => Self::Macro { name: name.clone() },
            DutRef::Circuit(name) => Self::Circuit { name: name.clone() },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawCircuitView {
    SmallSignal,
}

impl Default for RawCircuitView {
    fn default() -> Self {
        Self::SmallSignal
    }
}

impl RawCircuitView {
    fn into_view(self) -> CircuitView {
        match self {
            Self::SmallSignal => CircuitView::SmallSignal,
        }
    }

    fn from_view(view: CircuitView) -> Self {
        match view {
            CircuitView::SmallSignal => Self::SmallSignal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawTestbenchElement {
    VoltageSource {
        name: String,
        nplus: String,
        nminus: String,
        value: String,
    },
    CurrentSource {
        name: String,
        nplus: String,
        nminus: String,
        value: String,
    },
    Resistor {
        name: String,
        n1: String,
        n2: String,
        value: String,
    },
    Capacitor {
        name: String,
        n1: String,
        n2: String,
        value: String,
    },
}

impl RawTestbenchElement {
    fn into_element(self) -> TestbenchElement {
        match self {
            Self::VoltageSource {
                name,
                nplus,
                nminus,
                value,
            } => TestbenchElement::VoltageSource {
                name,
                nplus,
                nminus,
                value,
            },
            Self::CurrentSource {
                name,
                nplus,
                nminus,
                value,
            } => TestbenchElement::CurrentSource {
                name,
                nplus,
                nminus,
                value,
            },
            Self::Resistor {
                name,
                n1,
                n2,
                value,
            } => TestbenchElement::Resistor {
                name,
                n1,
                n2,
                value,
            },
            Self::Capacitor {
                name,
                n1,
                n2,
                value,
            } => TestbenchElement::Capacitor {
                name,
                n1,
                n2,
                value,
            },
        }
    }

    fn from_element(element: &TestbenchElement) -> Self {
        match element {
            TestbenchElement::VoltageSource {
                name,
                nplus,
                nminus,
                value,
            } => Self::VoltageSource {
                name: name.clone(),
                nplus: nplus.clone(),
                nminus: nminus.clone(),
                value: value.clone(),
            },
            TestbenchElement::CurrentSource {
                name,
                nplus,
                nminus,
                value,
            } => Self::CurrentSource {
                name: name.clone(),
                nplus: nplus.clone(),
                nminus: nminus.clone(),
                value: value.clone(),
            },
            TestbenchElement::Resistor {
                name,
                n1,
                n2,
                value,
            } => Self::Resistor {
                name: name.clone(),
                n1: n1.clone(),
                n2: n2.clone(),
                value: value.clone(),
            },
            TestbenchElement::Capacitor {
                name,
                n1,
                n2,
                value,
            } => Self::Capacitor {
                name: name.clone(),
                n1: n1.clone(),
                n2: n2.clone(),
                value: value.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CandidateInputDocument {
    #[serde(default)]
    axes: Vec<RawCandidateAxis>,
    #[serde(default)]
    sets: Vec<RawCandidateSet>,
    #[serde(default)]
    filters: Vec<RawExplorationFilter>,
}

impl CandidateInputDocument {
    fn into_input(self) -> ExplorationCandidateInput {
        ExplorationCandidateInput {
            axes: self
                .axes
                .into_iter()
                .map(RawCandidateAxis::into_axis)
                .collect(),
            sets: self
                .sets
                .into_iter()
                .map(RawCandidateSet::into_set)
                .collect(),
            filters: self
                .filters
                .into_iter()
                .map(RawExplorationFilter::into_filter)
                .collect(),
        }
    }

    fn from_input(input: &ExplorationCandidateInput) -> Self {
        Self {
            axes: input.axes.iter().map(RawCandidateAxis::from_axis).collect(),
            sets: input.sets.iter().map(RawCandidateSet::from_set).collect(),
            filters: input
                .filters
                .iter()
                .map(RawExplorationFilter::from_filter)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RawCandidateAxis {
    name: String,
    values: Vec<f64>,
}

impl RawCandidateAxis {
    fn into_axis(self) -> CandidateAxis {
        CandidateAxis::new(self.name, self.values)
    }

    fn from_axis(axis: &CandidateAxis) -> Self {
        Self {
            name: axis.name.clone(),
            values: axis.values.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RawCandidateSet {
    name: String,
    points: Vec<BTreeMap<String, f64>>,
}

impl RawCandidateSet {
    fn into_set(self) -> CandidateSet {
        CandidateSet::new(
            self.name,
            self.points
                .into_iter()
                .map(|point| CandidatePoint::new(point.into_iter().collect()))
                .collect(),
        )
    }

    fn from_set(set: &CandidateSet) -> Self {
        Self {
            name: set.name.clone(),
            points: set
                .points
                .iter()
                .map(|point| point.values.iter().cloned().collect())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawExplorationFilter {
    Range {
        phase: RawFilterPhase,
        column: String,
        condition: RawRangeCondition,
    },
    AllowedValues {
        phase: RawFilterPhase,
        column: String,
        values: Vec<f64>,
    },
    EqualColumns {
        phase: RawFilterPhase,
        columns: Vec<String>,
    },
}

impl RawExplorationFilter {
    fn into_filter(self) -> ExplorationFilter {
        match self {
            Self::Range {
                phase,
                column,
                condition,
            } => ExplorationFilter::new(phase.into_phase(), column, condition.into_condition()),
            Self::AllowedValues {
                phase,
                column,
                values,
            } => ExplorationFilter::allowed_values(phase.into_phase(), column, values),
            Self::EqualColumns { phase, columns } => {
                ExplorationFilter::equal_columns(phase.into_phase(), columns)
            }
        }
    }

    fn from_filter(filter: &ExplorationFilter) -> Self {
        match filter {
            ExplorationFilter::Range {
                phase,
                column,
                condition,
            } => Self::Range {
                phase: RawFilterPhase::from_phase(*phase),
                column: column.clone(),
                condition: RawRangeCondition::from_condition(*condition),
            },
            ExplorationFilter::AllowedValues {
                phase,
                column,
                values,
            } => Self::AllowedValues {
                phase: RawFilterPhase::from_phase(*phase),
                column: column.clone(),
                values: values.clone(),
            },
            ExplorationFilter::EqualColumns { phase, columns } => Self::EqualColumns {
                phase: RawFilterPhase::from_phase(*phase),
                columns: columns.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawFilterPhase {
    AxisPreEvaluation,
    CandidatePreEvaluation,
    PostEvaluation,
}

impl RawFilterPhase {
    fn into_phase(self) -> FilterPhase {
        match self {
            Self::AxisPreEvaluation => FilterPhase::AxisPreEvaluation,
            Self::CandidatePreEvaluation => FilterPhase::CandidatePreEvaluation,
            Self::PostEvaluation => FilterPhase::PostEvaluation,
        }
    }

    fn from_phase(phase: FilterPhase) -> Self {
        match phase {
            FilterPhase::AxisPreEvaluation => Self::AxisPreEvaluation,
            FilterPhase::CandidatePreEvaluation => Self::CandidatePreEvaluation,
            FilterPhase::PostEvaluation => Self::PostEvaluation,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ExplorationSpecDocument {
    specs: Vec<RawExplorationSpec>,
}

impl ExplorationSpecDocument {
    fn into_specs(
        self,
        testbenches: &[TestbenchSpec],
    ) -> Result<Vec<ExplorationSpec>, ExplorationIoError> {
        let testbench_by_name = testbench_map(testbenches)?;

        self.specs
            .into_iter()
            .map(|spec| spec.into_spec(&testbench_by_name))
            .collect()
    }

    fn from_specs(specs: &[ExplorationSpec]) -> Self {
        Self {
            specs: specs.iter().map(RawExplorationSpec::from_spec).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RawExplorationSpec {
    name: String,
    condition: RawRangeCondition,
    source: RawSpecSource,
    output: RawSpecOutput,
    #[serde(default)]
    parameter_map: Vec<RawSpecParameter>,
    #[serde(default)]
    variables: Vec<RawSpecVariable>,
}

impl RawExplorationSpec {
    fn into_spec(
        self,
        testbench_by_name: &BTreeMap<String, TestbenchSpec>,
    ) -> Result<ExplorationSpec, ExplorationIoError> {
        let mut spec = ExplorationSpec::new(
            self.name,
            self.condition.into_condition(),
            self.source.into_source(testbench_by_name)?,
            self.output.into_output(),
        );

        spec.parameter_map = self
            .parameter_map
            .into_iter()
            .map(RawSpecParameter::into_parameter)
            .collect();
        spec.variables = self
            .variables
            .into_iter()
            .map(RawSpecVariable::into_variable)
            .collect();

        Ok(spec)
    }

    fn from_spec(spec: &ExplorationSpec) -> Self {
        Self {
            name: spec.name.clone(),
            condition: RawRangeCondition::from_condition(spec.condition),
            source: RawSpecSource::from_source(&spec.source),
            output: RawSpecOutput::from_output(&spec.output),
            parameter_map: spec
                .parameter_map
                .iter()
                .map(RawSpecParameter::from_parameter)
                .collect(),
            variables: spec
                .variables
                .iter()
                .map(RawSpecVariable::from_variable)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RawSpecParameter {
    name: String,
    value: String,
}

impl RawSpecParameter {
    fn into_parameter(self) -> SpecParameter {
        SpecParameter::new(self.name, self.value)
    }

    fn from_parameter(parameter: &SpecParameter) -> Self {
        Self {
            name: parameter.name.clone(),
            value: parameter.value.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RawSpecVariable {
    name: String,
    values: Vec<f64>,
}

impl RawSpecVariable {
    fn into_variable(self) -> SpecVariable {
        SpecVariable::new(self.name, self.values)
    }

    fn from_variable(variable: &SpecVariable) -> Self {
        Self {
            name: variable.name.clone(),
            values: variable.values.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct RawRangeCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
}

impl RawRangeCondition {
    fn into_condition(self) -> RangeCondition {
        RangeCondition::new(self.min, self.max)
    }

    fn from_condition(condition: RangeCondition) -> Self {
        Self {
            min: condition.min,
            max: condition.max,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawSpecSource {
    CandidateExpression {
        expression: String,
    },
    TransferFunction {
        testbench: String,
        input: String,
        output: String,
    },
    NodeVoltage {
        testbench: String,
        node: String,
    },
    Composed,
}

impl RawSpecSource {
    fn into_source(
        self,
        testbench_by_name: &BTreeMap<String, TestbenchSpec>,
    ) -> Result<SpecSource, ExplorationIoError> {
        match self {
            Self::CandidateExpression { expression } => {
                Ok(SpecSource::CandidateExpression { expression })
            }
            Self::TransferFunction {
                testbench,
                input,
                output,
            } => {
                let testbench_spec = testbench_by_name
                    .get(&testbench)
                    .cloned()
                    .ok_or(ExplorationIoError::MissingTestbench { name: testbench })?;

                Ok(SpecSource::TransferFunction {
                    testbench: testbench_spec,
                    input,
                    output,
                })
            }
            Self::NodeVoltage { testbench, node } => {
                let testbench_spec = testbench_by_name
                    .get(&testbench)
                    .cloned()
                    .ok_or(ExplorationIoError::MissingTestbench { name: testbench })?;

                Ok(SpecSource::NodeVoltage {
                    testbench: testbench_spec,
                    node,
                })
            }
            Self::Composed => Ok(SpecSource::Composed),
        }
    }

    fn from_source(source: &SpecSource) -> Self {
        match source {
            SpecSource::CandidateExpression { expression } => Self::CandidateExpression {
                expression: expression.clone(),
            },
            SpecSource::TransferFunction {
                testbench,
                input,
                output,
            } => Self::TransferFunction {
                testbench: testbench.name.clone(),
                input: input.clone(),
                output: output.clone(),
            },
            SpecSource::NodeVoltage { testbench, node } => Self::NodeVoltage {
                testbench: testbench.name.clone(),
                node: node.clone(),
            },
            SpecSource::Composed => Self::Composed,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawSpecOutput {
    Eval,
    Diff {
        variable: String,
        values: Vec<f64>,
    },
    Frequency3db {
        sweep: RawFrequencySweep,
    },
    PhaseMargin {
        sweep: RawFrequencySweep,
    },
    Divide {
        numerator: String,
        denominator: String,
    },
}

impl RawSpecOutput {
    fn into_output(self) -> SpecOutput {
        match self {
            Self::Eval => SpecOutput::Eval,
            Self::Diff { variable, values } => SpecOutput::Diff { variable, values },
            Self::Frequency3db { sweep } => SpecOutput::Frequency3db {
                sweep: sweep.into_sweep(),
            },
            Self::PhaseMargin { sweep } => SpecOutput::PhaseMargin {
                sweep: sweep.into_sweep(),
            },
            Self::Divide {
                numerator,
                denominator,
            } => SpecOutput::Divide {
                numerator,
                denominator,
            },
        }
    }

    fn from_output(output: &SpecOutput) -> Self {
        match output {
            SpecOutput::Eval => Self::Eval,
            SpecOutput::Diff { variable, values } => Self::Diff {
                variable: variable.clone(),
                values: values.clone(),
            },
            SpecOutput::Frequency3db { sweep } => Self::Frequency3db {
                sweep: RawFrequencySweep::from_sweep(sweep),
            },
            SpecOutput::PhaseMargin { sweep } => Self::PhaseMargin {
                sweep: RawFrequencySweep::from_sweep(sweep),
            },
            SpecOutput::Divide {
                numerator,
                denominator,
            } => Self::Divide {
                numerator: numerator.clone(),
                denominator: denominator.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct RawFrequencySweep {
    start: f64,
    stop: f64,
    points: usize,
}

impl RawFrequencySweep {
    fn into_sweep(self) -> FrequencySweep {
        FrequencySweep::new(self.start, self.stop, self.points)
    }

    fn from_sweep(sweep: &FrequencySweep) -> Self {
        Self {
            start: sweep.start,
            stop: sweep.stop,
            points: sweep.points,
        }
    }
}

fn testbench_map(
    testbenches: &[TestbenchSpec],
) -> Result<BTreeMap<String, TestbenchSpec>, ExplorationIoError> {
    let mut seen = BTreeSet::new();
    let mut by_name = BTreeMap::new();

    for testbench in testbenches {
        if !seen.insert(testbench.name.clone()) {
            return Err(ExplorationIoError::DuplicateTestbench {
                name: testbench.name.clone(),
            });
        }
        by_name.insert(testbench.name.clone(), testbench.clone());
    }

    Ok(by_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_testbenches_from_json() {
        let dir = make_temp_dir("load_testbenches");
        let path = dir.join("testbenches.json");
        write_file(
            &path,
            r#"{
              "testbenches": [
                {
                  "name": "ota_gain",
                  "dut": { "type": "macro", "name": "ota_1stage" },
                  "view": "small_signal",
                  "elements": [
                    {
                      "type": "voltage_source",
                      "name": "Vin",
                      "nplus": "VINP",
                      "nminus": "VSS",
                      "value": "vin"
                    },
                    {
                      "type": "current_source",
                      "name": "I0",
                      "nplus": "VBIAS",
                      "nminus": "VSS",
                      "value": "0"
                    }
                  ],
                  "extra_body": ".ac dec 10 1 1e9"
                }
              ]
            }"#,
        );

        let testbenches = load_testbenches(&path).unwrap();

        assert_eq!(testbenches.len(), 1);
        assert_eq!(testbenches[0].name, "ota_gain");
        assert_eq!(
            testbenches[0].dut,
            Some(DutRef::Macro("ota_1stage".to_string()))
        );
        assert_eq!(testbenches[0].view, CircuitView::SmallSignal);
        assert_eq!(testbenches[0].elements.len(), 2);
        assert_eq!(
            testbenches[0].body_text(),
            "Vin VINP VSS vin\nI0 VBIAS VSS 0\n.ac dec 10 1 1e9"
        );

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn loads_legacy_testbench_without_dut() {
        let dir = make_temp_dir("load_legacy_testbench_without_dut");
        let path = dir.join("testbenches.json");
        write_file(
            &path,
            r#"{
              "testbenches": [
                {
                  "name": "legacy_gain",
                  "elements": []
                }
              ]
            }"#,
        );

        let testbenches = load_testbenches(&path).unwrap();

        assert_eq!(testbenches[0].name, "legacy_gain");
        assert_eq!(testbenches[0].dut, None);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn saves_and_loads_testbenches_roundtrip() {
        let dir = make_temp_dir("roundtrip_testbenches");
        let path = dir.join("testbenches.json");
        let testbenches = vec![
            TestbenchSpec::new("ota_gain")
                .with_macro_dut("ota_1stage")
                .with_compact_output("bias_current", "isource")
                .with_element(TestbenchElement::VoltageSource {
                    name: "Vin".to_string(),
                    nplus: "VINP".to_string(),
                    nminus: "VSS".to_string(),
                    value: "vin".to_string(),
                }),
        ];

        save_testbenches(&path, &testbenches).unwrap();
        let loaded = load_testbenches(&path).unwrap();

        assert_eq!(loaded, testbenches);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn loads_exploration_specs_with_testbench_reference() {
        let dir = make_temp_dir("load_specs");
        let path = dir.join("specs.json");
        write_file(
            &path,
            r#"{
              "specs": [
                {
                  "name": "gain",
                  "condition": { "min": 40.0 },
                  "source": {
                    "type": "transfer_function",
                    "testbench": "ota_gain",
                    "input": "VINP",
                    "output": "VOUT"
                  },
                  "output": { "type": "eval" },
                  "parameter_map": [
                    { "name": "vin", "value": "1" },
                    { "name": "gm__xdp__m2", "value": "gm__xdp__m1" }
                  ]
                }
              ]
            }"#,
        );
        let testbench =
            TestbenchSpec::new("ota_gain").with_element(TestbenchElement::VoltageSource {
                name: "Vin".to_string(),
                nplus: "VINP".to_string(),
                nminus: "VSS".to_string(),
                value: "vin".to_string(),
            });

        let specs = load_exploration_specs(&path, &[testbench.clone()]).unwrap();

        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].name, "gain");
        assert_eq!(specs[0].condition, RangeCondition::min(40.0));
        assert_eq!(specs[0].parameter_map.len(), 2);
        assert_eq!(
            specs[0].source,
            SpecSource::TransferFunction {
                testbench,
                input: "VINP".to_string(),
                output: "VOUT".to_string(),
            }
        );

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn loads_node_voltage_spec_with_testbench_reference() {
        let dir = make_temp_dir("load_node_voltage_spec");
        let path = dir.join("specs.json");
        write_file(
            &path,
            r#"{
              "specs": [
                {
                  "name": "vout",
                  "condition": { "min": 0.0 },
                  "source": {
                    "type": "node_voltage",
                    "testbench": "ota_gain",
                    "node": "VOUT"
                  },
                  "output": { "type": "eval" }
                }
              ]
            }"#,
        );
        let testbench = TestbenchSpec::new("ota_gain");

        let specs = load_exploration_specs(&path, &[testbench.clone()]).unwrap();

        assert_eq!(
            specs[0].source,
            SpecSource::NodeVoltage {
                testbench,
                node: "VOUT".to_string(),
            }
        );

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn reports_missing_testbench_reference() {
        let dir = make_temp_dir("missing_testbench");
        let path = dir.join("specs.json");
        write_file(
            &path,
            r#"{
              "specs": [
                {
                  "name": "gain",
                  "condition": { "min": 40.0 },
                  "source": {
                    "type": "transfer_function",
                    "testbench": "missing",
                    "input": "VINP",
                    "output": "VOUT"
                  },
                  "output": { "type": "eval" }
                }
              ]
            }"#,
        );

        assert!(matches!(
            load_exploration_specs(&path, &[]),
            Err(ExplorationIoError::MissingTestbench { name }) if name == "missing"
        ));

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn saves_and_loads_exploration_specs_roundtrip() {
        let dir = make_temp_dir("roundtrip_specs");
        let path = dir.join("specs.json");
        let testbench = TestbenchSpec::new("ota_gain");
        let mut spec = ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::TransferFunction {
                testbench: testbench.clone(),
                input: "VINP".to_string(),
                output: "VOUT".to_string(),
            },
            SpecOutput::Eval,
        );
        spec.parameter_map = vec![SpecParameter::new("vin", "1")];

        save_exploration_specs(&path, &[spec.clone()]).unwrap();
        let loaded = load_exploration_specs(&path, &[testbench]).unwrap();

        assert_eq!(loaded, vec![spec]);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn loads_ota_1stage_example_json_files() {
        let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/exploration");
        let testbenches =
            load_testbenches(&examples_dir.join("ota_1stage_testbenches.json")).unwrap();
        let specs =
            load_exploration_specs(&examples_dir.join("ota_1stage_specs.json"), &testbenches)
                .unwrap();

        assert_eq!(testbenches.len(), 2);
        assert_eq!(specs.len(), 2);
        assert_eq!(specs[0].name, "gain_1stage");
        assert_eq!(specs[1].name, "rout_1stage");
        assert!(matches!(
            specs[0].source,
            SpecSource::TransferFunction { .. }
        ));
    }

    #[test]
    fn loads_exploration_candidates_from_json() {
        let dir = make_temp_dir("load_candidates");
        let path = dir.join("candidates.json");
        write_file(
            &path,
            r#"{
              "axes": [
                { "name": "vdd", "values": [1.7, 1.8] }
              ],
              "sets": [
                {
                  "name": "diffpair",
                  "points": [
                    { "xdp.gm": 0.001, "xdp.ro": 100000.0, "xdp.vs": 0.3 },
                    { "xdp.gm": 0.002, "xdp.ro": 80000.0, "xdp.vs": 0.4 }
                  ]
                }
              ],
              "filters": [
                {
                  "type": "range",
                  "phase": "axis_pre_evaluation",
                  "column": "vdd",
                  "condition": { "min": 1.75 }
                },
                {
                  "type": "equal_columns",
                  "phase": "candidate_pre_evaluation",
                  "columns": ["xdp.vs", "xcs.vs"]
                }
              ]
            }"#,
        );

        let input = load_exploration_candidates(&path).unwrap();

        assert_eq!(input.axes.len(), 1);
        assert_eq!(input.axes[0], CandidateAxis::new("vdd", vec![1.7, 1.8]));
        assert_eq!(input.sets.len(), 1);
        assert_eq!(input.sets[0].points.len(), 2);
        assert_eq!(input.sets[0].points[0].get("xdp.gm"), Some(0.001));
        assert_eq!(input.filters.len(), 2);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn saves_and_loads_exploration_candidates_roundtrip() {
        let dir = make_temp_dir("roundtrip_candidates");
        let path = dir.join("candidates.json");
        let input = ExplorationCandidateInput {
            axes: vec![CandidateAxis::new("vdd", vec![1.8])],
            sets: vec![CandidateSet::new(
                "diffpair",
                vec![CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), 0.001),
                    ("xdp.ro".to_string(), 100000.0),
                ])],
            )],
            filters: vec![ExplorationFilter::new(
                FilterPhase::AxisPreEvaluation,
                "vdd",
                RangeCondition::min(1.0),
            )],
        };

        save_exploration_candidates(&path, &input).unwrap();
        let loaded = load_exploration_candidates(&path).unwrap();

        assert_eq!(loaded, input);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn loads_ota_1stage_candidate_example_json_file() {
        let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/exploration");
        let input =
            load_exploration_candidates(&examples_dir.join("ota_1stage_candidates.json")).unwrap();

        assert!(input.axes.is_empty());
        assert_eq!(input.sets.len(), 1);
        assert_eq!(input.sets[0].name, "ota_1stage_manual_lut_points");
        assert_eq!(input.sets[0].points.len(), 3);
        assert_eq!(input.sets[0].points[0].get("gm__xdp__m1"), Some(0.00026688));
    }

    fn make_temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "libsstadex_exploration_io_{name}_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_file(path: &Path, content: &str) {
        fs::write(path, content).unwrap();
    }
}
