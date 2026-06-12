use super::conditions::RangeCondition;

#[derive(Debug, Clone, PartialEq)]
pub struct ExplorationSpec {
    pub name: String,
    pub condition: RangeCondition,
    pub source: SpecSource,
    pub output: SpecOutput,
    pub parameter_map: Vec<SpecParameter>,
    pub variables: Vec<SpecVariable>,
}

impl ExplorationSpec {
    pub fn new(
        name: impl Into<String>,
        condition: RangeCondition,
        source: SpecSource,
        output: SpecOutput,
    ) -> Self {
        Self {
            name: name.into(),
            condition,
            source,
            output,
            parameter_map: Vec::new(),
            variables: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecSource {
    CandidateExpression {
        expression: String,
    },
    TransferFunction {
        testbench: TestbenchSpec,
        input: String,
        output: String,
    },
    Composed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecOutput {
    Eval,
    Diff {
        variable: String,
        values: Vec<f64>,
    },
    Frequency3db {
        sweep: FrequencySweep,
    },
    PhaseMargin {
        sweep: FrequencySweep,
    },
    Divide {
        numerator: String,
        denominator: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrequencySweep {
    pub start: f64,
    pub stop: f64,
    pub points: usize,
}

impl FrequencySweep {
    pub fn new(start: f64, stop: f64, points: usize) -> Self {
        Self {
            start,
            stop,
            points,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecParameter {
    pub name: String,
    pub value: String,
}

impl SpecParameter {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecVariable {
    pub name: String,
    pub values: Vec<f64>,
}

impl SpecVariable {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            values,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitView {
    SmallSignal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestbenchSpec {
    pub name: String,
    pub view: CircuitView,
    pub elements: Vec<TestbenchElement>,
    pub extra_body: Option<String>,
}

impl TestbenchSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            view: CircuitView::SmallSignal,
            elements: Vec::new(),
            extra_body: None,
        }
    }

    pub fn with_element(mut self, element: TestbenchElement) -> Self {
        self.elements.push(element);
        self
    }

    pub fn with_extra_body(mut self, extra_body: impl Into<String>) -> Self {
        self.extra_body = Some(extra_body.into());
        self
    }

    pub fn body_lines(&self) -> Vec<String> {
        self.elements
            .iter()
            .map(TestbenchElement::to_spice)
            .collect()
    }

    pub fn body_text(&self) -> String {
        let mut body = self.body_lines();

        if let Some(extra_body) = &self.extra_body {
            if !extra_body.trim().is_empty() {
                body.push(extra_body.trim().to_string());
            }
        }

        body.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestbenchElement {
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

impl TestbenchElement {
    pub fn to_spice(&self) -> String {
        match self {
            Self::VoltageSource {
                name,
                nplus,
                nminus,
                value,
            }
            | Self::CurrentSource {
                name,
                nplus,
                nminus,
                value,
            } => format!("{name} {nplus} {nminus} {value}"),
            Self::Resistor {
                name,
                n1,
                n2,
                value,
            }
            | Self::Capacitor {
                name,
                n1,
                n2,
                value,
            } => format!("{name} {n1} {n2} {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_eval_transfer_function_spec_with_testbench() {
        let testbench = TestbenchSpec::new("ota_gain")
            .with_element(TestbenchElement::VoltageSource {
                name: "Vin".to_string(),
                nplus: "VIN".to_string(),
                nminus: "0".to_string(),
                value: "ac 1".to_string(),
            })
            .with_extra_body(".ac dec 10 1 1e9");
        let spec = ExplorationSpec::new(
            "gain",
            RangeCondition::min(40.0),
            SpecSource::TransferFunction {
                testbench: testbench.clone(),
                input: "VIN".to_string(),
                output: "VOUT".to_string(),
            },
            SpecOutput::Eval,
        );

        assert_eq!(spec.name, "gain");
        assert_eq!(spec.condition, RangeCondition::min(40.0));
        assert_eq!(
            spec.source,
            SpecSource::TransferFunction {
                testbench,
                input: "VIN".to_string(),
                output: "VOUT".to_string(),
            }
        );
        assert_eq!(spec.output, SpecOutput::Eval);
    }

    #[test]
    fn creates_frequency_3db_spec() {
        let spec = ExplorationSpec::new(
            "gbw",
            RangeCondition::min(1e6),
            SpecSource::CandidateExpression {
                expression: "vout/vin".to_string(),
            },
            SpecOutput::Frequency3db {
                sweep: FrequencySweep::new(1.0, 1e9, 200),
            },
        );

        assert_eq!(
            spec.output,
            SpecOutput::Frequency3db {
                sweep: FrequencySweep::new(1.0, 1e9, 200),
            }
        );
    }

    #[test]
    fn creates_composed_divide_spec() {
        let spec = ExplorationSpec::new(
            "efficiency",
            RangeCondition::min(1.0),
            SpecSource::Composed,
            SpecOutput::Divide {
                numerator: "gain".to_string(),
                denominator: "power".to_string(),
            },
        );

        assert_eq!(spec.source, SpecSource::Composed);
        assert_eq!(
            spec.output,
            SpecOutput::Divide {
                numerator: "gain".to_string(),
                denominator: "power".to_string(),
            }
        );
    }

    #[test]
    fn creates_testbench_with_elements() {
        let testbench = TestbenchSpec::new("pm")
            .with_element(TestbenchElement::CurrentSource {
                name: "Ibias".to_string(),
                nplus: "VDD".to_string(),
                nminus: "IBIAS".to_string(),
                value: "10u".to_string(),
            })
            .with_element(TestbenchElement::Capacitor {
                name: "Cload".to_string(),
                n1: "VOUT".to_string(),
                n2: "0".to_string(),
                value: "1p".to_string(),
            });

        assert_eq!(testbench.name, "pm");
        assert_eq!(testbench.view, CircuitView::SmallSignal);
        assert_eq!(testbench.elements.len(), 2);
        assert_eq!(testbench.extra_body, None);
    }

    #[test]
    fn renders_testbench_elements_to_spice() {
        assert_eq!(
            TestbenchElement::VoltageSource {
                name: "Vin".to_string(),
                nplus: "VIN".to_string(),
                nminus: "0".to_string(),
                value: "ac 1".to_string(),
            }
            .to_spice(),
            "Vin VIN 0 ac 1"
        );
        assert_eq!(
            TestbenchElement::CurrentSource {
                name: "Ibias".to_string(),
                nplus: "VDD".to_string(),
                nminus: "IBIAS".to_string(),
                value: "10u".to_string(),
            }
            .to_spice(),
            "Ibias VDD IBIAS 10u"
        );
        assert_eq!(
            TestbenchElement::Resistor {
                name: "Rload".to_string(),
                n1: "VOUT".to_string(),
                n2: "0".to_string(),
                value: "1k".to_string(),
            }
            .to_spice(),
            "Rload VOUT 0 1k"
        );
        assert_eq!(
            TestbenchElement::Capacitor {
                name: "Cload".to_string(),
                n1: "VOUT".to_string(),
                n2: "0".to_string(),
                value: "1p".to_string(),
            }
            .to_spice(),
            "Cload VOUT 0 1p"
        );
    }

    #[test]
    fn renders_testbench_body_lines() {
        let testbench = TestbenchSpec::new("gain")
            .with_element(TestbenchElement::VoltageSource {
                name: "Vin".to_string(),
                nplus: "VIN".to_string(),
                nminus: "0".to_string(),
                value: "ac 1".to_string(),
            })
            .with_element(TestbenchElement::Resistor {
                name: "Rload".to_string(),
                n1: "VOUT".to_string(),
                n2: "0".to_string(),
                value: "1k".to_string(),
            });

        assert_eq!(
            testbench.body_lines(),
            vec![
                "Vin VIN 0 ac 1".to_string(),
                "Rload VOUT 0 1k".to_string(),
            ]
        );
        assert_eq!(testbench.body_text(), "Vin VIN 0 ac 1\nRload VOUT 0 1k");
    }

    #[test]
    fn renders_testbench_body_with_extra_body() {
        let testbench = TestbenchSpec::new("gain")
            .with_element(TestbenchElement::VoltageSource {
                name: "Vin".to_string(),
                nplus: "VIN".to_string(),
                nminus: "0".to_string(),
                value: "ac 1".to_string(),
            })
            .with_extra_body("\n.ac dec 10 1 1e9\n");

        assert_eq!(testbench.body_text(), "Vin VIN 0 ac 1\n.ac dec 10 1 1e9");
    }
}
