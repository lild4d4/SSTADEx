use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use libsstadex::analysis::{analyze_circuit_mna, CircuitMnaAnalysisError, CircuitMnaOutput};
use libsstadex::catalog::{load_primitive_catalog, PrimitiveLoadError};
use libsstadex::circuit::{load_circuit, Circuit, CircuitIoError, Connection, Instance, PinRef};
use libsstadex::exploration::{
    load_exploration_candidates, load_exploration_specs, load_testbenches,
    prepare_candidate_expression_spec, prepare_transfer_function_spec,
    run_prepared_expression_flow, ExplorationIoError, ExplorationTable, PreparedSpec,
    PreparedSpecSource, SpecPrepareError, SpecSource,
};
use libsstadex::mna::mna::{mna, mna_solve, MnaError};
use libsstadex::mna::pretty::{pretty_solutions, pretty_system};
use libsstadex::netlist::{
    render_circuit_netlist, render_small_signal_netlist, render_testbench_small_signal_netlist,
    NetlistRenderError, SmallSignalRenderError,
};

#[derive(Debug)]
struct CatalogListArgs {
    primitives_dir: PathBuf,
}

#[derive(Debug)]
struct CatalogShowArgs {
    primitive: String,
    primitives_dir: PathBuf,
}

#[derive(Debug)]
struct CircuitRenderArgs {
    primitives_dir: PathBuf,
    name: String,
    instances: Vec<Instance>,
    connections: Vec<Connection>,
    view: CircuitView,
    output: Option<PathBuf>,
}

#[derive(Debug)]
struct CircuitRenderFileArgs {
    primitives_dir: PathBuf,
    circuit: PathBuf,
    view: CircuitView,
    output: Option<PathBuf>,
}

#[derive(Debug)]
struct CircuitMnaArgs {
    primitives_dir: PathBuf,
    circuit: PathBuf,
    output: PathBuf,
    solve: bool,
    format: CircuitMnaFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitMnaFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitView {
    Structural,
    SmallSignal,
}

#[derive(Debug)]
struct MnaArgs {
    spice_dir: PathBuf,
    design: String,
    output: PathBuf,
    solve: bool,
}

#[derive(Debug)]
struct ExplorationValidateArgs {
    primitives_dir: PathBuf,
    circuit: PathBuf,
    testbenches: PathBuf,
    specs: PathBuf,
    candidates: Option<PathBuf>,
    format: ExplorationValidateFormat,
}

#[derive(Debug)]
struct ExplorationPrepareArgs {
    primitives_dir: PathBuf,
    circuit: PathBuf,
    testbenches: PathBuf,
    specs: PathBuf,
    output: PathBuf,
    format: ExplorationPrepareFormat,
}

#[derive(Debug)]
struct ExplorationRunArgs {
    primitives_dir: PathBuf,
    circuit: PathBuf,
    testbenches: PathBuf,
    specs: PathBuf,
    candidates: PathBuf,
    work_dir: PathBuf,
    format: ExplorationRunFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExplorationValidateFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExplorationPrepareFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExplorationRunFormat {
    Text,
    Json,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) if is_help_requested_error(&error) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn is_help_requested_error(error: &str) -> bool {
    error.ends_with("help requested")
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("--help") | Some("-h") => {
            print_help();
            Ok(())
        }
        Some("catalog") => run_catalog(args.collect()),
        Some("circuit") => run_circuit(args.collect()),
        Some("exploration") => run_exploration(args.collect()),
        Some("mna") => run_mna(parse_mna_args(args.collect())?),
        Some(command) => Err(format!(
            "unknown command '{command}'\n\nRun `sstadex --help` for usage."
        )),
    }
}

fn run_exploration(args: Vec<String>) -> Result<(), String> {
    let mut args = args.into_iter();

    match args.next().as_deref() {
        Some("prepare") => run_exploration_prepare(parse_exploration_prepare_args(args.collect())?),
        Some("run") => run_exploration_run(parse_exploration_run_args(args.collect())?),
        Some("validate") => {
            run_exploration_validate(parse_exploration_validate_args(args.collect())?)
        }
        Some("--help") | Some("-h") | None => {
            print_exploration_help();
            Ok(())
        }
        Some(command) => Err(format!(
            "unknown exploration command '{command}'\n\nRun `sstadex exploration --help` for usage."
        )),
    }
}

fn run_circuit(args: Vec<String>) -> Result<(), String> {
    let mut args = args.into_iter();

    match args.next().as_deref() {
        Some("render") => run_circuit_render(parse_circuit_render_args(args.collect())?),
        Some("render-file") => {
            run_circuit_render_file(parse_circuit_render_file_args(args.collect())?)
        }
        Some("mna") => run_circuit_mna(parse_circuit_mna_args(args.collect())?),
        Some("--help") | Some("-h") | None => {
            print_circuit_help();
            Ok(())
        }
        Some(command) => Err(format!(
            "unknown circuit command '{command}'\n\nRun `sstadex circuit --help` for usage."
        )),
    }
}

fn parse_circuit_mna_args(args: Vec<String>) -> Result<CircuitMnaArgs, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_circuit_mna_help();
        return Err("circuit mna help requested".to_string());
    }

    let mut primitives_dir = None;
    let mut circuit = None;
    let mut output = None;
    let mut solve = false;
    let mut format = CircuitMnaFormat::Text;

    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--primitives-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--primitives-dir requires a value".to_string())?;
                primitives_dir = Some(PathBuf::from(value));
            }
            "--circuit" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--circuit requires a value".to_string())?;
                circuit = Some(PathBuf::from(value));
            }
            "--output" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--output requires a value".to_string())?;
                output = Some(PathBuf::from(value));
            }
            "--solve" => {
                solve = true;
            }
            "--format" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--format requires a value".to_string())?;
                format = parse_circuit_mna_format(value)?;
            }
            flag => {
                return Err(format!(
                    "unknown circuit mna option '{flag}'\n\nRun `sstadex circuit mna --help` for usage."
                ));
            }
        }

        idx += 1;
    }

    Ok(CircuitMnaArgs {
        primitives_dir: primitives_dir
            .ok_or_else(|| "missing required option --primitives-dir".to_string())?,
        circuit: circuit.ok_or_else(|| "missing required option --circuit".to_string())?,
        output: output.ok_or_else(|| "missing required option --output".to_string())?,
        solve,
        format,
    })
}

fn parse_circuit_mna_format(value: &str) -> Result<CircuitMnaFormat, String> {
    match value {
        "text" => Ok(CircuitMnaFormat::Text),
        "json" => Ok(CircuitMnaFormat::Json),
        _ => Err(format!(
            "unknown circuit mna format '{value}'. Expected: text, json"
        )),
    }
}

fn parse_exploration_validate_args(args: Vec<String>) -> Result<ExplorationValidateArgs, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_exploration_validate_help();
        return Err("exploration validate help requested".to_string());
    }

    let mut primitives_dir = None;
    let mut circuit = None;
    let mut testbenches = None;
    let mut specs = None;
    let mut candidates = None;
    let mut format = ExplorationValidateFormat::Text;

    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--primitives-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--primitives-dir requires a value".to_string())?;
                primitives_dir = Some(PathBuf::from(value));
            }
            "--circuit" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--circuit requires a value".to_string())?;
                circuit = Some(PathBuf::from(value));
            }
            "--testbenches" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--testbenches requires a value".to_string())?;
                testbenches = Some(PathBuf::from(value));
            }
            "--specs" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--specs requires a value".to_string())?;
                specs = Some(PathBuf::from(value));
            }
            "--candidates" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--candidates requires a value".to_string())?;
                candidates = Some(PathBuf::from(value));
            }
            "--format" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--format requires a value".to_string())?;
                format = parse_exploration_validate_format(value)?;
            }
            flag => {
                return Err(format!(
                    "unknown exploration validate option '{flag}'\n\nRun `sstadex exploration validate --help` for usage."
                ));
            }
        }

        idx += 1;
    }

    Ok(ExplorationValidateArgs {
        primitives_dir: primitives_dir
            .ok_or_else(|| "missing required option --primitives-dir".to_string())?,
        circuit: circuit.ok_or_else(|| "missing required option --circuit".to_string())?,
        testbenches: testbenches
            .ok_or_else(|| "missing required option --testbenches".to_string())?,
        specs: specs.ok_or_else(|| "missing required option --specs".to_string())?,
        candidates,
        format,
    })
}

fn parse_exploration_validate_format(value: &str) -> Result<ExplorationValidateFormat, String> {
    match value {
        "text" => Ok(ExplorationValidateFormat::Text),
        "json" => Ok(ExplorationValidateFormat::Json),
        _ => Err(format!(
            "unknown exploration validate format '{value}'. Expected: text, json"
        )),
    }
}

fn parse_exploration_prepare_args(args: Vec<String>) -> Result<ExplorationPrepareArgs, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_exploration_prepare_help();
        return Err("exploration prepare help requested".to_string());
    }

    let mut primitives_dir = None;
    let mut circuit = None;
    let mut testbenches = None;
    let mut specs = None;
    let mut output = None;
    let mut format = ExplorationPrepareFormat::Text;

    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--primitives-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--primitives-dir requires a value".to_string())?;
                primitives_dir = Some(PathBuf::from(value));
            }
            "--circuit" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--circuit requires a value".to_string())?;
                circuit = Some(PathBuf::from(value));
            }
            "--testbenches" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--testbenches requires a value".to_string())?;
                testbenches = Some(PathBuf::from(value));
            }
            "--specs" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--specs requires a value".to_string())?;
                specs = Some(PathBuf::from(value));
            }
            "--output" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--output requires a value".to_string())?;
                output = Some(PathBuf::from(value));
            }
            "--format" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--format requires a value".to_string())?;
                format = parse_exploration_prepare_format(value)?;
            }
            flag => {
                return Err(format!(
                    "unknown exploration prepare option '{flag}'\n\nRun `sstadex exploration prepare --help` for usage."
                ));
            }
        }

        idx += 1;
    }

    Ok(ExplorationPrepareArgs {
        primitives_dir: primitives_dir
            .ok_or_else(|| "missing required option --primitives-dir".to_string())?,
        circuit: circuit.ok_or_else(|| "missing required option --circuit".to_string())?,
        testbenches: testbenches
            .ok_or_else(|| "missing required option --testbenches".to_string())?,
        specs: specs.ok_or_else(|| "missing required option --specs".to_string())?,
        output: output.ok_or_else(|| "missing required option --output".to_string())?,
        format,
    })
}

fn parse_exploration_prepare_format(value: &str) -> Result<ExplorationPrepareFormat, String> {
    match value {
        "text" => Ok(ExplorationPrepareFormat::Text),
        "json" => Ok(ExplorationPrepareFormat::Json),
        _ => Err(format!(
            "unknown exploration prepare format '{value}'. Expected: text, json"
        )),
    }
}

fn parse_exploration_run_args(args: Vec<String>) -> Result<ExplorationRunArgs, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_exploration_run_help();
        return Err("exploration run help requested".to_string());
    }

    let mut primitives_dir = None;
    let mut circuit = None;
    let mut testbenches = None;
    let mut specs = None;
    let mut candidates = None;
    let mut work_dir = None;
    let mut format = ExplorationRunFormat::Text;

    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--primitives-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--primitives-dir requires a value".to_string())?;
                primitives_dir = Some(PathBuf::from(value));
            }
            "--circuit" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--circuit requires a value".to_string())?;
                circuit = Some(PathBuf::from(value));
            }
            "--testbenches" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--testbenches requires a value".to_string())?;
                testbenches = Some(PathBuf::from(value));
            }
            "--specs" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--specs requires a value".to_string())?;
                specs = Some(PathBuf::from(value));
            }
            "--candidates" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--candidates requires a value".to_string())?;
                candidates = Some(PathBuf::from(value));
            }
            "--work-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--work-dir requires a value".to_string())?;
                work_dir = Some(PathBuf::from(value));
            }
            "--format" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--format requires a value".to_string())?;
                format = parse_exploration_run_format(value)?;
            }
            flag => {
                return Err(format!(
                    "unknown exploration run option '{flag}'\n\nRun `sstadex exploration run --help` for usage."
                ));
            }
        }

        idx += 1;
    }

    Ok(ExplorationRunArgs {
        primitives_dir: primitives_dir
            .ok_or_else(|| "missing required option --primitives-dir".to_string())?,
        circuit: circuit.ok_or_else(|| "missing required option --circuit".to_string())?,
        testbenches: testbenches
            .ok_or_else(|| "missing required option --testbenches".to_string())?,
        specs: specs.ok_or_else(|| "missing required option --specs".to_string())?,
        candidates: candidates.ok_or_else(|| "missing required option --candidates".to_string())?,
        work_dir: work_dir.ok_or_else(|| "missing required option --work-dir".to_string())?,
        format,
    })
}

fn parse_exploration_run_format(value: &str) -> Result<ExplorationRunFormat, String> {
    match value {
        "text" => Ok(ExplorationRunFormat::Text),
        "json" => Ok(ExplorationRunFormat::Json),
        _ => Err(format!(
            "unknown exploration run format '{value}'. Expected: text, json"
        )),
    }
}

fn parse_circuit_render_file_args(args: Vec<String>) -> Result<CircuitRenderFileArgs, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_circuit_render_file_help();
        return Err("circuit render-file help requested".to_string());
    }

    let mut primitives_dir = None;
    let mut circuit = None;
    let mut view = CircuitView::Structural;
    let mut output = None;

    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--primitives-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--primitives-dir requires a value".to_string())?;
                primitives_dir = Some(PathBuf::from(value));
            }
            "--circuit" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--circuit requires a value".to_string())?;
                circuit = Some(PathBuf::from(value));
            }
            "--view" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--view requires a value".to_string())?;
                view = parse_circuit_view(value)?;
            }
            "--output" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--output requires a value".to_string())?;
                output = Some(PathBuf::from(value));
            }
            flag => {
                return Err(format!(
                    "unknown circuit render-file option '{flag}'\n\nRun `sstadex circuit render-file --help` for usage."
                ));
            }
        }

        idx += 1;
    }

    Ok(CircuitRenderFileArgs {
        primitives_dir: primitives_dir
            .ok_or_else(|| "missing required option --primitives-dir".to_string())?,
        circuit: circuit.ok_or_else(|| "missing required option --circuit".to_string())?,
        view,
        output,
    })
}

fn parse_circuit_render_args(args: Vec<String>) -> Result<CircuitRenderArgs, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_circuit_render_help();
        return Err("circuit render help requested".to_string());
    }

    let mut primitives_dir = None;
    let mut name = None;
    let mut instances = Vec::new();
    let mut connections = Vec::new();
    let mut view = CircuitView::Structural;
    let mut output = None;

    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--primitives-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--primitives-dir requires a value".to_string())?;
                primitives_dir = Some(PathBuf::from(value));
            }
            "--name" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--name requires a value".to_string())?;
                name = Some(value.clone());
            }
            "--instance" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--instance requires a value".to_string())?;
                instances.push(parse_instance(value)?);
            }
            "--connect" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--connect requires a value".to_string())?;
                connections.push(parse_connection(value)?);
            }
            "--view" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--view requires a value".to_string())?;
                view = parse_circuit_view(value)?;
            }
            "--output" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--output requires a value".to_string())?;
                output = Some(PathBuf::from(value));
            }
            flag => {
                return Err(format!(
                    "unknown circuit render option '{flag}'\n\nRun `sstadex circuit render --help` for usage."
                ));
            }
        }

        idx += 1;
    }

    if instances.is_empty() {
        return Err("at least one --instance is required".to_string());
    }

    Ok(CircuitRenderArgs {
        primitives_dir: primitives_dir
            .ok_or_else(|| "missing required option --primitives-dir".to_string())?,
        name: name.ok_or_else(|| "missing required option --name".to_string())?,
        instances,
        connections,
        view,
        output,
    })
}

fn parse_circuit_view(value: &str) -> Result<CircuitView, String> {
    match value {
        "structural" => Ok(CircuitView::Structural),
        "small-signal" => Ok(CircuitView::SmallSignal),
        _ => Err(format!(
            "unknown circuit view '{value}'. Expected: structural, small-signal"
        )),
    }
}

fn parse_instance(value: &str) -> Result<Instance, String> {
    let Some((id, primitive)) = value.split_once(':') else {
        return Err(format!(
            "invalid --instance '{value}', expected format <ID>:<PRIMITIVE>"
        ));
    };

    if id.is_empty() || primitive.is_empty() {
        return Err(format!(
            "invalid --instance '{value}', instance id and primitive are required"
        ));
    }

    Ok(Instance::new(id, primitive))
}

fn parse_connection(value: &str) -> Result<Connection, String> {
    let Some((pin_ref, net)) = value.split_once('=') else {
        return Err(format!(
            "invalid --connect '{value}', expected format <INSTANCE>.<PIN>=<NET>"
        ));
    };
    let Some((instance, pin)) = pin_ref.split_once('.') else {
        return Err(format!(
            "invalid --connect '{value}', expected format <INSTANCE>.<PIN>=<NET>"
        ));
    };

    if instance.is_empty() || pin.is_empty() || net.is_empty() {
        return Err(format!(
            "invalid --connect '{value}', instance, pin, and net are required"
        ));
    }

    Ok(Connection::new(PinRef::new(instance, pin), net))
}

fn run_circuit_render(args: CircuitRenderArgs) -> Result<(), String> {
    let catalog =
        load_primitive_catalog(&args.primitives_dir).map_err(format_primitive_load_error)?;

    let mut circuit = Circuit::new(args.name);
    for instance in args.instances {
        circuit.add_instance(instance);
    }
    for connection in args.connections {
        circuit.connect(connection);
    }

    let netlist = render_circuit_view(&circuit, &catalog, args.view)?;
    emit_output(&netlist, args.output.as_ref())
}

fn run_circuit_render_file(args: CircuitRenderFileArgs) -> Result<(), String> {
    let catalog =
        load_primitive_catalog(&args.primitives_dir).map_err(format_primitive_load_error)?;
    let circuit = load_circuit(&args.circuit).map_err(format_circuit_io_error)?;
    let netlist = render_circuit_view(&circuit, &catalog, args.view)?;

    emit_output(&netlist, args.output.as_ref())
}

fn run_circuit_mna(args: CircuitMnaArgs) -> Result<(), String> {
    let catalog =
        load_primitive_catalog(&args.primitives_dir).map_err(format_primitive_load_error)?;
    let circuit = load_circuit(&args.circuit).map_err(format_circuit_io_error)?;

    if args.format == CircuitMnaFormat::Text {
        println!("SSTADEx Circuit MNA");
        println!();
        println!("Running MNA...");
    }

    let analysis = analyze_circuit_mna(&circuit, &catalog, &args.output, args.solve)
        .map_err(format_circuit_mna_error)?;

    if args.format == CircuitMnaFormat::Json {
        let output = CircuitMnaOutput::from_analysis(&analysis);
        let json = serde_json::to_string_pretty(&output)
            .map_err(|error| format!("failed to serialize circuit MNA output: {error}"))?;
        println!("{json}");
        return Ok(());
    }

    println!("Generated small-signal netlist:");
    println!("  {}", analysis.spice_path.display());
    println!("Generated MNA netlist:");
    println!("  {}", analysis.cir_path.display());
    println!();
    println!("{}", analysis.mna.nodes);
    print_node_variable_map(&analysis.mna);
    println!("{}", analysis.mna.report);
    println!(
        "{}",
        pretty_system(&analysis.mna.a, &analysis.mna.x, &analysis.mna.z)
    );

    if let Some(solution) = analysis.solution {
        println!("{}", pretty_solutions(&solution.solutions));
    }

    Ok(())
}

fn run_exploration_validate(args: ExplorationValidateArgs) -> Result<(), String> {
    let catalog =
        load_primitive_catalog(&args.primitives_dir).map_err(format_primitive_load_error)?;
    let circuit = load_circuit(&args.circuit).map_err(format_circuit_io_error)?;
    let testbenches = load_testbenches(&args.testbenches).map_err(format_exploration_io_error)?;
    let specs =
        load_exploration_specs(&args.specs, &testbenches).map_err(format_exploration_io_error)?;
    let candidates = args
        .candidates
        .as_ref()
        .map(|path| load_exploration_candidates(path).map_err(format_exploration_io_error))
        .transpose()?;

    let mut rendered_testbenches = Vec::new();
    for spec in &specs {
        let SpecSource::TransferFunction { testbench, .. } = &spec.source else {
            continue;
        };

        if rendered_testbenches
            .iter()
            .any(|name: &String| name == &testbench.name)
        {
            continue;
        }

        render_testbench_small_signal_netlist(&circuit, &catalog, testbench)
            .map_err(format_small_signal_error)?;
        rendered_testbenches.push(testbench.name.clone());
    }

    if args.format == ExplorationValidateFormat::Json {
        let json = serde_json::json!({
            "circuit": circuit.name,
            "testbench_count": testbenches.len(),
            "spec_count": specs.len(),
            "candidate_axis_count": candidates.as_ref().map(|input| input.axes.len()),
            "candidate_set_count": candidates.as_ref().map(|input| input.sets.len()),
            "candidate_filter_count": candidates.as_ref().map(|input| input.filters.len()),
            "rendered_testbenches": rendered_testbenches,
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&json)
                .map_err(|error| format!("failed to serialize exploration validation: {error}"))?
        );
        return Ok(());
    }

    println!("SSTADEx Exploration Validation");
    println!();
    println!("Circuit: {}", circuit.name);
    println!("Testbenches: {}", testbenches.len());
    println!("Specs: {}", specs.len());
    if let Some(candidates) = &candidates {
        println!("Candidate axes: {}", candidates.axes.len());
        println!("Candidate sets: {}", candidates.sets.len());
        println!("Candidate filters: {}", candidates.filters.len());
    }
    println!(
        "Rendered testbench netlists: {}",
        rendered_testbenches.len()
    );
    for name in rendered_testbenches {
        println!("  {name}");
    }

    Ok(())
}

fn run_exploration_prepare(args: ExplorationPrepareArgs) -> Result<(), String> {
    let catalog =
        load_primitive_catalog(&args.primitives_dir).map_err(format_primitive_load_error)?;
    let circuit = load_circuit(&args.circuit).map_err(format_circuit_io_error)?;
    let testbenches = load_testbenches(&args.testbenches).map_err(format_exploration_io_error)?;
    let specs =
        load_exploration_specs(&args.specs, &testbenches).map_err(format_exploration_io_error)?;

    let mut prepared_specs = Vec::with_capacity(specs.len());
    for spec in &specs {
        let prepared = match &spec.source {
            SpecSource::CandidateExpression { .. } => prepare_candidate_expression_spec(spec),
            SpecSource::TransferFunction { .. } => {
                prepare_transfer_function_spec(spec, &circuit, &catalog, &args.output)
            }
            SpecSource::Composed => Err(SpecPrepareError::UnsupportedSource { source: "composed" }),
        }
        .map_err(format_spec_prepare_error)?;
        prepared_specs.push(prepared);
    }

    if args.format == ExplorationPrepareFormat::Json {
        let json = serde_json::json!({
            "circuit": circuit.name,
            "output_dir": args.output.display().to_string(),
            "prepared_specs": prepared_specs
                .iter()
                .map(prepared_spec_json)
                .collect::<Vec<_>>(),
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&json).map_err(|error| format!(
                "failed to serialize prepared exploration specs: {error}"
            ))?
        );
        return Ok(());
    }

    println!("SSTADEx Exploration Prepare");
    println!();
    println!("Circuit: {}", circuit.name);
    println!("Output directory: {}", args.output.display());
    println!("Prepared specs: {}", prepared_specs.len());
    for prepared in &prepared_specs {
        println!();
        println!("Spec: {}", prepared.name);
        println!("Source: {}", prepared_source_kind(prepared));
        if let Some(expression) = prepared_expression(prepared) {
            println!("Expression:");
            println!("{expression}");
        }
    }

    Ok(())
}

fn run_exploration_run(args: ExplorationRunArgs) -> Result<(), String> {
    let catalog =
        load_primitive_catalog(&args.primitives_dir).map_err(format_primitive_load_error)?;
    let circuit = load_circuit(&args.circuit).map_err(format_circuit_io_error)?;
    let testbenches = load_testbenches(&args.testbenches).map_err(format_exploration_io_error)?;
    let specs =
        load_exploration_specs(&args.specs, &testbenches).map_err(format_exploration_io_error)?;
    let candidates =
        load_exploration_candidates(&args.candidates).map_err(format_exploration_io_error)?;

    let mut prepared_specs = Vec::with_capacity(specs.len());
    for spec in &specs {
        let prepared = match &spec.source {
            SpecSource::CandidateExpression { .. } => prepare_candidate_expression_spec(spec),
            SpecSource::TransferFunction { .. } => {
                prepare_transfer_function_spec(spec, &circuit, &catalog, &args.work_dir)
            }
            SpecSource::Composed => Err(SpecPrepareError::UnsupportedSource { source: "composed" }),
        }
        .map_err(format_spec_prepare_error)?;
        prepared_specs.push(prepared);
    }

    let table = run_prepared_expression_flow(
        &candidates.axes,
        &candidates.sets,
        &candidates.filters,
        &prepared_specs,
    )
    .map_err(|error| format!("failed to run exploration: {error:?}"))?;

    if args.format == ExplorationRunFormat::Json {
        let json = serde_json::json!({
            "circuit": circuit.name,
            "candidate_axis_count": candidates.axes.len(),
            "candidate_set_count": candidates.sets.len(),
            "candidate_filter_count": candidates.filters.len(),
            "prepared_spec_count": prepared_specs.len(),
            "row_count": table.row_count,
            "table": exploration_table_json(&table),
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&json)
                .map_err(|error| format!("failed to serialize exploration result: {error}"))?
        );
        return Ok(());
    }

    println!("SSTADEx Exploration Run");
    println!();
    println!("Circuit: {}", circuit.name);
    println!("Candidate axes: {}", candidates.axes.len());
    println!("Candidate sets: {}", candidates.sets.len());
    println!("Candidate filters: {}", candidates.filters.len());
    println!("Prepared specs: {}", prepared_specs.len());
    println!("Kept rows: {}", table.row_count);
    println!();
    print_exploration_table(&table);

    Ok(())
}

fn emit_output(content: &str, output: Option<&PathBuf>) -> Result<(), String> {
    let Some(output) = output else {
        print!("{content}");
        return Ok(());
    };

    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "failed to create output directory '{}': {error}",
                parent.display()
            )
        })?;
    }

    fs::write(output, content).map_err(|error| {
        format!(
            "failed to write output file '{}': {error}",
            output.display()
        )
    })
}

fn render_circuit_view(
    circuit: &Circuit,
    catalog: &libsstadex::catalog::PrimitiveCatalog,
    view: CircuitView,
) -> Result<String, String> {
    match view {
        CircuitView::Structural => {
            render_circuit_netlist(circuit, catalog).map_err(format_netlist_error)
        }
        CircuitView::SmallSignal => {
            render_small_signal_netlist(circuit, catalog).map_err(format_small_signal_error)
        }
    }
}

fn run_catalog(args: Vec<String>) -> Result<(), String> {
    let mut args = args.into_iter();

    match args.next().as_deref() {
        Some("list") => run_catalog_list(parse_catalog_list_args(args.collect())?),
        Some("show") => run_catalog_show(parse_catalog_show_args(args.collect())?),
        Some("--help") | Some("-h") | None => {
            print_catalog_help();
            Ok(())
        }
        Some(command) => Err(format!(
            "unknown catalog command '{command}'\n\nRun `sstadex catalog --help` for usage."
        )),
    }
}

fn parse_catalog_show_args(args: Vec<String>) -> Result<CatalogShowArgs, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_catalog_show_help();
        return Err("catalog show help requested".to_string());
    }

    let mut primitive = None;
    let mut primitives_dir = None;
    let mut idx = 0;

    while idx < args.len() {
        match args[idx].as_str() {
            "--primitives-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--primitives-dir requires a value".to_string())?;
                primitives_dir = Some(PathBuf::from(value));
            }
            value if value.starts_with("--") => {
                return Err(format!(
                    "unknown catalog show option '{value}'\n\nRun `sstadex catalog show --help` for usage."
                ));
            }
            value => {
                if primitive.is_some() {
                    return Err(format!(
                        "unexpected extra argument '{value}'\n\nRun `sstadex catalog show --help` for usage."
                    ));
                }
                primitive = Some(value.to_string());
            }
        }

        idx += 1;
    }

    Ok(CatalogShowArgs {
        primitive: primitive.ok_or_else(|| "missing primitive name".to_string())?,
        primitives_dir: primitives_dir
            .ok_or_else(|| "missing required option --primitives-dir".to_string())?,
    })
}

fn parse_catalog_list_args(args: Vec<String>) -> Result<CatalogListArgs, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_catalog_list_help();
        return Err("catalog list help requested".to_string());
    }

    let mut primitives_dir = None;
    let mut idx = 0;

    while idx < args.len() {
        match args[idx].as_str() {
            "--primitives-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--primitives-dir requires a value".to_string())?;
                primitives_dir = Some(PathBuf::from(value));
            }
            flag => {
                return Err(format!(
                    "unknown catalog list option '{flag}'\n\nRun `sstadex catalog list --help` for usage."
                ));
            }
        }

        idx += 1;
    }

    Ok(CatalogListArgs {
        primitives_dir: primitives_dir
            .ok_or_else(|| "missing required option --primitives-dir".to_string())?,
    })
}

fn run_catalog_list(args: CatalogListArgs) -> Result<(), String> {
    let catalog =
        load_primitive_catalog(&args.primitives_dir).map_err(format_primitive_load_error)?;

    println!("Primitive catalog");
    println!("=================");
    println!();

    for primitive in catalog.list() {
        println!(
            "{:<24} pins={:<3} subckt={}",
            primitive.name,
            primitive.pins.len(),
            primitive.subckt_name
        );
    }

    Ok(())
}

fn run_catalog_show(args: CatalogShowArgs) -> Result<(), String> {
    let catalog =
        load_primitive_catalog(&args.primitives_dir).map_err(format_primitive_load_error)?;
    let primitive = catalog
        .get(&args.primitive)
        .ok_or_else(|| format!("primitive '{}' not found in catalog", args.primitive))?;

    println!("Primitive: {}", primitive.name);
    println!("Version: {}", primitive.version);
    println!("Subckt: {}", primitive.subckt_name);
    if let Some(description) = &primitive.description {
        println!("Description: {description}");
    }
    println!("Netlist: {}", primitive.files.netlist);
    println!("UI: {:?}", primitive.ui.shape);
    println!();
    println!("Pins:");

    for pin in &primitive.pins {
        println!("  {:<12} {:?}", pin.name, pin.role);
    }

    Ok(())
}

fn parse_mna_args(args: Vec<String>) -> Result<MnaArgs, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_mna_help();
        return Err("mna command help requested".to_string());
    }

    let mut spice_dir = None;
    let mut design = None;
    let mut output = None;
    let mut solve = false;

    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--spice-dir" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--spice-dir requires a value".to_string())?;
                spice_dir = Some(PathBuf::from(value));
            }
            "--design" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--design requires a value".to_string())?;
                design = Some(value.clone());
            }
            "--output" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| "--output requires a value".to_string())?;
                output = Some(PathBuf::from(value));
            }
            "--solve" => {
                solve = true;
            }
            flag => {
                return Err(format!(
                    "unknown mna option '{flag}'\n\nRun `sstadex mna --help` for usage."
                ));
            }
        }

        idx += 1;
    }

    Ok(MnaArgs {
        spice_dir: spice_dir.ok_or_else(|| "missing required option --spice-dir".to_string())?,
        design: design.ok_or_else(|| "missing required option --design".to_string())?,
        output: output.ok_or_else(|| "missing required option --output".to_string())?,
        solve,
    })
}

fn run_mna(args: MnaArgs) -> Result<(), String> {
    println!("SSTADEx MNA");
    println!();
    println!("Running MNA...");

    let result = mna(&args.spice_dir, &args.output, &args.design).map_err(format_mna_error)?;
    let generated_netlist = args.output.join(format!("{}.cir", args.design));

    println!("Generated netlist:");
    println!("  {}", generated_netlist.display());
    println!();
    println!("{}", result.nodes);
    print_node_variable_map(&result);
    println!("{}", result.report);
    println!("{}", pretty_system(&result.a, &result.x, &result.z));

    if args.solve {
        let solution = mna_solve(&result.a, &result.x, &result.z).map_err(format_mna_error)?;
        println!("{}", pretty_solutions(&solution.solutions));
    }

    Ok(())
}

fn print_node_variable_map(result: &libsstadex::mna::mna::MnaResult) {
    let node_variables = result.node_variables();
    if node_variables.is_empty() {
        return;
    }

    println!("MNA variable map");
    println!("================");
    println!();
    println!("{:<12} {}", "Variable", "Node name");
    println!("{:<12} {}", "--------", "---------");

    for node in node_variables {
        println!("{:<12} {}", node.variable, node.node_name);
    }

    println!();
}

fn print_exploration_table(table: &ExplorationTable) {
    if table.columns.is_empty() {
        println!("No columns");
        return;
    }

    for column in &table.columns {
        print!("{:<20}", column.name);
    }
    println!();

    for _ in &table.columns {
        print!("{:<20}", "----------------");
    }
    println!();

    for row in 0..table.row_count {
        for column in &table.columns {
            print!("{:<20}", format!("{:.6e}", column.values[row]));
        }
        println!();
    }
}

fn exploration_table_json(table: &ExplorationTable) -> serde_json::Value {
    serde_json::json!({
        "columns": table.columns
            .iter()
            .map(|column| {
                serde_json::json!({
                    "name": column.name,
                    "values": column.values,
                })
            })
            .collect::<Vec<_>>(),
        "rows": (0..table.row_count)
            .map(|row| {
                table.columns
                    .iter()
                    .map(|column| (column.name.clone(), serde_json::json!(column.values[row])))
                    .collect::<serde_json::Map<_, _>>()
            })
            .collect::<Vec<_>>(),
    })
}

fn prepared_spec_json(prepared: &PreparedSpec) -> serde_json::Value {
    serde_json::json!({
        "name": prepared.name,
        "source": prepared_source_kind(prepared),
        "expression": prepared_expression(prepared),
        "parameter_map": prepared
            .parameter_map
            .iter()
            .map(|parameter| {
                serde_json::json!({
                    "name": parameter.name,
                    "value": parameter.value,
                })
            })
            .collect::<Vec<_>>(),
    })
}

fn prepared_source_kind(prepared: &PreparedSpec) -> &'static str {
    match &prepared.source {
        PreparedSpecSource::CandidateExpression { .. } => "candidate_expression",
        PreparedSpecSource::TransferFunction { .. } => "transfer_function",
        PreparedSpecSource::Composed => "composed",
    }
}

fn prepared_expression(prepared: &PreparedSpec) -> Option<&str> {
    match &prepared.source {
        PreparedSpecSource::CandidateExpression { expression }
        | PreparedSpecSource::TransferFunction { expression } => Some(expression),
        PreparedSpecSource::Composed => None,
    }
}

fn format_primitive_load_error(error: PrimitiveLoadError) -> String {
    match error {
        PrimitiveLoadError::Io(error) => format!("I/O failure while loading catalog: {error}"),
        PrimitiveLoadError::Json(error) => format!("invalid primitive JSON: {error}"),
        PrimitiveLoadError::MissingPort { primitive, pin } => {
            format!("primitive '{primitive}' pin_order references missing port '{pin}'")
        }
        PrimitiveLoadError::MissingNetlistFile { primitive } => {
            format!("primitive '{primitive}' is missing files.netlist")
        }
    }
}

fn format_circuit_io_error(error: CircuitIoError) -> String {
    match error {
        CircuitIoError::Io(error) => format!("I/O failure while loading circuit: {error}"),
        CircuitIoError::Json(error) => format!("invalid circuit JSON: {error}"),
    }
}

fn format_exploration_io_error(error: ExplorationIoError) -> String {
    match error {
        ExplorationIoError::Io(error) => {
            format!("I/O failure while loading exploration JSON: {error}")
        }
        ExplorationIoError::Json(error) => format!("invalid exploration JSON: {error}"),
        ExplorationIoError::DuplicateTestbench { name } => {
            format!("duplicate testbench '{name}'")
        }
        ExplorationIoError::MissingTestbench { name } => {
            format!("spec references missing testbench '{name}'")
        }
    }
}

fn format_netlist_error(error: NetlistRenderError) -> String {
    match error {
        NetlistRenderError::InvalidCircuit(errors) => {
            let mut message = String::from("invalid circuit:");
            for error in errors {
                message.push_str(&format!("\n  - {error:?}"));
            }
            message
        }
        NetlistRenderError::UnconnectedPin { instance, pin } => {
            format!("instance '{instance}' pin '{pin}' is not connected")
        }
        NetlistRenderError::MissingPrimitive { primitive } => {
            format!("primitive '{primitive}' is missing from catalog")
        }
        NetlistRenderError::UnsupportedMacroInstance {
            instance,
            macro_name,
        } => {
            format!("instance '{instance}' references macro '{macro_name}', but macro rendering is not implemented yet")
        }
    }
}

fn format_small_signal_error(error: SmallSignalRenderError) -> String {
    match error {
        SmallSignalRenderError::InvalidCircuit(errors) => {
            let mut message = String::from("invalid circuit:");
            for error in errors {
                message.push_str(&format!("\n  - {error:?}"));
            }
            message
        }
        SmallSignalRenderError::MissingPrimitive { primitive } => {
            format!("primitive '{primitive}' is missing from catalog")
        }
        SmallSignalRenderError::MissingSmallSignalModel { primitive } => {
            format!("primitive '{primitive}' has no small-signal model")
        }
        SmallSignalRenderError::UnsupportedMacroInstance {
            instance,
            macro_name,
        } => {
            format!("instance '{instance}' references macro '{macro_name}', but macro small-signal rendering is not implemented yet")
        }
        SmallSignalRenderError::MissingBranchPin {
            instance,
            branch,
            pin,
        } => {
            format!("instance '{instance}' branch '{branch}' references missing pin '{pin}'")
        }
    }
}

fn format_circuit_mna_error(error: CircuitMnaAnalysisError) -> String {
    match error {
        CircuitMnaAnalysisError::Io(error) => format!("I/O failure during circuit MNA: {error}"),
        CircuitMnaAnalysisError::SmallSignalRender(error) => format_small_signal_error(error),
        CircuitMnaAnalysisError::Mna(error) => format_mna_error(error),
    }
}

fn format_spec_prepare_error(error: SpecPrepareError) -> String {
    match error {
        SpecPrepareError::UnsupportedSource { source } => {
            format!("unsupported exploration spec source: {source}")
        }
        SpecPrepareError::TransferFunction(error) => {
            format!("failed to extract transfer function: {error:?}")
        }
        SpecPrepareError::MnaAnalysis { reason } => {
            if reason.contains("No module named 'sympy'") {
                "exploration prepare requires Python package `sympy`, but it is not installed"
                    .to_string()
            } else {
                format!("failed to prepare MNA-backed exploration spec: {reason}")
            }
        }
        SpecPrepareError::MissingMnaSolution => {
            "failed to prepare MNA-backed exploration spec: missing MNA solution".to_string()
        }
    }
}

fn format_mna_error(error: MnaError) -> String {
    match error {
        MnaError::Io(error) => format!("I/O failure: {error}"),
        MnaError::SpiceConversion(error) => format!("SPICE conversion failed: {error:?}"),
        MnaError::SymMna(error) => format!("symbolic MNA failed: {error:?}"),
        MnaError::PythonSolve(error) => {
            if error.contains("No module named 'sympy'") {
                "MNA solve requires Python package `sympy`, but it is not installed".to_string()
            } else {
                format!("Python/SymPy solve failed: {error}")
            }
        }
        MnaError::MissingNode(node) => format!("missing node: {node}"),
    }
}

fn print_help() {
    println!(
        "SSTADEx command line interface\n\
\n\
Usage:\n\
  sstadex --help\n\
  sstadex catalog list --primitives-dir <DIR>\n\
  sstadex circuit render --primitives-dir <DIR> --name <NAME> --instance <ID:PRIMITIVE> --connect <INSTANCE.PIN=NET> [--view structural|small-signal] [--output <FILE>]\n\
  sstadex circuit render-file --primitives-dir <DIR> --circuit <FILE> [--view structural|small-signal] [--output <FILE>]\n\
  sstadex circuit mna --primitives-dir <DIR> --circuit <FILE> --output <DIR> [--solve] [--format text|json]\n\
  sstadex exploration prepare --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> --output <DIR> [--format text|json]\n\
  sstadex exploration run --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> --candidates <FILE> --work-dir <DIR> [--format text|json]\n\
  sstadex exploration validate --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> [--candidates <FILE>] [--format text|json]\n\
  sstadex mna --spice-dir <DIR> --design <NAME> --output <DIR> [--solve]\n\
\n\
Commands:\n\
  catalog    Inspect primitive catalogs\n\
  circuit    Build and render circuits from primitives\n\
  exploration Prepare, validate, and run exploration flows\n\
  mna        Generate and print a symbolic MNA system from a SPICE netlist\n"
    );
}

fn print_catalog_help() {
    println!(
        "Usage:\n\
  sstadex catalog --help\n\
  sstadex catalog list --primitives-dir <DIR>\n\
  sstadex catalog show <PRIMITIVE> --primitives-dir <DIR>\n\
\n\
Commands:\n\
  list    List primitives found in a primitives directory\n\
  show    Show details for one primitive\n"
    );
}

fn print_catalog_list_help() {
    println!(
        "Usage:\n\
  sstadex catalog list --primitives-dir <DIR>\n\
\n\
Options:\n\
  --primitives-dir <DIR>    Directory containing primitive subfolders\n"
    );
}

fn print_catalog_show_help() {
    println!(
        "Usage:\n\
  sstadex catalog show <PRIMITIVE> --primitives-dir <DIR>\n\
\n\
Options:\n\
  --primitives-dir <DIR>    Directory containing primitive subfolders\n"
    );
}

fn print_circuit_help() {
    println!(
        "Usage:\n\
  sstadex circuit --help\n\
  sstadex circuit render --primitives-dir <DIR> --name <NAME> --instance <ID:PRIMITIVE> --connect <INSTANCE.PIN=NET> [--view structural|small-signal] [--output <FILE>]\n\
  sstadex circuit render-file --primitives-dir <DIR> --circuit <FILE> [--view structural|small-signal] [--output <FILE>]\n\
  sstadex circuit mna --primitives-dir <DIR> --circuit <FILE> --output <DIR> [--solve] [--format text|json]\n\
\n\
Commands:\n\
  render         Render a circuit netlist from instances and connections\n\
  render-file    Render a circuit netlist from a circuit JSON file\n\
  mna            Analyze a circuit JSON file with small-signal MNA\n"
    );
}

fn print_circuit_render_help() {
    println!(
        "Usage:\n\
  sstadex circuit render --primitives-dir <DIR> --name <NAME> --instance <ID:PRIMITIVE> --connect <INSTANCE.PIN=NET> [--view structural|small-signal] [--output <FILE>]\n\
\n\
Options:\n\
  --primitives-dir <DIR>       Directory containing primitive subfolders\n\
  --name <NAME>                Circuit name\n\
  --instance <ID:PRIMITIVE>    Primitive instance; can be repeated\n\
  --connect <INSTANCE.PIN=NET> Pin-to-net connection; can be repeated\n\
  --view <VIEW>                Render view: structural or small-signal; default structural\n\
  --output <FILE>              Write rendered netlist to a file; stdout if omitted\n"
    );
}

fn print_circuit_render_file_help() {
    println!(
        "Usage:\n\
  sstadex circuit render-file --primitives-dir <DIR> --circuit <FILE> [--view structural|small-signal] [--output <FILE>]\n\
\n\
Options:\n\
  --primitives-dir <DIR>    Directory containing primitive subfolders\n\
  --circuit <FILE>          Circuit JSON file\n\
  --view <VIEW>             Render view: structural or small-signal; default structural\n\
  --output <FILE>           Write rendered netlist to a file; stdout if omitted\n"
    );
}

fn print_circuit_mna_help() {
    println!(
        "Usage:\n\
  sstadex circuit mna --primitives-dir <DIR> --circuit <FILE> --output <DIR> [--solve] [--format text|json]\n\
\n\
Options:\n\
  --primitives-dir <DIR>    Directory containing primitive subfolders\n\
  --circuit <FILE>          Circuit JSON file\n\
  --output <DIR>            Directory where generated .spice and .cir files are written\n\
  --solve                   Solve the symbolic MNA system using Python/SymPy\n\
  --format <FORMAT>         Output format: text or json; default text\n"
    );
}

fn print_exploration_help() {
    println!(
        "Usage:\n\
  sstadex exploration --help\n\
  sstadex exploration prepare --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> --output <DIR> [--format text|json]\n\
  sstadex exploration run --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> --candidates <FILE> --work-dir <DIR> [--format text|json]\n\
  sstadex exploration validate --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> [--candidates <FILE>] [--format text|json]\n\
\n\
Commands:\n\
  prepare     Prepare exploration specs and extract transfer-function expressions\n\
  run         Run exploration specs over candidate inputs and print the filtered table\n\
  validate    Validate exploration testbenches/specs and render referenced testbench netlists\n"
    );
}

fn print_exploration_prepare_help() {
    println!(
        "Usage:\n\
  sstadex exploration prepare --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> --output <DIR> [--format text|json]\n\
\n\
Options:\n\
  --primitives-dir <DIR>    Directory containing primitive subfolders\n\
  --circuit <FILE>          Circuit JSON file\n\
  --testbenches <FILE>      Exploration testbench JSON file\n\
  --specs <FILE>            Exploration spec JSON file\n\
  --output <DIR>            Directory where generated .spice and .cir files are written\n\
  --format <FORMAT>         Output format: text or json; default text\n"
    );
}

fn print_exploration_validate_help() {
    println!(
        "Usage:\n\
  sstadex exploration validate --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> [--candidates <FILE>] [--format text|json]\n\
\n\
Options:\n\
  --primitives-dir <DIR>    Directory containing primitive subfolders\n\
  --circuit <FILE>          Circuit JSON file\n\
  --testbenches <FILE>      Exploration testbench JSON file\n\
  --specs <FILE>            Exploration spec JSON file\n\
  --candidates <FILE>       Optional exploration candidate JSON file\n\
  --format <FORMAT>         Output format: text or json; default text\n"
    );
}

fn print_exploration_run_help() {
    println!(
        "Usage:\n\
  sstadex exploration run --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> --candidates <FILE> --work-dir <DIR> [--format text|json]\n\
\n\
Options:\n\
  --primitives-dir <DIR>    Directory containing primitive subfolders\n\
  --circuit <FILE>          Circuit JSON file\n\
  --testbenches <FILE>      Exploration testbench JSON file\n\
  --specs <FILE>            Exploration spec JSON file\n\
  --candidates <FILE>       Exploration candidate JSON file\n\
  --work-dir <DIR>          Directory where transfer-function intermediate files are written\n\
  --format <FORMAT>         Output format: text or json; default text\n"
    );
}

fn print_mna_help() {
    println!(
        "Usage:\n\
  sstadex mna --spice-dir <DIR> --design <NAME> --output <DIR> [--solve]\n\
\n\
Options:\n\
  --spice-dir <DIR>    Directory containing <NAME>.spice\n\
  --design <NAME>      Design name without extension\n\
  --output <DIR>       Directory where generated files are written\n\
  --solve              Solve the symbolic MNA system using Python/SymPy\n"
    );
}
