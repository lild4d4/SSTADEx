use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::exploration::candidate::{
    CandidateSet, CandidateSetBuildError, candidate_set_from_prefixed_columns,
};
use crate::exploration::table::ExplorationColumn;
use crate::primitive::manifest::PrimitiveManifest;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimitiveBuildSpec {
    #[serde(default)]
    pub inputs: Vec<PrimitiveBuildInputSpec>,
    #[serde(default)]
    pub sweep_mode: SweepMode,
    #[serde(default)]
    pub derived: Vec<BuildExpression>,
    #[serde(default)]
    pub lut: Vec<LutBuildSpec>,
    #[serde(default)]
    pub columns: Vec<BuildExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveBuildInputSpec {
    pub name: String,
    pub kind: PrimitiveBuildInputKind,
    #[serde(default)]
    pub required: bool,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrimitiveBuildInputKind {
    Scalar,
    Vector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SweepMode {
    #[default]
    Aligned,
    Cartesian,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildExpression {
    pub name: String,
    pub expr: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LutBuildSpec {
    pub name: String,
    pub device: String,
    #[serde(default)]
    pub dof: HashMap<String, String>,
    pub lengths: Option<LutLengths>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LutLengths {
    Values(Vec<f64>),
    Ref(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveBuildValue {
    Scalar(f64),
    Vector(Vec<f64>),
}

impl PrimitiveBuildValue {
    fn values(&self) -> &[f64] {
        match self {
            Self::Scalar(value) => std::slice::from_ref(value),
            Self::Vector(values) => values,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrimitiveBuildInput {
    pub values: HashMap<String, PrimitiveBuildValue>,
    pub lut_config: Option<serde_json::Value>,
}

impl PrimitiveBuildInput {
    pub fn new(values: HashMap<String, PrimitiveBuildValue>) -> Self {
        Self {
            values,
            lut_config: None,
        }
    }

    pub fn with_lut_config(mut self, lut_config: serde_json::Value) -> Self {
        self.lut_config = Some(lut_config);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveBuildOutput {
    pub columns: Vec<ExplorationColumn>,
    pub row_count: usize,
}

impl PrimitiveBuildOutput {
    pub fn to_candidate_set(
        &self,
        instance_name: &str,
    ) -> Result<CandidateSet, CandidateSetBuildError> {
        candidate_set_from_prefixed_columns(instance_name, instance_name, &self.columns)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveBuildError {
    MissingBuildSpec {
        primitive: String,
    },
    MissingRequiredInput {
        input: String,
    },
    EmptyVector {
        input: String,
    },
    AlignedLengthMismatch {
        input: String,
        expected: usize,
        actual: usize,
    },
    MissingSymbol {
        symbol: String,
    },
    Expression {
        name: String,
        expression: String,
        reason: String,
    },
    CyclicExpression {
        names: Vec<String>,
    },
    MissingLutLengths {
        lut: String,
    },
    InvalidLutLengthsRef {
        lut: String,
        reference: String,
    },
    Lut {
        lut: String,
        reason: String,
    },
    CandidateSet(CandidateSetBuildError),
}

pub trait LutBackend {
    fn query(&self, request: &LutQuery) -> Result<HashMap<String, f64>, String>;

    fn query_many(&self, requests: &[LutQuery]) -> Result<Vec<HashMap<String, f64>>, String> {
        requests.iter().map(|request| self.query(request)).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LutQuery {
    pub lut_name: String,
    pub device: String,
    pub dof: HashMap<String, f64>,
    pub length: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DeterministicLutBackend;

impl LutBackend for DeterministicLutBackend {
    fn query(&self, request: &LutQuery) -> Result<HashMap<String, f64>, String> {
        let vds = request.dof.get("vds").copied().unwrap_or(0.0);
        let vgs = request.dof.get("vgs").copied().unwrap_or(0.0);
        let magnitude = vds.abs() + vgs.abs() + request.length * 1.0e6;
        let id = (1.0 + magnitude).max(1.0e-12);
        let gds = 0.01 * id + request.length;
        let mut values = HashMap::new();
        values.insert("length".to_string(), request.length);
        values.insert("id".to_string(), id);
        values.insert("jd".to_string(), id * 1.0e6);
        values.insert("gmid".to_string(), 10.0 + magnitude);
        values.insert("gds".to_string(), gds);
        values.insert("cgg".to_string(), 1.0e-15 * (1.0 + magnitude));
        values.insert("cgs".to_string(), 0.6e-15 * (1.0 + magnitude));
        values.insert("cgd".to_string(), 0.2e-15 * (1.0 + magnitude));
        values.insert("vdsat".to_string(), 0.1 + 0.01 * magnitude);
        Ok(values)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonGmidLutBackend {
    python: PathBuf,
    helper_script: PathBuf,
    lut_files: HashMap<String, PathBuf>,
    timing_output: bool,
}

impl PythonGmidLutBackend {
    pub fn new(
        python: impl Into<PathBuf>,
        helper_script: impl Into<PathBuf>,
        lut_files: HashMap<String, PathBuf>,
    ) -> Self {
        Self {
            python: python.into(),
            helper_script: helper_script.into(),
            lut_files,
            timing_output: false,
        }
    }

    pub fn with_default_helper(
        python: impl Into<PathBuf>,
        lut_files: HashMap<String, PathBuf>,
    ) -> Self {
        Self::new(python, default_python_gmid_helper(), lut_files)
    }

    pub fn python(&self) -> &Path {
        &self.python
    }

    pub fn helper_script(&self) -> &Path {
        &self.helper_script
    }

    pub fn lut_files(&self) -> &HashMap<String, PathBuf> {
        &self.lut_files
    }

    pub fn with_timing_output(mut self, enabled: bool) -> Self {
        self.timing_output = enabled;
        self
    }

    pub fn timing_output(&self) -> bool {
        self.timing_output
    }
}

impl LutBackend for PythonGmidLutBackend {
    fn query(&self, request: &LutQuery) -> Result<HashMap<String, f64>, String> {
        self.query_many(std::slice::from_ref(request))?
            .into_iter()
            .next()
            .ok_or_else(|| "python gmid backend returned no rows".to_string())
    }

    fn query_many(&self, requests: &[LutQuery]) -> Result<Vec<HashMap<String, f64>>, String> {
        if requests.is_empty() {
            return Ok(Vec::new());
        }

        validate_python_gmid_backend_paths(self)?;
        let started_at = Instant::now();
        let request = PythonGmidRequest::from_backend(self, requests);
        let request_json = serde_json::to_vec(&request)
            .map_err(|error| format!("failed to serialize gmid request: {error}"))?;
        let mut child = Command::new(&self.python)
            .arg(&self.helper_script)
            .env("MPLCONFIGDIR", std::env::temp_dir())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| {
                format!(
                    "failed to spawn python gmid backend '{}': {error}",
                    self.python.display()
                )
            })?;

        child
            .stdin
            .as_mut()
            .ok_or_else(|| "failed to open python gmid backend stdin".to_string())?
            .write_all(&request_json)
            .map_err(|error| format!("failed to write gmid request: {error}"))?;

        let output = child
            .wait_with_output()
            .map_err(|error| format!("failed to wait for python gmid backend: {error}"))?;

        if !output.status.success() {
            return Err(format!(
                "python gmid backend failed with status {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }

        let response: PythonGmidResponse = serde_json::from_slice(&output.stdout)
            .map_err(|error| format!("failed to parse gmid response JSON: {error}"))?;

        if response.results.len() != requests.len() {
            return Err(format!(
                "python gmid backend returned {} rows for {} requests",
                response.results.len(),
                requests.len()
            ));
        }

        if self.timing_output {
            eprintln!(
                "PythonGmidLutBackend resolved {} LUT queries in {:.3} ms",
                requests.len(),
                started_at.elapsed().as_secs_f64() * 1000.0
            );
        }

        Ok(response.results)
    }
}

#[derive(Debug, Serialize)]
struct PythonGmidRequest {
    lut_files: HashMap<String, String>,
    queries: Vec<PythonGmidQuery>,
}

impl PythonGmidRequest {
    fn from_backend(backend: &PythonGmidLutBackend, requests: &[LutQuery]) -> Self {
        Self {
            lut_files: backend
                .lut_files
                .iter()
                .map(|(device, path)| (device.clone(), path.display().to_string()))
                .collect(),
            queries: requests
                .iter()
                .map(|request| PythonGmidQuery {
                    lut_name: request.lut_name.clone(),
                    device: request.device.clone(),
                    dof: request.dof.clone(),
                    length: request.length,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
struct PythonGmidQuery {
    lut_name: String,
    device: String,
    dof: HashMap<String, f64>,
    length: f64,
}

#[derive(Debug, Deserialize)]
struct PythonGmidResponse {
    results: Vec<HashMap<String, f64>>,
}

fn default_python_gmid_helper() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("python/gmid_lut_backend.py")
}

fn validate_python_gmid_backend_paths(backend: &PythonGmidLutBackend) -> Result<(), String> {
    if !is_path_search_executable(&backend.python) && !backend.python.exists() {
        return Err(format!(
            "python executable does not exist: {}",
            backend.python.display()
        ));
    }

    if !backend.helper_script.exists() {
        return Err(format!(
            "python gmid helper does not exist: {}",
            backend.helper_script.display()
        ));
    }

    if backend.lut_files.is_empty() {
        return Err("python gmid backend has no LUT files configured".to_string());
    }

    for (device, path) in &backend.lut_files {
        if !path.exists() {
            return Err(format!(
                "LUT file for device '{device}' does not exist: {}",
                path.display()
            ));
        }
    }

    Ok(())
}

fn is_path_search_executable(path: &Path) -> bool {
    path.file_name().is_some() && path.components().count() == 1
}

pub struct PrimitiveBuildEngine<B> {
    lut_backend: B,
}

impl<B: LutBackend> PrimitiveBuildEngine<B> {
    pub fn new(lut_backend: B) -> Self {
        Self { lut_backend }
    }

    pub fn build(
        &self,
        spec: &PrimitiveBuildSpec,
        input: &PrimitiveBuildInput,
    ) -> Result<PrimitiveBuildOutput, PrimitiveBuildError> {
        validate_required_inputs(spec, input)?;
        validate_no_expression_cycles(&spec.derived)?;
        validate_no_expression_cycles(&spec.columns)?;

        let mut rows = expand_inputs(spec, input)?;
        evaluate_expressions(&mut rows, &spec.derived)?;
        apply_luts(&mut rows, spec, input, &self.lut_backend)?;
        evaluate_expressions(&mut rows, &spec.columns)?;

        let columns = spec
            .columns
            .iter()
            .map(|column| {
                let values = rows
                    .iter()
                    .map(|row| {
                        row.get(&column.name).copied().ok_or_else(|| {
                            PrimitiveBuildError::MissingSymbol {
                                symbol: column.name.clone(),
                            }
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ExplorationColumn {
                    name: column.name.clone(),
                    values,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PrimitiveBuildOutput {
            row_count: rows.len(),
            columns,
        })
    }

    pub fn build_candidate_set_for_primitive(
        &self,
        primitive: &PrimitiveManifest,
        instance_name: &str,
        input: &PrimitiveBuildInput,
    ) -> Result<CandidateSet, PrimitiveBuildError> {
        let build_spec =
            primitive
                .build
                .as_ref()
                .ok_or_else(|| PrimitiveBuildError::MissingBuildSpec {
                    primitive: primitive.name.clone(),
                })?;
        let mut input = input.clone();

        if input.lut_config.is_none() {
            input.lut_config = primitive.lut_config.clone();
        }

        self.build(build_spec, &input)?
            .to_candidate_set(instance_name)
            .map_err(PrimitiveBuildError::CandidateSet)
    }
}

fn validate_required_inputs(
    spec: &PrimitiveBuildSpec,
    input: &PrimitiveBuildInput,
) -> Result<(), PrimitiveBuildError> {
    for input_spec in &spec.inputs {
        if input_spec.required && !input.values.contains_key(&input_spec.name) {
            return Err(PrimitiveBuildError::MissingRequiredInput {
                input: input_spec.name.clone(),
            });
        }
    }
    Ok(())
}

fn expand_inputs(
    spec: &PrimitiveBuildSpec,
    input: &PrimitiveBuildInput,
) -> Result<Vec<HashMap<String, f64>>, PrimitiveBuildError> {
    let active_inputs = spec
        .inputs
        .iter()
        .filter_map(|input_spec| {
            input
                .values
                .get(&input_spec.name)
                .map(|value| (&input_spec.name, value))
        })
        .collect::<Vec<_>>();

    for (name, value) in &active_inputs {
        if value.values().is_empty() {
            return Err(PrimitiveBuildError::EmptyVector {
                input: (*name).clone(),
            });
        }
    }

    match spec.sweep_mode {
        SweepMode::Aligned => expand_aligned(active_inputs),
        SweepMode::Cartesian => expand_cartesian(active_inputs),
    }
}

fn expand_aligned(
    inputs: Vec<(&String, &PrimitiveBuildValue)>,
) -> Result<Vec<HashMap<String, f64>>, PrimitiveBuildError> {
    let row_count = inputs
        .iter()
        .filter_map(|(_, value)| match value {
            PrimitiveBuildValue::Vector(values) => Some(values.len()),
            PrimitiveBuildValue::Scalar(_) => None,
        })
        .max()
        .unwrap_or(1);

    for (name, value) in &inputs {
        if let PrimitiveBuildValue::Vector(values) = value {
            if values.len() != row_count {
                return Err(PrimitiveBuildError::AlignedLengthMismatch {
                    input: (*name).clone(),
                    expected: row_count,
                    actual: values.len(),
                });
            }
        }
    }

    let mut rows = Vec::with_capacity(row_count);
    for row_idx in 0..row_count {
        let mut row = HashMap::new();
        for (name, value) in &inputs {
            let value = match value {
                PrimitiveBuildValue::Scalar(value) => *value,
                PrimitiveBuildValue::Vector(values) => values[row_idx],
            };
            row.insert((*name).clone(), value);
        }
        rows.push(row);
    }
    Ok(rows)
}

fn expand_cartesian(
    inputs: Vec<(&String, &PrimitiveBuildValue)>,
) -> Result<Vec<HashMap<String, f64>>, PrimitiveBuildError> {
    let mut rows = vec![HashMap::new()];

    for (name, value) in inputs {
        let mut next_rows = Vec::new();
        for row in &rows {
            for item in value.values() {
                let mut next_row = row.clone();
                next_row.insert(name.clone(), *item);
                next_rows.push(next_row);
            }
        }
        rows = next_rows;
    }

    Ok(rows)
}

fn evaluate_expressions(
    rows: &mut [HashMap<String, f64>],
    expressions: &[BuildExpression],
) -> Result<(), PrimitiveBuildError> {
    for expression in expressions {
        for row in rows.iter_mut() {
            let value = eval_expression(&expression.expr, row).map_err(|reason| {
                PrimitiveBuildError::Expression {
                    name: expression.name.clone(),
                    expression: expression.expr.clone(),
                    reason,
                }
            })?;
            row.insert(expression.name.clone(), value);
        }
    }
    Ok(())
}

fn apply_luts<B: LutBackend>(
    rows: &mut Vec<HashMap<String, f64>>,
    spec: &PrimitiveBuildSpec,
    input: &PrimitiveBuildInput,
    backend: &B,
) -> Result<(), PrimitiveBuildError> {
    for lut in &spec.lut {
        let lengths = resolve_lut_lengths(lut, input)?;
        let mut query_rows = Vec::with_capacity(rows.len() * lengths.len());
        let mut queries = Vec::with_capacity(rows.len() * lengths.len());

        for row in rows.iter() {
            for length in &lengths {
                let mut dof = HashMap::new();
                for (dof_name, expr) in &lut.dof {
                    let value = eval_expression(expr, row).map_err(|reason| {
                        PrimitiveBuildError::Expression {
                            name: format!("lut.{}.{}", lut.name, dof_name),
                            expression: expr.clone(),
                            reason,
                        }
                    })?;
                    dof.insert(dof_name.clone(), value);
                }

                queries.push(LutQuery {
                    lut_name: lut.name.clone(),
                    device: lut.device.clone(),
                    dof,
                    length: *length,
                });
                query_rows.push((row.clone(), *length));
            }
        }

        let lut_results =
            backend
                .query_many(&queries)
                .map_err(|reason| PrimitiveBuildError::Lut {
                    lut: lut.name.clone(),
                    reason,
                })?;
        if lut_results.len() != query_rows.len() {
            return Err(PrimitiveBuildError::Lut {
                lut: lut.name.clone(),
                reason: format!(
                    "backend returned {} rows for {} queries",
                    lut_results.len(),
                    query_rows.len()
                ),
            });
        }

        let mut next_rows = Vec::with_capacity(query_rows.len());
        for ((row, length), lut_values) in query_rows.into_iter().zip(lut_results) {
            let mut next_row = row;
            next_row.insert(format!("lut.{}.length", lut.name), length);
            for (key, value) in lut_values {
                next_row.insert(format!("lut.{}.{}", lut.name, key), value);
            }
            next_rows.push(next_row);
        }

        *rows = next_rows;
    }
    Ok(())
}

fn resolve_lut_lengths(
    lut: &LutBuildSpec,
    input: &PrimitiveBuildInput,
) -> Result<Vec<f64>, PrimitiveBuildError> {
    match &lut.lengths {
        Some(LutLengths::Values(values)) if !values.is_empty() => Ok(values.clone()),
        Some(LutLengths::Values(_)) => Err(PrimitiveBuildError::MissingLutLengths {
            lut: lut.name.clone(),
        }),
        Some(LutLengths::Ref(reference)) if reference == "$lut_config.lengths" => input
            .lut_config
            .as_ref()
            .and_then(|config| config.get("lengths"))
            .and_then(|lengths| lengths.as_array())
            .map(|lengths| {
                lengths
                    .iter()
                    .filter_map(serde_json::Value::as_f64)
                    .collect::<Vec<_>>()
            })
            .filter(|lengths| !lengths.is_empty())
            .ok_or_else(|| PrimitiveBuildError::InvalidLutLengthsRef {
                lut: lut.name.clone(),
                reference: reference.clone(),
            }),
        Some(LutLengths::Ref(reference)) => Err(PrimitiveBuildError::InvalidLutLengthsRef {
            lut: lut.name.clone(),
            reference: reference.clone(),
        }),
        None => Err(PrimitiveBuildError::MissingLutLengths {
            lut: lut.name.clone(),
        }),
    }
}

fn validate_no_expression_cycles(
    expressions: &[BuildExpression],
) -> Result<(), PrimitiveBuildError> {
    let targets = expressions
        .iter()
        .map(|expression| expression.name.clone())
        .collect::<HashSet<_>>();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for expression in expressions {
        let deps = expression
            .dependencies()
            .into_iter()
            .filter(|dependency| targets.contains(dependency))
            .collect::<Vec<_>>();
        graph.insert(expression.name.clone(), deps);
    }

    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    for name in graph.keys() {
        let mut stack = Vec::new();
        if has_cycle(name, &graph, &mut visiting, &mut visited, &mut stack) {
            return Err(PrimitiveBuildError::CyclicExpression { names: stack });
        }
    }

    Ok(())
}

impl BuildExpression {
    fn dependencies(&self) -> Vec<String> {
        collect_identifiers(&self.expr)
    }
}

fn has_cycle(
    name: &str,
    graph: &HashMap<String, Vec<String>>,
    visiting: &mut HashSet<String>,
    visited: &mut HashSet<String>,
    stack: &mut Vec<String>,
) -> bool {
    if visited.contains(name) {
        return false;
    }
    if !visiting.insert(name.to_string()) {
        stack.push(name.to_string());
        return true;
    }

    stack.push(name.to_string());
    for dependency in graph.get(name).into_iter().flatten() {
        if has_cycle(dependency, graph, visiting, visited, stack) {
            return true;
        }
    }
    stack.pop();
    visiting.remove(name);
    visited.insert(name.to_string());
    false
}

fn eval_expression(expression: &str, symbols: &HashMap<String, f64>) -> Result<f64, String> {
    ExpressionParser::new(expression, symbols).parse()
}

struct ExpressionParser<'a> {
    expression: &'a str,
    bytes: &'a [u8],
    pos: usize,
    symbols: &'a HashMap<String, f64>,
}

impl<'a> ExpressionParser<'a> {
    fn new(expression: &'a str, symbols: &'a HashMap<String, f64>) -> Self {
        Self {
            expression,
            bytes: expression.as_bytes(),
            pos: 0,
            symbols,
        }
    }

    fn parse(mut self) -> Result<f64, String> {
        let value = self.parse_expression()?;
        self.skip_whitespace();
        if self.pos != self.bytes.len() {
            return Err(format!("unexpected token at byte {}", self.pos));
        }
        Ok(value)
    }

    fn parse_expression(&mut self) -> Result<f64, String> {
        let mut value = self.parse_term()?;
        loop {
            self.skip_whitespace();
            if self.consume(b'+') {
                value += self.parse_term()?;
            } else if self.consume(b'-') {
                value -= self.parse_term()?;
            } else {
                break;
            }
        }
        Ok(value)
    }

    fn parse_term(&mut self) -> Result<f64, String> {
        let mut value = self.parse_factor()?;
        loop {
            self.skip_whitespace();
            if self.consume(b'*') {
                value *= self.parse_factor()?;
            } else if self.consume(b'/') {
                value /= self.parse_factor()?;
            } else {
                break;
            }
        }
        Ok(value)
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        self.skip_whitespace();
        if self.consume(b'-') {
            return Ok(-self.parse_factor()?);
        }
        if self.consume(b'+') {
            return self.parse_factor();
        }
        if self.consume(b'(') {
            let value = self.parse_expression()?;
            self.skip_whitespace();
            if !self.consume(b')') {
                return Err("missing closing ')'".to_string());
            }
            return Ok(value);
        }
        if self.peek().is_some_and(is_number_start) {
            return self.parse_number();
        }
        if self.peek().is_some_and(is_identifier_start) {
            return self.parse_symbol();
        }
        Err(format!("expected factor at byte {}", self.pos))
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            self.pos += 1;
        }
        if self.consume(b'.') {
            while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let exponent_start = self.pos;
            while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                self.pos += 1;
            }
            if self.pos == exponent_start {
                return Err("missing exponent digits".to_string());
            }
        }
        self.expression[start..self.pos]
            .parse::<f64>()
            .map_err(|error| format!("invalid number: {error}"))
    }

    fn parse_symbol(&mut self) -> Result<f64, String> {
        let start = self.pos;
        while self.peek().is_some_and(is_identifier_body) {
            self.pos += 1;
        }
        let symbol = &self.expression[start..self.pos];
        self.symbols
            .get(symbol)
            .copied()
            .ok_or_else(|| format!("missing symbol '{symbol}'"))
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_whitespace()) {
            self.pos += 1;
        }
    }

    fn consume(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }
}

fn collect_identifiers(expression: &str) -> Vec<String> {
    let bytes = expression.as_bytes();
    let mut pos = 0;
    let mut identifiers = Vec::new();

    while let Some(byte) = bytes.get(pos).copied() {
        if is_identifier_start(byte) {
            let start = pos;
            pos += 1;
            while bytes.get(pos).copied().is_some_and(is_identifier_body) {
                pos += 1;
            }
            identifiers.push(expression[start..pos].to_string());
        } else {
            pos += 1;
        }
    }

    identifiers
}

fn is_number_start(byte: u8) -> bool {
    byte.is_ascii_digit() || byte == b'.'
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_body(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::manifest::{Pin, PinRole, PrimitiveFiles, PrimitiveShape, PrimitiveUi};

    #[test]
    fn aligned_sweep_keeps_rows_together() {
        let spec = PrimitiveBuildSpec {
            inputs: vec![
                input("bias", PrimitiveBuildInputKind::Scalar),
                input("vin", PrimitiveBuildInputKind::Vector),
                input("vout", PrimitiveBuildInputKind::Vector),
            ],
            sweep_mode: SweepMode::Aligned,
            derived: vec![expr("delta", "vout - vin")],
            lut: Vec::new(),
            columns: vec![expr("gm", "bias * delta")],
        };
        let input = PrimitiveBuildInput::new(HashMap::from([
            ("bias".to_string(), PrimitiveBuildValue::Scalar(2.0)),
            (
                "vin".to_string(),
                PrimitiveBuildValue::Vector(vec![0.1, 0.2]),
            ),
            (
                "vout".to_string(),
                PrimitiveBuildValue::Vector(vec![1.1, 1.4]),
            ),
        ]));

        let output = PrimitiveBuildEngine::new(DeterministicLutBackend)
            .build(&spec, &input)
            .unwrap();

        assert_eq!(output.row_count, 2);
        assert_eq!(output.columns[0].values, vec![2.0, 2.4]);
    }

    #[test]
    fn cartesian_sweep_generates_internal_product() {
        let spec = PrimitiveBuildSpec {
            inputs: vec![
                input("a", PrimitiveBuildInputKind::Vector),
                input("b", PrimitiveBuildInputKind::Vector),
            ],
            sweep_mode: SweepMode::Cartesian,
            derived: Vec::new(),
            lut: Vec::new(),
            columns: vec![expr("sum", "a + b")],
        };
        let input = PrimitiveBuildInput::new(HashMap::from([
            ("a".to_string(), PrimitiveBuildValue::Vector(vec![1.0, 2.0])),
            (
                "b".to_string(),
                PrimitiveBuildValue::Vector(vec![10.0, 20.0]),
            ),
        ]));

        let output = PrimitiveBuildEngine::new(DeterministicLutBackend)
            .build(&spec, &input)
            .unwrap();

        assert_eq!(output.columns[0].values, vec![11.0, 21.0, 12.0, 22.0]);
    }

    #[test]
    fn aligned_sweep_rejects_mismatched_vector_lengths() {
        let spec = PrimitiveBuildSpec {
            inputs: vec![
                input("a", PrimitiveBuildInputKind::Vector),
                input("b", PrimitiveBuildInputKind::Vector),
            ],
            sweep_mode: SweepMode::Aligned,
            derived: Vec::new(),
            lut: Vec::new(),
            columns: vec![expr("sum", "a + b")],
        };
        let input = PrimitiveBuildInput::new(HashMap::from([
            ("a".to_string(), PrimitiveBuildValue::Vector(vec![1.0])),
            ("b".to_string(), PrimitiveBuildValue::Vector(vec![1.0, 2.0])),
        ]));

        assert!(matches!(
            PrimitiveBuildEngine::new(DeterministicLutBackend).build(&spec, &input),
            Err(PrimitiveBuildError::AlignedLengthMismatch { .. })
        ));
    }

    #[test]
    fn expression_engine_reports_missing_symbols() {
        let spec = PrimitiveBuildSpec {
            inputs: vec![input("a", PrimitiveBuildInputKind::Scalar)],
            sweep_mode: SweepMode::Aligned,
            derived: Vec::new(),
            lut: Vec::new(),
            columns: vec![expr("sum", "a + missing")],
        };
        let input = PrimitiveBuildInput::new(HashMap::from([(
            "a".to_string(),
            PrimitiveBuildValue::Scalar(1.0),
        )]));

        assert!(matches!(
            PrimitiveBuildEngine::new(DeterministicLutBackend).build(&spec, &input),
            Err(PrimitiveBuildError::Expression { reason, .. })
                if reason.contains("missing symbol")
        ));
    }

    #[test]
    fn expression_cycle_is_reported() {
        let spec = PrimitiveBuildSpec {
            inputs: vec![input("a", PrimitiveBuildInputKind::Scalar)],
            sweep_mode: SweepMode::Aligned,
            derived: vec![expr("x", "y + 1"), expr("y", "x + 1")],
            lut: Vec::new(),
            columns: vec![expr("out", "x")],
        };
        let input = PrimitiveBuildInput::new(HashMap::from([(
            "a".to_string(),
            PrimitiveBuildValue::Scalar(1.0),
        )]));

        assert!(matches!(
            PrimitiveBuildEngine::new(DeterministicLutBackend).build(&spec, &input),
            Err(PrimitiveBuildError::CyclicExpression { .. })
        ));
    }

    #[test]
    fn deterministic_lut_values_are_available_to_columns() {
        let spec = PrimitiveBuildSpec {
            inputs: vec![
                input("id_m1", PrimitiveBuildInputKind::Scalar),
                input("vds_m1", PrimitiveBuildInputKind::Scalar),
                input("vgs_m1", PrimitiveBuildInputKind::Scalar),
            ],
            sweep_mode: SweepMode::Aligned,
            derived: Vec::new(),
            lut: vec![LutBuildSpec {
                name: "m1".to_string(),
                device: "nmos".to_string(),
                dof: HashMap::from([
                    ("vds".to_string(), "vds_m1".to_string()),
                    ("vgs".to_string(), "vgs_m1".to_string()),
                ]),
                lengths: Some(LutLengths::Values(vec![1.0e-6, 2.0e-6])),
            }],
            columns: vec![
                expr("length", "lut.m1.length"),
                expr("gm__m1", "lut.m1.gmid * id_m1"),
            ],
        };
        let input = PrimitiveBuildInput::new(HashMap::from([
            ("id_m1".to_string(), PrimitiveBuildValue::Scalar(1.0e-6)),
            ("vds_m1".to_string(), PrimitiveBuildValue::Scalar(0.8)),
            ("vgs_m1".to_string(), PrimitiveBuildValue::Scalar(0.7)),
        ]));

        let output = PrimitiveBuildEngine::new(DeterministicLutBackend)
            .build(&spec, &input)
            .unwrap();

        assert_eq!(output.row_count, 2);
        assert_eq!(output.columns[0].values, vec![1.0e-6, 2.0e-6]);
        assert_eq!(output.columns[1].values.len(), 2);
    }

    #[test]
    fn engine_uses_batch_lut_backend() {
        #[derive(Debug, Clone, Copy)]
        struct BatchOnlyBackend;

        impl LutBackend for BatchOnlyBackend {
            fn query(&self, _request: &LutQuery) -> Result<HashMap<String, f64>, String> {
                panic!("engine should call query_many for LUT batches");
            }

            fn query_many(
                &self,
                requests: &[LutQuery],
            ) -> Result<Vec<HashMap<String, f64>>, String> {
                Ok(requests
                    .iter()
                    .map(|request| {
                        HashMap::from([
                            ("length".to_string(), request.length),
                            ("id".to_string(), 1.0),
                            ("gmid".to_string(), 2.0 + request.length * 1.0e6),
                        ])
                    })
                    .collect())
            }
        }

        let spec = PrimitiveBuildSpec {
            inputs: vec![input("id_m1", PrimitiveBuildInputKind::Scalar)],
            sweep_mode: SweepMode::Aligned,
            derived: Vec::new(),
            lut: vec![LutBuildSpec {
                name: "m1".to_string(),
                device: "nmos".to_string(),
                dof: HashMap::from([
                    ("vds".to_string(), "0.8".to_string()),
                    ("vgs".to_string(), "0.7".to_string()),
                ]),
                lengths: Some(LutLengths::Values(vec![1.0e-6, 2.0e-6])),
            }],
            columns: vec![expr("gm", "lut.m1.gmid * id_m1")],
        };
        let input = PrimitiveBuildInput::new(HashMap::from([(
            "id_m1".to_string(),
            PrimitiveBuildValue::Scalar(1.0e-6),
        )]));

        let output = PrimitiveBuildEngine::new(BatchOnlyBackend)
            .build(&spec, &input)
            .unwrap();

        assert_eq!(output.columns[0].values, vec![3.0e-6, 4.0e-6]);
    }

    #[test]
    fn python_gmid_backend_allows_python_from_path() {
        assert!(is_path_search_executable(Path::new("python")));
        assert!(!is_path_search_executable(Path::new(".venv/bin/python")));

        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let lut_files = HashMap::from([(
            "nmos".to_string(),
            workspace_root.join("LUTs/ihp-sg13g2/lv_5w_nmos.npz"),
        )]);
        let backend = PythonGmidLutBackend::with_default_helper("python", lut_files);

        if backend.lut_files()["nmos"].exists() {
            assert!(validate_python_gmid_backend_paths(&backend).is_ok());
        }
    }

    #[test]
    fn python_gmid_backend_queries_real_npz_lut_when_available() {
        let Some((python, lut_files)) = python_gmid_test_paths() else {
            return;
        };
        let backend = PythonGmidLutBackend::with_default_helper(python, lut_files);
        let queries = vec![
            LutQuery {
                lut_name: "m1".to_string(),
                device: "nmos".to_string(),
                dof: HashMap::from([("vds".to_string(), 0.75), ("vgs".to_string(), 0.65)]),
                length: 4.0e-7,
            },
            LutQuery {
                lut_name: "m1".to_string(),
                device: "nmos".to_string(),
                dof: HashMap::from([("vds".to_string(), 0.85), ("vgs".to_string(), 0.55)]),
                length: 4.0e-7,
            },
        ];

        let results = backend.query_many(&queries).unwrap();
        assert_eq!(results.len(), 2);

        for key in [
            "length", "id", "jd", "gmid", "gds", "cgg", "cgs", "cgd", "vdsat",
        ] {
            for values in &results {
                assert!(
                    values.get(key).is_some_and(|value| value.is_finite()),
                    "missing or non-finite {key}: {values:?}"
                );
            }
        }
        assert!(results[0]["jd"] > 0.0);
        assert!(results[0]["gmid"] > 0.0);
        assert_ne!(results[0]["gmid"], results[1]["gmid"]);
    }

    #[test]
    fn output_converts_to_prefixed_candidate_set() {
        let output = PrimitiveBuildOutput {
            row_count: 2,
            columns: vec![ExplorationColumn::new("gm__m1", vec![1.0, 2.0])],
        };

        let candidates = output.to_candidate_set("xdp").unwrap();

        assert_eq!(candidates.name, "xdp");
        assert_eq!(candidates.points[0].get("xdp.gm__m1"), Some(1.0));
        assert_eq!(candidates.points[1].get("xdp.gm__m1"), Some(2.0));
    }

    #[test]
    fn engine_builds_candidate_set_from_primitive_manifest() {
        let primitive = primitive_with_build(PrimitiveBuildSpec {
            inputs: vec![input("current", PrimitiveBuildInputKind::Scalar)],
            sweep_mode: SweepMode::Aligned,
            derived: Vec::new(),
            lut: Vec::new(),
            columns: vec![expr("gm", "current * 10")],
        });
        let input = PrimitiveBuildInput::new(HashMap::from([(
            "current".to_string(),
            PrimitiveBuildValue::Scalar(2.0),
        )]));

        let candidates = PrimitiveBuildEngine::new(DeterministicLutBackend)
            .build_candidate_set_for_primitive(&primitive, "x1", &input)
            .unwrap();

        assert_eq!(candidates.name, "x1");
        assert_eq!(candidates.points[0].get("x1.gm"), Some(20.0));
    }

    #[test]
    fn engine_reports_missing_manifest_build_spec() {
        let mut primitive = primitive_with_build(PrimitiveBuildSpec {
            inputs: Vec::new(),
            sweep_mode: SweepMode::Aligned,
            derived: Vec::new(),
            lut: Vec::new(),
            columns: Vec::new(),
        });
        primitive.build = None;

        let error = PrimitiveBuildEngine::new(DeterministicLutBackend)
            .build_candidate_set_for_primitive(&primitive, "x1", &PrimitiveBuildInput::default())
            .unwrap_err();

        assert_eq!(
            error,
            PrimitiveBuildError::MissingBuildSpec {
                primitive: "primitive".to_string()
            }
        );
    }

    fn input(name: &str, kind: PrimitiveBuildInputKind) -> PrimitiveBuildInputSpec {
        PrimitiveBuildInputSpec {
            name: name.to_string(),
            kind,
            required: true,
            source: None,
        }
    }

    fn expr(name: &str, expression: &str) -> BuildExpression {
        BuildExpression {
            name: name.to_string(),
            expr: expression.to_string(),
        }
    }

    fn primitive_with_build(build: PrimitiveBuildSpec) -> PrimitiveManifest {
        PrimitiveManifest {
            name: "primitive".to_string(),
            version: "1.0".to_string(),
            description: None,
            subckt_name: "primitive".to_string(),
            pins: vec![Pin {
                name: "IN".to_string(),
                role: PinRole::Input,
            }],
            files: PrimitiveFiles {
                netlist: "netlist/netlist.spice".to_string(),
                build: None,
                symbol: None,
            },
            ui: PrimitiveUi {
                shape: PrimitiveShape::Box,
                symbol: None,
            },
            small_signal: None,
            transistor_type: None,
            layout_params: None,
            lut_config: None,
            build: Some(build),
        }
    }

    fn python_gmid_test_paths() -> Option<(PathBuf, HashMap<String, PathBuf>)> {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent()?;
        let python = workspace_root.join(".venv-sstadex/bin/python");
        let nmos_lut = workspace_root.join("LUTs/ihp-sg13g2/lv_5w_nmos.npz");
        let pmos_lut = workspace_root.join("LUTs/ihp-sg13g2/lv_5w_pmos.npz");

        if !(python.exists() && nmos_lut.exists() && pmos_lut.exists()) {
            return None;
        }

        Some((
            python,
            HashMap::from([
                ("nmos".to_string(), nmos_lut),
                ("pmos".to_string(), pmos_lut),
            ]),
        ))
    }
}
