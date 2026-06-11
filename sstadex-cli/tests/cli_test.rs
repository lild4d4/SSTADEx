use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn sstadex(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sstadex"))
        .args(args)
        .output()
        .expect("failed to run sstadex binary")
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("sstadex-cli should be inside the workspace")
        .to_path_buf()
}

fn primitives_dir() -> PathBuf {
    workspace_root().join("analoglib/primitives")
}

fn ota_circuit_file() -> PathBuf {
    workspace_root().join("libsstadex/examples/circuits/ota_primitives.json")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "command failed\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn top_level_help_exits_successfully() {
    let output = sstadex(&["--help"]);

    assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SSTADEx command line interface"));
}

#[test]
fn render_file_help_exits_successfully() {
    let output = sstadex(&["circuit", "render-file", "--help"]);

    assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("sstadex circuit render-file"));
    assert!(stdout.contains("--output <FILE>"));
}

#[test]
fn circuit_mna_help_exits_successfully() {
    let output = sstadex(&["circuit", "mna", "--help"]);

    assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("sstadex circuit mna"));
    assert!(stdout.contains("--solve"));
}

#[test]
fn render_file_writes_netlist_to_stdout_by_default() {
    let primitives_dir = primitives_dir();
    let circuit = ota_circuit_file();
    let output = sstadex(&[
        "circuit",
        "render-file",
        "--primitives-dir",
        primitives_dir.to_str().expect("valid primitives path"),
        "--circuit",
        circuit.to_str().expect("valid circuit path"),
        "--view",
        "small-signal",
    ]);

    assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("* Small-signal circuit: ota_primitives"));
    assert!(stdout.contains("G_gm__xdp__m1 VOUT IBIAS VINP IBIAS gm__xdp__m1"));
}

#[test]
fn render_file_output_writes_netlist_to_file() {
    let output_dir = std::env::temp_dir().join(format!("sstadex-cli-test-{}", std::process::id()));
    fs::create_dir_all(&output_dir).expect("failed to create temporary test directory");

    let output_file = output_dir.join("ota_small_signal.spice");
    let primitives_dir = primitives_dir();
    let circuit = ota_circuit_file();
    let output = sstadex(&[
        "circuit",
        "render-file",
        "--primitives-dir",
        primitives_dir.to_str().expect("valid primitives path"),
        "--circuit",
        circuit.to_str().expect("valid circuit path"),
        "--view",
        "small-signal",
        "--output",
        output_file.to_str().expect("valid output path"),
    ]);

    assert_success(&output);
    assert!(
        output.stdout.is_empty(),
        "expected empty stdout when --output is used, got:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    let netlist = fs::read_to_string(&output_file).expect("failed to read generated netlist");
    assert!(netlist.contains("* Small-signal circuit: ota_primitives"));
    assert!(netlist.contains("R_ro__xcs__m1 IBIAS VSS ro__xcs__m1"));

    fs::remove_dir_all(output_dir).expect("failed to remove temporary test directory");
}

#[test]
fn circuit_mna_generates_small_signal_and_mna_netlists() {
    let output_dir = std::env::temp_dir().join(format!(
        "sstadex-cli-circuit-mna-test-{}",
        std::process::id()
    ));
    fs::create_dir_all(&output_dir).expect("failed to create temporary test directory");

    let primitives_dir = primitives_dir();
    let circuit = ota_circuit_file();
    let output = sstadex(&[
        "circuit",
        "mna",
        "--primitives-dir",
        primitives_dir.to_str().expect("valid primitives path"),
        "--circuit",
        circuit.to_str().expect("valid circuit path"),
        "--output",
        output_dir.to_str().expect("valid output path"),
    ]);

    assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SSTADEx Circuit MNA"));
    assert!(stdout.contains("Running MNA..."));
    assert!(stdout.contains("Generated small-signal netlist:"));
    assert!(stdout.contains("Generated MNA netlist:"));
    assert!(stdout.contains("SPICE parser report"));
    assert!(stdout.contains("MNA variable map"));
    assert!(stdout.contains("v1           VOUT"));
    assert!(stdout.contains("v3           VINP"));
    assert!(stdout.contains("gm__xdp__m1"));

    let spice = fs::read_to_string(output_dir.join("ota_primitives.spice"))
        .expect("failed to read generated small-signal netlist");
    let cir = fs::read_to_string(output_dir.join("ota_primitives.cir"))
        .expect("failed to read MNA netlist");

    assert!(spice.contains("G_gm__xdp__m1 VOUT IBIAS VINP IBIAS gm__xdp__m1"));
    assert!(cir.contains("G_gm__xdp__m1"));

    fs::remove_dir_all(output_dir).expect("failed to remove temporary test directory");
}

#[test]
fn circuit_mna_json_outputs_structured_result() {
    let output_dir = std::env::temp_dir().join(format!(
        "sstadex-cli-circuit-mna-json-test-{}",
        std::process::id()
    ));
    fs::create_dir_all(&output_dir).expect("failed to create temporary test directory");

    let primitives_dir = primitives_dir();
    let circuit = ota_circuit_file();
    let output = sstadex(&[
        "circuit",
        "mna",
        "--primitives-dir",
        primitives_dir.to_str().expect("valid primitives path"),
        "--circuit",
        circuit.to_str().expect("valid circuit path"),
        "--output",
        output_dir.to_str().expect("valid output path"),
        "--format",
        "json",
    ]);

    assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("Running MNA..."));

    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("expected valid JSON output");

    assert_eq!(
        json["spice_path"],
        output_dir
            .join("ota_primitives.spice")
            .display()
            .to_string()
    );
    assert_eq!(
        json["cir_path"],
        output_dir.join("ota_primitives.cir").display().to_string()
    );
    assert_eq!(json["solution"], serde_json::Value::Null);
    assert_eq!(json["nodes"][1]["name"], "VOUT");
    assert_eq!(json["nodes"][1]["number"], 1);
    assert_eq!(json["variables"][0]["variable"], "v1");
    assert_eq!(json["variables"][0]["node_name"], "VOUT");
    assert!(
        json["equations"]
            .as_array()
            .expect("equations should be an array")
            .iter()
            .any(|equation| equation["text"]
                .as_str()
                .expect("equation text should be a string")
                .contains("gm__xdp__m1"))
    );

    fs::remove_dir_all(output_dir).expect("failed to remove temporary test directory");
}
