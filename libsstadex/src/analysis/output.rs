use serde::Serialize;

use crate::analysis::CircuitMnaAnalysis;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CircuitMnaOutput {
    pub spice_path: String,
    pub cir_path: String,
    pub reports: CircuitMnaReportsOutput,
    pub nodes: Vec<NodeOutput>,
    pub variables: Vec<NodeVariableOutput>,
    pub equations: Vec<EquationOutput>,
    pub solution: Option<Vec<SolutionOutput>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CircuitMnaReportsOutput {
    pub spice_parser: String,
    pub netlist: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NodeOutput {
    pub name: String,
    pub number: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NodeVariableOutput {
    pub variable: String,
    pub node_name: String,
    pub node_number: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EquationOutput {
    pub index: usize,
    pub lhs: String,
    pub rhs: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SolutionOutput {
    pub variable: String,
    pub expression: String,
}

impl CircuitMnaOutput {
    pub fn from_analysis(analysis: &CircuitMnaAnalysis) -> Self {
        let mut nodes = analysis
            .mna
            .nodes
            .nodes
            .iter()
            .map(|(name, number)| NodeOutput {
                name: name.clone(),
                number: *number,
            })
            .collect::<Vec<_>>();
        nodes.sort_by_key(|node| node.number);

        let variables = analysis
            .mna
            .node_variables()
            .into_iter()
            .map(|node| NodeVariableOutput {
                variable: node.variable,
                node_name: node.node_name,
                node_number: node.node_number,
            })
            .collect();

        let solution = analysis.solution.as_ref().map(|solution| {
            let mut entries = solution
                .solutions
                .iter()
                .map(|(variable, expression)| SolutionOutput {
                    variable: variable.clone(),
                    expression: expression.clone(),
                })
                .collect::<Vec<_>>();
            entries.sort_by(|left, right| left.variable.cmp(&right.variable));
            entries
        });

        Self {
            spice_path: analysis.spice_path.display().to_string(),
            cir_path: analysis.cir_path.display().to_string(),
            reports: CircuitMnaReportsOutput {
                spice_parser: analysis.mna.nodes.to_string(),
                netlist: analysis.mna.report.clone(),
            },
            nodes,
            variables,
            equations: mna_equations(&analysis.mna.a, &analysis.mna.x, &analysis.mna.z),
            solution,
        }
    }
}

fn mna_equations<T>(a: &[Vec<T>], x: &[T], z: &[T]) -> Vec<EquationOutput>
where
    T: std::fmt::Display,
{
    a.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let lhs = equation_lhs(row, x);
            let rhs = z
                .get(row_idx)
                .map(|expr| expr.to_string())
                .unwrap_or_else(|| "<missing rhs>".to_string());
            let index = row_idx + 1;

            EquationOutput {
                index,
                text: format!("eq_{index}: {lhs} = {rhs}"),
                lhs,
                rhs,
            }
        })
        .collect()
}

fn equation_lhs<T>(row: &[T], x: &[T]) -> String
where
    T: std::fmt::Display,
{
    let terms = row
        .iter()
        .enumerate()
        .filter_map(|(col_idx, coeff)| {
            let var = x.get(col_idx)?;
            let coeff_str = coeff.to_string();

            if coeff_str == "0" || coeff_str == "0.0" {
                None
            } else if coeff_str == "1" || coeff_str == "1.0" {
                Some(format!("{var}"))
            } else if coeff_str == "-1" || coeff_str == "-1.0" {
                Some(format!("-{var}"))
            } else {
                Some(format!("({coeff})·{var}"))
            }
        })
        .collect::<Vec<_>>();

    if terms.is_empty() {
        "0".to_string()
    } else {
        terms.join(" + ")
    }
}
