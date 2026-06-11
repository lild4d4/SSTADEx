use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::mna::spice_parser::{NodeMap, SpiceError, spice_parser};
use crate::mna::symmna::{Matrix, SmnaError, Vector, smna};

#[derive(Debug)]
pub struct MnaResult {
    pub report: String,
    pub a: Matrix,
    pub x: Vector,
    pub z: Vector,
    pub nodes: NodeMap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MnaSolveResult {
    pub solutions: HashMap<String, String>,
}

#[derive(Debug)]
pub enum MnaError {
    Io(std::io::Error),
    SpiceConversion(SpiceError),
    SymMna(SmnaError),
    PythonSolve(String),
    MissingNode(String),
}

impl From<std::io::Error> for MnaError {
    fn from(error: std::io::Error) -> Self {
        MnaError::Io(error)
    }
}

pub fn mna(spice_dir: &Path, output_dir: &Path, design_name: &str) -> Result<MnaResult, MnaError> {
    println!("Running MNA...");

    let nodes =
        spice_parser(spice_dir, output_dir, design_name).map_err(MnaError::SpiceConversion)?;
    let input_path = output_dir.join(format!("{design_name}.cir"));
    let content = fs::read_to_string(input_path)?;

    let symmna_output = smna(&content).map_err(MnaError::SymMna)?;

    Ok(MnaResult {
        report: symmna_output.report,
        a: symmna_output.a,
        x: symmna_output.x,
        z: symmna_output.z,
        nodes,
    })
}

pub fn mna_solve(a: &Matrix, x: &Vector, z: &Vector) -> Result<MnaSolveResult, MnaError> {
    let output = Command::new("python3")
        .arg("-c")
        .arg(SYMPY_SOLVE_SCRIPT)
        .arg(matrix_to_python_literal(a))
        .arg(vector_to_python_literal(x))
        .arg(vector_to_python_literal(z))
        .output()?;

    if !output.status.success() {
        return Err(MnaError::PythonSolve(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut solutions = HashMap::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let Some((name, value)) = line.split_once('\t') else {
            return Err(MnaError::PythonSolve(format!(
                "invalid SymPy solver output line: {line}"
            )));
        };

        solutions.insert(name.to_string(), value.to_string());
    }

    Ok(MnaSolveResult { solutions })
}

fn matrix_to_python_literal(matrix: &Matrix) -> String {
    let rows = matrix
        .iter()
        .map(|row| {
            let values = row
                .iter()
                .map(|expr| python_string_literal(&expr.to_string()))
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{values}]")
        })
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{rows}]")
}

fn vector_to_python_literal(vector: &Vector) -> String {
    let values = vector
        .iter()
        .map(|expr| python_string_literal(&expr.to_string()))
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{values}]")
}

fn python_string_literal(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('\'', "\\'");

    format!("'{escaped}'")
}

const SYMPY_SOLVE_SCRIPT: &str = r#"
import ast
import sys
import sympy as sym

a_literal = ast.literal_eval(sys.argv[1])
x_literal = ast.literal_eval(sys.argv[2])
z_literal = ast.literal_eval(sys.argv[3])

A = sym.Matrix([
    [sym.sympify(item) for item in row]
    for row in a_literal
])
X = sym.Matrix([sym.sympify(item) for item in x_literal])
Z = sym.Matrix([sym.sympify(item) for item in z_literal])

equations = list(A * X - Z)
solutions = sym.solve(equations, list(X), dict=True)

if solutions:
    for key, value in solutions[0].items():
        print(f"{key}\t{value}")
"#;
