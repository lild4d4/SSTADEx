use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use libsstadex::mna::mna::{mna, mna_solve, MnaError};
use libsstadex::mna::pretty::{pretty_solutions, pretty_system};

#[derive(Debug)]
struct MnaArgs {
    spice_dir: PathBuf,
    design: String,
    output: PathBuf,
    solve: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("--help") | Some("-h") => {
            print_help();
            Ok(())
        }
        Some("mna") => run_mna(parse_mna_args(args.collect())?),
        Some(command) => Err(format!(
            "unknown command '{command}'\n\nRun `sstadex --help` for usage."
        )),
    }
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
    let result = mna(&args.spice_dir, &args.output, &args.design).map_err(format_mna_error)?;
    let generated_netlist = args.output.join(format!("{}.cir", args.design));

    println!("SSTADEx MNA");
    println!();
    println!("Generated netlist:");
    println!("  {}", generated_netlist.display());
    println!();
    println!("{}", result.report);
    println!("{}", pretty_system(&result.a, &result.x, &result.z));

    if args.solve {
        let solution = mna_solve(&result.a, &result.x, &result.z).map_err(format_mna_error)?;
        println!("{}", pretty_solutions(&solution.solutions));
    }

    Ok(())
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
  sstadex mna --spice-dir <DIR> --design <NAME> --output <DIR> [--solve]\n\
\n\
Commands:\n\
  mna    Generate and print a symbolic MNA system from a SPICE netlist\n"
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
