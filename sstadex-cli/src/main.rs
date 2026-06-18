use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use libsstadex::analysis::{analyze_circuit_mna, CircuitMnaAnalysisError, CircuitMnaOutput};
use libsstadex::catalog::{load_primitive_catalog, PrimitiveLoadError};
use libsstadex::circuit::{load_circuit, Circuit, CircuitIoError, Connection, Instance, PinRef};
use libsstadex::exploration::{
    load_exploration_specs, load_testbenches, ExplorationIoError, SpecSource,
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
    format: ExplorationValidateFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExplorationValidateFormat {
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
    println!(
        "Rendered testbench netlists: {}",
        rendered_testbenches.len()
    );
    for name in rendered_testbenches {
        println!("  {name}");
    }

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
  sstadex exploration validate --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> [--format text|json]\n\
  sstadex mna --spice-dir <DIR> --design <NAME> --output <DIR> [--solve]\n\
\n\
Commands:\n\
  catalog    Inspect primitive catalogs\n\
  circuit    Build and render circuits from primitives\n\
  exploration Validate exploration testbenches and specs\n\
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
  sstadex exploration validate --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> [--format text|json]\n\
\n\
Commands:\n\
  validate    Validate exploration testbenches/specs and render referenced testbench netlists\n"
    );
}

fn print_exploration_validate_help() {
    println!(
        "Usage:\n\
  sstadex exploration validate --primitives-dir <DIR> --circuit <FILE> --testbenches <FILE> --specs <FILE> [--format text|json]\n\
\n\
Options:\n\
  --primitives-dir <DIR>    Directory containing primitive subfolders\n\
  --circuit <FILE>          Circuit JSON file\n\
  --testbenches <FILE>      Exploration testbench JSON file\n\
  --specs <FILE>            Exploration spec JSON file\n\
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
