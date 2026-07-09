use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use eframe::egui;
use libsstadex::analysis::{
    CircuitMnaOutput, analyze_circuit_mna, analyze_macro_testbench_mna_with_mode,
};
use libsstadex::catalog::{PrimitiveCatalog, load_primitive_catalog};
use libsstadex::circuit::{Circuit, Connection, Instance, PinRef, save_circuit};
use libsstadex::exploration::{
    CandidateAxis, CandidatePoint, CandidateSet, CompactOutputBinding, DerivedColumnSpec,
    ExplorationCandidateInput, ExplorationSpec, ExplorationTable, PreparedSpec, PreparedSpecSource,
    RangeCondition, SpecOutput, SpecParameter, SpecSource, TestbenchElement, TestbenchSpec,
    add_automatic_area_column, build_filtered_candidates, prepare_macro_testbench_specs_with_mode,
    run_prepared_expression_flow_with_derived_columns, save_exploration_candidates,
    save_exploration_specs, save_testbenches, submacro_results_to_compact_candidate_set,
};
use libsstadex::macro_model::{
    MacroCatalog, MacroMetadata, MacroModel, MacroPort, MacroPortRole, MacroSmallSignalMode,
    MacroSmallSignalModel, MacroSymbol, MacroSymbolPin, load_macro_catalog, save_macro_model,
};
use libsstadex::primitive::build::{
    PrimitiveBuildEngine, PrimitiveBuildInput, PrimitiveBuildInputKind, PrimitiveBuildValue,
    PythonGmidLutBackend,
};
use libsstadex::primitive::manifest::{PinRole, PrimitiveManifest, SymbolPinSide};
use serde::{Deserialize, Serialize};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "SSTADEx",
        options,
        Box::new(|_cc| Ok(Box::new(SstadexApp::default()))),
    )
}

struct SstadexApp {
    catalog: Option<PrimitiveCatalog>,
    macro_catalog: Option<MacroCatalog>,
    load_error: Option<String>,
    macro_load_error: Option<String>,
    show_insert_block_window: bool,
    insert_block_selection: Option<GuiInsertBlockSelection>,
    canvas_instances: Vec<CanvasInstance>,
    label_pins: Vec<CanvasLabelPin>,
    macro_ports: Vec<CanvasMacroPort>,
    selected_instance_id: Option<usize>,
    selected_endpoint: Option<CanvasEndpoint>,
    pending_connection: Option<CanvasEndpoint>,
    connections: Vec<CanvasConnection>,
    circuits: Vec<GuiCircuitDocument>,
    macro_workspaces: Vec<GuiMacroWorkspace>,
    active_circuit: usize,
    active_document: ActiveDocument,
    renaming_circuit: Option<usize>,
    bottom_view: BottomView,
    testbenches: Vec<GuiTestbenchDocument>,
    selected_testbench: Option<usize>,
    renaming_testbench: Option<usize>,
    specs: Vec<GuiSpecDocument>,
    selected_spec: Option<usize>,
    renaming_spec: Option<usize>,
    derived_columns: Vec<GuiDerivedColumnDocument>,
    candidates: GuiCandidateDocument,
    project_path: String,
    output_log: String,
    output_netlist: String,
    output_equations: String,
    output_artifacts: String,
    output_prepared_specs: String,
    output_candidates: String,
    output_results: String,
    output_results_table: Option<ExplorationTable>,
    next_instance_id: usize,
    next_label_pin_id: usize,
    next_macro_port_id: usize,
}

#[derive(Clone)]
struct GuiMacroWorkspace {
    testbenches: Vec<GuiTestbenchDocument>,
    selected_testbench: Option<usize>,
    specs: Vec<GuiSpecDocument>,
    selected_spec: Option<usize>,
    derived_columns: Vec<GuiDerivedColumnDocument>,
    candidates: GuiCandidateDocument,
    output_prepared_specs: String,
    output_candidates: String,
    output_results: String,
    output_results_table: Option<ExplorationTable>,
}

struct GuiWorkspaceEvaluation {
    prepared_specs: Vec<PreparedSpec>,
    candidate_input: ExplorationCandidateInput,
    table: ExplorationTable,
}

#[derive(Clone)]
struct CanvasInstance {
    id: usize,
    instance_name: String,
    block: GuiBlockRef,
    position: egui::Pos2,
    orientation: GuiOrientation,
}

#[derive(Clone)]
enum GuiBlockRef {
    Primitive { name: String },
    Macro { name: String },
}

#[derive(Clone, PartialEq, Eq)]
enum GuiInsertBlockSelection {
    Primitive(String),
    Macro(String),
}

impl GuiInsertBlockSelection {
    fn into_block_ref(self) -> GuiBlockRef {
        match self {
            Self::Primitive(name) => GuiBlockRef::Primitive { name },
            Self::Macro(name) => GuiBlockRef::Macro { name },
        }
    }
}

impl GuiBlockRef {
    fn primitive_name(&self) -> Option<&str> {
        match self {
            Self::Primitive { name } => Some(name),
            Self::Macro { .. } => None,
        }
    }

    fn macro_name(&self) -> Option<&str> {
        match self {
            Self::Primitive { .. } => None,
            Self::Macro { name } => Some(name),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Primitive { name } | Self::Macro { name } => name,
        }
    }

    fn kind_label(&self) -> &'static str {
        match self {
            Self::Primitive { .. } => "Primitive",
            Self::Macro { .. } => "Macro",
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum CanvasEndpoint {
    PrimitivePin {
        instance_id: usize,
        pin_name: String,
    },
    LabelPin {
        label_id: usize,
    },
    MacroPort {
        port_id: usize,
    },
}

#[derive(Clone)]
struct CanvasLabelPin {
    id: usize,
    name: String,
    position: egui::Pos2,
}

#[derive(Clone)]
struct CanvasMacroPort {
    id: usize,
    name: String,
    role: GuiMacroPortRole,
    symbol_side: SymbolPinSide,
    symbol_offset: f32,
    position: egui::Pos2,
}

#[derive(Clone)]
struct CanvasConnection {
    from: CanvasEndpoint,
    to: CanvasEndpoint,
}

struct GuiCircuitDocument {
    name: String,
    subckt_name: String,
    small_signal: Option<MacroSmallSignalModel>,
    ports: Vec<GuiMacroPort>,
    canvas_instances: Vec<CanvasInstance>,
    label_pins: Vec<CanvasLabelPin>,
    macro_ports: Vec<CanvasMacroPort>,
    connections: Vec<CanvasConnection>,
    next_instance_id: usize,
    next_label_pin_id: usize,
    next_macro_port_id: usize,
}

#[derive(Clone)]
struct GuiMacroPort {
    name: String,
    role: GuiMacroPortRole,
    symbol_side: SymbolPinSide,
    symbol_offset: f32,
}

#[derive(Clone)]
struct GuiDutMacroView {
    name: String,
    ports: Vec<GuiMacroPort>,
}

#[derive(Clone, Copy)]
enum GuiMacroPortView<'a> {
    Canvas(&'a CanvasMacroPort),
    Metadata(&'a GuiMacroPort),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GuiMacroPortRole {
    Input,
    Output,
    Inout,
    Bias,
    Supply,
    Ground,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BottomView {
    Logs,
    Netlist,
    Equations,
    Artifacts,
    Prepared,
    Candidates,
    Results,
    Testbenches,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActiveDocument {
    Circuit,
    Testbench,
    Spec,
    Candidates,
}

#[derive(Clone)]
struct GuiTestbenchDocument {
    name: String,
    dut_macro: String,
    dut_position: egui::Pos2,
    small_signal_mode: GuiSmallSignalMode,
    compact_outputs: Vec<GuiCompactOutputBinding>,
    elements: Vec<GuiTestbenchElement>,
    connections: Vec<GuiTestbenchConnection>,
    selected_endpoint: Option<TestbenchEndpoint>,
    pending_connection: Option<TestbenchEndpoint>,
    extra_body: String,
    next_element_id: usize,
}

#[derive(Clone, Default)]
struct GuiCompactOutputBinding {
    source_column: String,
    compact_parameter: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GuiSmallSignalMode {
    CompactWhenAvailable,
    Expand,
}

#[derive(Clone)]
struct GuiTestbenchConnection {
    from: TestbenchEndpoint,
    to: TestbenchEndpoint,
}

#[derive(Clone)]
struct GuiSpecDocument {
    name: String,
    source_kind: GuiSpecSourceKind,
    testbench: String,
    input: String,
    output: String,
    node: String,
    min: String,
    max: String,
    parameter_map: Vec<GuiSpecParameter>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuiSpecSourceKind {
    TransferFunction,
    NodeVoltage,
}

impl GuiSpecSourceKind {
    fn all() -> [Self; 2] {
        [Self::TransferFunction, Self::NodeVoltage]
    }

    fn label(self) -> &'static str {
        match self {
            Self::TransferFunction => "transfer function",
            Self::NodeVoltage => "node voltage",
        }
    }
}

#[derive(Clone)]
struct GuiSpecParameter {
    name: String,
    value: String,
}

#[derive(Clone)]
struct GuiDerivedColumnDocument {
    name: String,
    expression: String,
}

#[derive(Clone)]
struct GuiCandidateDocument {
    axes: Vec<GuiCandidateAxis>,
    net_voltage_constraints: Vec<GuiNetVoltageConstraint>,
    global_build_parameters: Vec<GuiBuildParameter>,
    primitive_build_overrides: Vec<GuiPrimitiveBuildOverride>,
    python_path: String,
    nmos_lut_path: String,
    pmos_lut_path: String,
    timing_output: bool,
}

#[derive(Clone)]
struct GuiCandidateAxis {
    name: String,
    values: String,
}

#[derive(Clone)]
struct GuiNetVoltageConstraint {
    net: String,
    values: String,
}

#[derive(Clone)]
struct GuiBuildParameter {
    name: String,
    values: String,
}

#[derive(Clone)]
struct GuiPrimitiveBuildOverride {
    instance_id: usize,
    parameters: Vec<GuiBuildParameter>,
}

struct GuiPrimitiveBuildInstanceSummary {
    id: usize,
    name: String,
    primitive: String,
    non_voltage_inputs: Vec<String>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum TestbenchEndpoint {
    ElementPin {
        element_id: usize,
        pin: TestbenchPin,
    },
    DutPort {
        port_name: String,
    },
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum TestbenchPin {
    A,
    B,
}

#[derive(Clone)]
struct GuiTestbenchElement {
    id: usize,
    kind: GuiTestbenchElementKind,
    name: String,
    nplus: String,
    nminus: String,
    value: String,
    position: egui::Pos2,
    orientation: GuiOrientation,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GuiOrientation {
    R0,
    R90,
    R180,
    R270,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GuiTestbenchElementKind {
    VoltageSource,
    CurrentSource,
    Resistor,
    Capacitor,
}

#[derive(Serialize, Deserialize)]
struct GuiProject {
    version: u32,
    active_circuit: usize,
    circuits: Vec<GuiProjectCircuit>,
    #[serde(default)]
    macro_workspaces: Vec<GuiProjectMacroWorkspace>,
    selected_testbench: Option<usize>,
    testbenches: Vec<GuiProjectTestbench>,
    #[serde(default)]
    selected_spec: Option<usize>,
    #[serde(default)]
    specs: Vec<GuiProjectSpec>,
    #[serde(default)]
    derived_columns: Vec<GuiProjectDerivedColumn>,
    #[serde(default)]
    candidates: GuiProjectCandidateDocument,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectMacroWorkspace {
    #[serde(default)]
    selected_testbench: Option<usize>,
    #[serde(default)]
    testbenches: Vec<GuiProjectTestbench>,
    #[serde(default)]
    selected_spec: Option<usize>,
    #[serde(default)]
    specs: Vec<GuiProjectSpec>,
    #[serde(default)]
    derived_columns: Vec<GuiProjectDerivedColumn>,
    #[serde(default)]
    candidates: GuiProjectCandidateDocument,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectSpec {
    name: String,
    #[serde(default)]
    source_kind: GuiProjectSpecSourceKind,
    testbench: String,
    #[serde(default)]
    input: String,
    #[serde(default)]
    output: String,
    #[serde(default)]
    node: String,
    min: String,
    max: String,
    parameter_map: Vec<GuiProjectSpecParameter>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
enum GuiProjectSpecSourceKind {
    #[default]
    TransferFunction,
    NodeVoltage,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectSpecParameter {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectDerivedColumn {
    name: String,
    expression: String,
}

#[derive(Serialize, Deserialize, Default)]
struct GuiProjectCandidateDocument {
    #[serde(default)]
    axes: Vec<GuiProjectCandidateAxis>,
    #[serde(default)]
    net_voltage_constraints: Vec<GuiProjectNetVoltageConstraint>,
    #[serde(default)]
    global_build_parameters: Vec<GuiProjectBuildParameter>,
    #[serde(default)]
    primitive_build_overrides: Vec<GuiProjectPrimitiveBuildOverride>,
    #[serde(default)]
    python_path: String,
    #[serde(default)]
    nmos_lut_path: String,
    #[serde(default)]
    pmos_lut_path: String,
    #[serde(default)]
    timing_output: bool,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectCandidateAxis {
    name: String,
    values: String,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectNetVoltageConstraint {
    net: String,
    values: String,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectBuildParameter {
    name: String,
    values: String,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectPrimitiveBuildOverride {
    instance_id: usize,
    parameters: Vec<GuiProjectBuildParameter>,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectCircuit {
    name: String,
    #[serde(default)]
    subckt_name: String,
    #[serde(default)]
    small_signal: Option<MacroSmallSignalModel>,
    #[serde(default)]
    ports: Vec<GuiProjectMacroPort>,
    instances: Vec<GuiProjectInstance>,
    label_pins: Vec<GuiProjectLabelPin>,
    #[serde(default)]
    macro_ports: Vec<GuiProjectCanvasMacroPort>,
    connections: Vec<GuiProjectConnection>,
    next_instance_id: usize,
    next_label_pin_id: usize,
    #[serde(default)]
    next_macro_port_id: usize,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectMacroPort {
    name: String,
    role: GuiProjectMacroPortRole,
    #[serde(default)]
    symbol_side: Option<SymbolPinSide>,
    #[serde(default)]
    symbol_offset: Option<f32>,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectCanvasMacroPort {
    id: usize,
    name: String,
    role: GuiProjectMacroPortRole,
    symbol_side: SymbolPinSide,
    symbol_offset: f32,
    position: GuiProjectPosition,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GuiProjectMacroPortRole {
    Input,
    Output,
    Inout,
    Bias,
    Supply,
    Ground,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectTestbench {
    name: String,
    #[serde(default)]
    dut_macro: String,
    #[serde(default)]
    small_signal_mode: GuiProjectSmallSignalMode,
    #[serde(default)]
    dut_position: Option<GuiProjectPosition>,
    #[serde(default)]
    compact_outputs: Vec<GuiProjectCompactOutputBinding>,
    elements: Vec<GuiProjectTestbenchElement>,
    #[serde(default)]
    connections: Vec<GuiProjectTestbenchConnection>,
    extra_body: String,
    next_element_id: usize,
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct GuiProjectCompactOutputBinding {
    source_column: String,
    compact_parameter: String,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
enum GuiProjectSmallSignalMode {
    #[default]
    CompactWhenAvailable,
    Expand,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectTestbenchConnection {
    from: GuiProjectTestbenchEndpoint,
    to: GuiProjectTestbenchEndpoint,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum GuiProjectTestbenchEndpoint {
    ElementPin {
        element_id: usize,
        pin: GuiProjectTestbenchPin,
    },
    DutPort {
        port_name: String,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GuiProjectTestbenchPin {
    A,
    B,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectTestbenchElement {
    id: usize,
    kind: GuiProjectTestbenchElementKind,
    name: String,
    nplus: String,
    nminus: String,
    value: String,
    position: GuiProjectPosition,
    #[serde(default)]
    orientation: GuiProjectOrientation,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
enum GuiProjectOrientation {
    #[default]
    R0,
    R90,
    R180,
    R270,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GuiProjectTestbenchElementKind {
    VoltageSource,
    CurrentSource,
    Resistor,
    Capacitor,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectInstance {
    id: usize,
    name: String,
    #[serde(default)]
    primitive: String,
    #[serde(default)]
    block: Option<GuiProjectBlockRef>,
    position: GuiProjectPosition,
    #[serde(default)]
    orientation: GuiProjectOrientation,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum GuiProjectBlockRef {
    Primitive { name: String },
    Macro { name: String },
}

#[derive(Serialize, Deserialize)]
struct GuiProjectLabelPin {
    id: usize,
    name: String,
    position: GuiProjectPosition,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectConnection {
    from: GuiProjectEndpoint,
    to: GuiProjectEndpoint,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum GuiProjectEndpoint {
    PrimitivePin { instance_id: usize, pin: String },
    LabelPin { label_id: usize },
    MacroPort { port_id: usize },
}

#[derive(Serialize, Deserialize)]
struct GuiProjectPosition {
    x: f32,
    y: f32,
}

struct CanvasView {
    rect: egui::Rect,
}

impl CanvasView {
    fn to_screen(&self, local: egui::Pos2) -> egui::Pos2 {
        self.rect.min + local.to_vec2()
    }
}

impl Default for SstadexApp {
    fn default() -> Self {
        let primitives_dir = PathBuf::from("analoglib/primitives");
        let macros_dir = PathBuf::from("analoglib/macros");
        let (catalog, load_error) = match load_primitive_catalog(&primitives_dir) {
            Ok(catalog) => (Some(catalog), None),
            Err(error) => (None, Some(format!("{error:?}"))),
        };
        let (macro_catalog, macro_load_error) = match load_macro_catalog(&macros_dir) {
            Ok(catalog) => (Some(catalog), None),
            Err(error) => (None, Some(format!("{error:?}"))),
        };

        Self {
            catalog,
            macro_catalog,
            load_error,
            macro_load_error,
            show_insert_block_window: false,
            insert_block_selection: None,
            canvas_instances: Vec::new(),
            label_pins: Vec::new(),
            macro_ports: Vec::new(),
            selected_instance_id: None,
            selected_endpoint: None,
            pending_connection: None,
            connections: Vec::new(),
            circuits: vec![GuiCircuitDocument::empty("macro_1")],
            macro_workspaces: vec![GuiMacroWorkspace::default()],
            active_circuit: 0,
            active_document: ActiveDocument::Circuit,
            renaming_circuit: None,
            bottom_view: BottomView::Logs,
            testbenches: Vec::new(),
            selected_testbench: None,
            renaming_testbench: None,
            specs: Vec::new(),
            selected_spec: None,
            renaming_spec: None,
            derived_columns: Vec::new(),
            candidates: GuiCandidateDocument::default(),
            project_path: default_project_path().display().to_string(),
            output_log: "Logs, netlists, and MNA results will appear here".to_string(),
            output_netlist: "Generated SPICE netlist will appear here".to_string(),
            output_equations: "MNA equations will appear here".to_string(),
            output_artifacts: "Generated artifact paths will appear here".to_string(),
            output_prepared_specs: "Prepared exploration specs will appear here".to_string(),
            output_candidates: "Generated candidates will appear here".to_string(),
            output_results: "Exploration results will appear here".to_string(),
            output_results_table: None,
            next_instance_id: 1,
            next_label_pin_id: 1,
            next_macro_port_id: 1,
        }
    }
}

impl Default for GuiMacroWorkspace {
    fn default() -> Self {
        Self {
            testbenches: Vec::new(),
            selected_testbench: None,
            specs: Vec::new(),
            selected_spec: None,
            derived_columns: Vec::new(),
            candidates: GuiCandidateDocument::default(),
            output_prepared_specs: "Prepared exploration specs will appear here".to_string(),
            output_candidates: "Generated candidates will appear here".to_string(),
            output_results: "Exploration results will appear here".to_string(),
            output_results_table: None,
        }
    }
}

impl Default for GuiCandidateDocument {
    fn default() -> Self {
        Self {
            axes: vec![GuiCandidateAxis {
                name: "xdp.gm".to_string(),
                values: "1e-3, 2e-3, 5e-3".to_string(),
            }],
            net_voltage_constraints: Vec::new(),
            global_build_parameters: vec![GuiBuildParameter {
                name: "current".to_string(),
                values: "100e-6".to_string(),
            }],
            primitive_build_overrides: Vec::new(),
            python_path: default_python_gmid_path(),
            nmos_lut_path: default_nmos_lut_path(),
            pmos_lut_path: default_pmos_lut_path(),
            timing_output: true,
        }
    }
}

impl eframe::App for SstadexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.active_document == ActiveDocument::Circuit
            && ctx.input(|input| input.key_pressed(egui::Key::Delete))
        {
            self.delete_selected_canvas_item();
        }
        if self.active_document == ActiveDocument::Testbench
            && ctx.input(|input| input.key_pressed(egui::Key::Delete))
        {
            self.delete_selected_testbench_element();
        }
        if ctx.input(|input| input.key_pressed(egui::Key::R)) {
            self.rotate_selected_canvas_item();
        }

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Project:");
                ui.add_sized(
                    egui::vec2(320.0, 20.0),
                    egui::TextEdit::singleline(&mut self.project_path),
                );
                if ui.button("Open...").clicked() {
                    self.choose_open_project_path();
                }
                if ui.button("Save as...").clicked() {
                    self.choose_save_project_path();
                }
                if ui.button("Open project").clicked() {
                    self.output_log = self.open_gui_project();
                    self.bottom_view = BottomView::Logs;
                }
                if ui.button("Save project").clicked() {
                    self.output_log = self.save_gui_project();
                    self.bottom_view = BottomView::Logs;
                }
                let is_circuit_active = self.active_document == ActiveDocument::Circuit;
                if ui
                    .add_enabled(is_circuit_active, egui::Button::new("Add lab pin"))
                    .clicked()
                {
                    self.add_label_pin();
                }
                if ui
                    .add_enabled(is_circuit_active, egui::Button::new("Add port"))
                    .clicked()
                {
                    self.add_macro_port();
                }
                if ui
                    .add_enabled(is_circuit_active, egui::Button::new("Insert block"))
                    .clicked()
                {
                    self.show_insert_block_window = true;
                }
                if ui
                    .add_enabled(is_circuit_active, egui::Button::new("Save macro"))
                    .clicked()
                {
                    self.output_log = self.save_active_macro_model();
                    self.bottom_view = BottomView::Logs;
                }
                if ui.button("Save flow inputs").clicked() {
                    self.output_log = self.save_flow_inputs();
                    self.bottom_view = BottomView::Logs;
                }
                if ui.button("Save specs").clicked() {
                    self.output_log = self.save_gui_specs();
                    self.bottom_view = BottomView::Logs;
                }
                if ui.button("Prepare specs").clicked() {
                    self.output_log = self.prepare_gui_specs();
                    self.bottom_view = BottomView::Prepared;
                }
                if ui
                    .add_enabled(
                        self.active_document == ActiveDocument::Candidates,
                        egui::Button::new("Generate candidates"),
                    )
                    .clicked()
                {
                    self.output_log = self.generate_gui_candidates();
                    self.bottom_view = BottomView::Candidates;
                }
                if ui.button("Evaluate specs").clicked() {
                    self.output_log = self.evaluate_gui_specs();
                    self.bottom_view = BottomView::Results;
                }
                if ui.button("Evaluate hierarchy").clicked() {
                    self.output_log = self.evaluate_gui_hierarchy();
                    self.bottom_view = BottomView::Results;
                }
                if ui
                    .add_enabled(
                        self.selected_testbench.is_some(),
                        egui::Button::new("Run testbench MNA"),
                    )
                    .clicked()
                {
                    self.output_log = self.run_selected_testbench_mna();
                    self.bottom_view = BottomView::Logs;
                }
                if ui
                    .add_enabled(is_circuit_active, egui::Button::new("Run MNA"))
                    .clicked()
                {
                    self.output_log = self.run_mna_from_canvas();
                }
            });
        });

        self.show_insert_block_window(ctx);

        egui::SidePanel::left("project_browser")
            .resizable(true)
            .default_width(220.0)
            .show(ctx, |ui| {
                self.show_project_browser_ui(ui);
            });

        egui::SidePanel::right("details")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("Details");
                ui.separator();

                match self.active_document {
                    ActiveDocument::Circuit => {
                        if let Some(circuit) = self.circuits.get_mut(self.active_circuit) {
                            show_macro_document_details(ui, circuit);
                            show_validation_messages(ui, &validate_macro_document(circuit));
                            ui.separator();
                        }

                        if let Some(label_pin) = self.selected_label_pin_mut() {
                            show_label_pin_details(ui, label_pin);
                        } else if let Some(macro_port) = self.selected_macro_port_mut() {
                            show_macro_port_details(ui, macro_port);
                        } else {
                            let selected_endpoint = self.selected_endpoint.clone();

                            if let Some(instance) = self.selected_instance_mut() {
                                show_instance_details(ui, instance, selected_endpoint.as_ref());
                            } else {
                                ui.label("Nothing selected");
                            }
                        }
                    }
                    ActiveDocument::Testbench => {
                        let macro_names = self.macro_names();
                        if let Some(testbench) = self.selected_testbench_document() {
                            ui.heading(&testbench.name);
                            ui.label(format!(
                                "DUT macro: {}",
                                display_optional_name(&testbench.dut_macro)
                            ));
                            ui.label(format!(
                                "Small-signal mode: {}",
                                testbench.small_signal_mode.label()
                            ));
                            ui.label(format!("Elements: {}", testbench.elements.len()));
                            show_validation_messages(
                                ui,
                                &validate_testbench_document(
                                    testbench,
                                    &macro_names,
                                    self.selected_testbench.unwrap_or(0) + 1,
                                ),
                            );
                            ui.label(if testbench.extra_body.trim().is_empty() {
                                "Extra body: empty"
                            } else {
                                "Extra body: present"
                            });
                        } else {
                            ui.label("No testbench selected");
                        }
                    }
                    ActiveDocument::Spec => {
                        if let Some(spec) = self.selected_spec_document() {
                            ui.heading(&spec.name);
                            ui.label(format!(
                                "Testbench: {}",
                                display_optional_name(&spec.testbench)
                            ));
                            ui.label(format!("Input: {}", display_optional_name(&spec.input)));
                            ui.label(format!("Output: {}", display_optional_name(&spec.output)));
                            ui.label(format!("Parameters: {}", spec.parameter_map.len()));
                        } else {
                            ui.label("No spec selected");
                        }
                    }
                    ActiveDocument::Candidates => {
                        ui.heading("Candidates");
                        ui.label(format!("Axes: {}", self.candidates.axes.len()));
                        ui.label("Use Generate candidates to build the current candidate grid.");
                    }
                }
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(240.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.bottom_view, BottomView::Logs, "Logs");
                    ui.selectable_value(&mut self.bottom_view, BottomView::Netlist, "Netlist");
                    ui.selectable_value(&mut self.bottom_view, BottomView::Equations, "Equations");
                    ui.selectable_value(&mut self.bottom_view, BottomView::Artifacts, "Artifacts");
                    ui.selectable_value(&mut self.bottom_view, BottomView::Prepared, "Prepared");
                    ui.selectable_value(
                        &mut self.bottom_view,
                        BottomView::Candidates,
                        "Candidates",
                    );
                    ui.selectable_value(&mut self.bottom_view, BottomView::Results, "Results");
                    ui.selectable_value(
                        &mut self.bottom_view,
                        BottomView::Testbenches,
                        "Testbenches",
                    );
                });
                ui.separator();

                match self.bottom_view {
                    BottomView::Logs => self.show_logs_ui(ui),
                    BottomView::Netlist => show_text_output(ui, &self.output_netlist),
                    BottomView::Equations => show_text_output(ui, &self.output_equations),
                    BottomView::Artifacts => show_text_output(ui, &self.output_artifacts),
                    BottomView::Prepared => show_text_output(ui, &self.output_prepared_specs),
                    BottomView::Candidates => show_text_output(ui, &self.output_candidates),
                    BottomView::Results => show_results_output(
                        ui,
                        &self.output_results,
                        self.output_results_table.as_ref(),
                    ),
                    BottomView::Testbenches => self.show_testbenches_ui(ui),
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| match self.active_document {
            ActiveDocument::Circuit => self.show_circuit_document_ui(ui),
            ActiveDocument::Testbench => self.show_testbench_document_ui(ui),
            ActiveDocument::Spec => self.show_spec_document_ui(ui),
            ActiveDocument::Candidates => self.show_candidate_document_ui(ui),
        });
    }
}

impl SstadexApp {
    fn show_circuit_document_ui(&mut self, ui: &mut egui::Ui) {
        let active_circuit_name = self
            .circuits
            .get(self.active_circuit)
            .map(|circuit| circuit.name.as_str())
            .unwrap_or("macro_1");
        ui.heading(format!("Macro - {active_circuit_name}"));
        ui.separator();

        let canvas_rect = ui.available_rect_before_wrap();
        let canvas = CanvasView { rect: canvas_rect };
        let painter = ui.painter_at(canvas_rect);
        let macro_blocks = self.available_macro_block_views();

        painter.rect_filled(canvas_rect, 0.0, egui::Color32::from_gray(24));

        if self.canvas_instances.is_empty() {
            ui.label("No instances yet");
        }

        draw_canvas_connections(
            &painter,
            &canvas,
            &self.canvas_instances,
            &self.label_pins,
            &self.macro_ports,
            self.catalog.as_ref(),
            &macro_blocks,
            &self.connections,
        );

        for instance in &mut self.canvas_instances {
            let rect = canvas.instance_rect(instance);
            let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());

            if response.clicked() || response.dragged() {
                self.selected_instance_id = Some(instance.id);
            }

            if response.dragged() {
                instance.position += response.drag_delta();
            }

            let selected = self.selected_instance_id == Some(instance.id);
            if let Some(pin_views) =
                block_pin_views(rect, instance, self.catalog.as_ref(), &macro_blocks)
            {
                for pin_view in pin_views {
                    let hit_rect =
                        egui::Rect::from_center_size(pin_view.position, egui::vec2(14.0, 14.0));
                    let response = ui.allocate_rect(hit_rect, egui::Sense::click());

                    if response.clicked() {
                        let selected_endpoint = CanvasEndpoint::PrimitivePin {
                            instance_id: instance.id,
                            pin_name: pin_view.name.clone(),
                        };

                        self.selected_instance_id = Some(instance.id);
                        self.selected_endpoint = Some(selected_endpoint.clone());
                        update_pending_connection(
                            &mut self.pending_connection,
                            &mut self.connections,
                            selected_endpoint,
                        );
                    }
                }
            }

            draw_canvas_instance(
                &painter,
                rect,
                instance,
                self.selected_endpoint.as_ref(),
                selected,
            );
            if let Some(pin_views) =
                block_pin_views(rect, instance, self.catalog.as_ref(), &macro_blocks)
            {
                draw_instance_pin_views(
                    &painter,
                    instance.id,
                    &pin_views,
                    self.selected_endpoint.as_ref(),
                );
            }
        }

        for label_pin in &mut self.label_pins {
            let position = canvas.to_screen(label_pin.position);
            let hit_rect = egui::Rect::from_center_size(position, egui::vec2(18.0, 18.0));
            let response = ui.allocate_rect(hit_rect, egui::Sense::click_and_drag());

            if response.clicked() || response.dragged() {
                self.selected_instance_id = None;
                self.selected_endpoint = Some(CanvasEndpoint::LabelPin {
                    label_id: label_pin.id,
                });
            }

            if response.clicked() {
                let selected_endpoint = CanvasEndpoint::LabelPin {
                    label_id: label_pin.id,
                };
                update_pending_connection(
                    &mut self.pending_connection,
                    &mut self.connections,
                    selected_endpoint,
                );
            }

            if response.dragged() {
                label_pin.position += response.drag_delta();
            }

            let selected = self.selected_endpoint.as_ref()
                == Some(&CanvasEndpoint::LabelPin {
                    label_id: label_pin.id,
                });
            draw_label_pin(&painter, position, label_pin, selected);
        }

        let macro_port_issue_ids =
            macro_port_canvas_issue_ids(&self.macro_ports, &self.connections);
        for macro_port in &mut self.macro_ports {
            let position = canvas.to_screen(macro_port.position);
            let hit_rect = egui::Rect::from_center_size(position, egui::vec2(18.0, 18.0));
            let response = ui.allocate_rect(hit_rect, egui::Sense::click_and_drag());

            if response.clicked() || response.dragged() {
                self.selected_instance_id = None;
                self.selected_endpoint = Some(CanvasEndpoint::MacroPort {
                    port_id: macro_port.id,
                });
            }

            if response.clicked() {
                let selected_endpoint = CanvasEndpoint::MacroPort {
                    port_id: macro_port.id,
                };
                update_pending_connection(
                    &mut self.pending_connection,
                    &mut self.connections,
                    selected_endpoint,
                );
            }

            if response.dragged() {
                macro_port.position += response.drag_delta();
            }

            let selected = self.selected_endpoint.as_ref()
                == Some(&CanvasEndpoint::MacroPort {
                    port_id: macro_port.id,
                });
            draw_macro_port_pin(
                &painter,
                position,
                macro_port,
                selected,
                macro_port_issue_ids.contains(&macro_port.id),
            );
        }
    }

    fn show_testbench_document_ui(&mut self, ui: &mut egui::Ui) {
        let Some(selected_index) = self.selected_testbench else {
            ui.heading("Testbench");
            ui.separator();
            ui.label("Select a testbench from the project browser");
            return;
        };

        let macro_names = self.macro_names();
        let dut_macro_view = self
            .testbenches
            .get(selected_index)
            .and_then(|testbench| {
                self.circuits
                    .iter()
                    .find(|circuit| circuit.name == testbench.dut_macro)
            })
            .map(GuiDutMacroView::from_circuit_document);
        let Some(testbench) = self.testbenches.get_mut(selected_index) else {
            self.selected_testbench = None;
            ui.label("Selected testbench no longer exists");
            return;
        };

        ui.heading(format!("Testbench - {}", testbench.name));
        ui.separator();

        show_testbench_dut_selector(ui, testbench, &macro_names);
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Insert:");
            if ui.button("Voltage source").clicked() {
                testbench.add_element(GuiTestbenchElementKind::VoltageSource);
            }
            if ui.button("Current source").clicked() {
                testbench.add_element(GuiTestbenchElementKind::CurrentSource);
            }
            if ui.button("Resistor").clicked() {
                testbench.add_element(GuiTestbenchElementKind::Resistor);
            }
            if ui.button("Capacitor").clicked() {
                testbench.add_element(GuiTestbenchElementKind::Capacitor);
            }
        });
        ui.separator();

        let canvas_max_height = (ui.available_height() * 0.55).clamp(220.0, 420.0);
        egui::ScrollArea::both()
            .id_salt("testbench_canvas_scroll")
            .max_height(canvas_max_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                draw_testbench_canvas(ui, testbench, dut_macro_view.as_ref());
            });

        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("central_testbench_editor_scroll")
            .show(ui, |ui| {
                show_testbench_editor(ui, selected_index, testbench);
            });
    }

    fn show_spec_document_ui(&mut self, ui: &mut egui::Ui) {
        let Some(selected_index) = self.selected_spec else {
            ui.heading("Spec");
            ui.separator();
            ui.label("Select a spec from the project browser");
            return;
        };

        let testbench_names = self
            .testbenches
            .iter()
            .map(|testbench| testbench.name.clone())
            .collect::<Vec<_>>();
        let Some(spec) = self.specs.get_mut(selected_index) else {
            self.selected_spec = None;
            ui.label("Selected spec no longer exists");
            return;
        };

        ui.heading(format!("Spec - {}", spec.name));
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut spec.name);
        });
        ui.horizontal(|ui| {
            ui.label("Type:");
            egui::ComboBox::from_id_salt("spec_source_kind")
                .selected_text(spec.source_kind.label())
                .show_ui(ui, |ui| {
                    for kind in GuiSpecSourceKind::all() {
                        ui.selectable_value(&mut spec.source_kind, kind, kind.label());
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label("Testbench:");
            egui::ComboBox::from_id_salt("spec_testbench")
                .selected_text(display_optional_name(&spec.testbench))
                .show_ui(ui, |ui| {
                    for testbench_name in &testbench_names {
                        ui.selectable_value(
                            &mut spec.testbench,
                            testbench_name.clone(),
                            testbench_name,
                        );
                    }
                });
        });
        match spec.source_kind {
            GuiSpecSourceKind::TransferFunction => {
                ui.horizontal(|ui| {
                    ui.label("Input:");
                    ui.text_edit_singleline(&mut spec.input);
                    ui.label("Output:");
                    ui.text_edit_singleline(&mut spec.output);
                });
            }
            GuiSpecSourceKind::NodeVoltage => {
                ui.horizontal(|ui| {
                    ui.label("Node:");
                    ui.text_edit_singleline(&mut spec.node);
                });
            }
        }
        ui.horizontal(|ui| {
            ui.label("Min:");
            ui.text_edit_singleline(&mut spec.min);
            ui.label("Max:");
            ui.text_edit_singleline(&mut spec.max);
        });

        ui.separator();
        ui.horizontal(|ui| {
            ui.heading("Parameter map");
            if ui.button("+").clicked() {
                spec.parameter_map.push(GuiSpecParameter {
                    name: String::new(),
                    value: String::new(),
                });
            }
        });

        let mut remove_parameter = None;
        for (index, parameter) in spec.parameter_map.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut parameter.name);
                ui.label("=");
                ui.text_edit_singleline(&mut parameter.value);
                if ui.button("Delete").clicked() {
                    remove_parameter = Some(index);
                }
            });
        }
        if let Some(index) = remove_parameter {
            spec.parameter_map.remove(index);
        }
    }

    fn show_candidate_document_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Candidates");
        ui.separator();

        let macro_blocks = self.available_macro_block_views();
        let net_names = self
            .catalog
            .as_ref()
            .and_then(|catalog| {
                self.circuits.get(self.active_circuit).map(|document| {
                    CanvasNetIndex::from_document(document, catalog, &macro_blocks)
                        .nets()
                        .iter()
                        .map(|net| net.name.clone())
                        .collect::<Vec<_>>()
                })
            })
            .unwrap_or_default();
        let build_instances = self.gui_primitive_build_instances();

        ui.horizontal(|ui| {
            if ui.button("+ axis").clicked() {
                self.candidates.axes.push(GuiCandidateAxis {
                    name: String::new(),
                    values: String::new(),
                });
            }
            if ui.button("+ net voltage").clicked() {
                let net = net_names
                    .iter()
                    .find(|net| {
                        !self
                            .candidates
                            .net_voltage_constraints
                            .iter()
                            .any(|constraint| constraint.net == **net)
                    })
                    .cloned()
                    .or_else(|| net_names.first().cloned())
                    .unwrap_or_default();
                self.candidates
                    .net_voltage_constraints
                    .push(GuiNetVoltageConstraint {
                        net,
                        values: String::new(),
                    });
            }
            if ui.button("+ parameter").clicked() {
                self.candidates
                    .global_build_parameters
                    .push(GuiBuildParameter {
                        name: String::new(),
                        values: String::new(),
                    });
            }
            if ui.button("+ derived").clicked() {
                self.derived_columns.push(GuiDerivedColumnDocument {
                    name: String::new(),
                    expression: String::new(),
                });
            }
            if ui.button("Generate candidates").clicked() {
                self.output_log = self.generate_gui_candidates();
                self.bottom_view = BottomView::Candidates;
            }
        });

        ui.separator();
        ui.label("Python GMID backend");
        ui.horizontal(|ui| {
            ui.label("Python");
            ui.add_sized(
                egui::vec2(260.0, 20.0),
                egui::TextEdit::singleline(&mut self.candidates.python_path),
            );
            ui.checkbox(&mut self.candidates.timing_output, "Timing");
        });
        ui.horizontal(|ui| {
            ui.label("NMOS LUT");
            ui.add_sized(
                egui::vec2(360.0, 20.0),
                egui::TextEdit::singleline(&mut self.candidates.nmos_lut_path),
            );
        });
        ui.horizontal(|ui| {
            ui.label("PMOS LUT");
            ui.add_sized(
                egui::vec2(360.0, 20.0),
                egui::TextEdit::singleline(&mut self.candidates.pmos_lut_path),
            );
        });

        ui.separator();
        ui.label("Manual axes");

        let mut remove_axis = None;
        for (index, axis) in self.candidates.axes.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("Axis {}", index + 1));
                ui.add_sized(
                    egui::vec2(140.0, 20.0),
                    egui::TextEdit::singleline(&mut axis.name).hint_text("name"),
                );
                ui.add_sized(
                    egui::vec2(320.0, 20.0),
                    egui::TextEdit::singleline(&mut axis.values)
                        .hint_text("1e-3, 2e-3, 5e-3 or linspace(1e-3, 5e-3, 9)"),
                );
                if ui.button("Delete").clicked() {
                    remove_axis = Some(index);
                }
            });
        }

        if let Some(index) = remove_axis {
            self.candidates.axes.remove(index);
        }

        ui.separator();
        ui.label("Derived columns");
        let mut remove_derived = None;
        for (index, column) in self.derived_columns.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("Column {}", index + 1));
                ui.add_sized(
                    egui::vec2(140.0, 20.0),
                    egui::TextEdit::singleline(&mut column.name).hint_text("gain_db"),
                );
                ui.label("=");
                ui.add_sized(
                    egui::vec2(320.0, 20.0),
                    egui::TextEdit::singleline(&mut column.expression)
                        .hint_text("20 * log10(abs(gain))"),
                );
                if ui.button("Delete").clicked() {
                    remove_derived = Some(index);
                }
            });
        }
        if let Some(index) = remove_derived {
            self.derived_columns.remove(index);
        }

        ui.separator();
        ui.label("Net voltage constraints");
        if net_names.is_empty() {
            ui.label("No canvas nets available in the active macro.");
        }
        let mut remove_constraint = None;
        for (index, constraint) in self
            .candidates
            .net_voltage_constraints
            .iter_mut()
            .enumerate()
        {
            ui.horizontal(|ui| {
                ui.label(format!("Net {}", index + 1));
                egui::ComboBox::from_id_salt(format!("candidate_net_constraint_{index}"))
                    .selected_text(display_optional_name(&constraint.net))
                    .show_ui(ui, |ui| {
                        for net in &net_names {
                            ui.selectable_value(&mut constraint.net, net.clone(), net);
                        }
                    });
                ui.add_sized(
                    egui::vec2(320.0, 20.0),
                    egui::TextEdit::singleline(&mut constraint.values)
                        .hint_text("0.8 or linspace(0.5, 1.0, 6)"),
                );
                if ui.button("Delete").clicked() {
                    remove_constraint = Some(index);
                }
            });
        }
        if let Some(index) = remove_constraint {
            self.candidates.net_voltage_constraints.remove(index);
        }

        ui.separator();
        ui.label("Global build parameters");
        let mut remove_parameter = None;
        for (index, parameter) in self
            .candidates
            .global_build_parameters
            .iter_mut()
            .enumerate()
        {
            ui.horizontal(|ui| {
                ui.label(format!("Parameter {}", index + 1));
                ui.add_sized(
                    egui::vec2(140.0, 20.0),
                    egui::TextEdit::singleline(&mut parameter.name).hint_text("current"),
                );
                ui.add_sized(
                    egui::vec2(320.0, 20.0),
                    egui::TextEdit::singleline(&mut parameter.values)
                        .hint_text("100e-6 or linspace(50e-6, 200e-6, 4)"),
                );
                if ui.button("Delete").clicked() {
                    remove_parameter = Some(index);
                }
            });
        }
        if let Some(index) = remove_parameter {
            self.candidates.global_build_parameters.remove(index);
        }

        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!(
                "Primitive builds from active canvas: {}",
                build_instances.len()
            ));
            if ui.button("+ instance override").clicked() {
                if let Some(instance) = build_instances.first() {
                    self.candidates
                        .primitive_build_overrides
                        .push(GuiPrimitiveBuildOverride {
                            instance_id: instance.id,
                            parameters: instance
                                .non_voltage_inputs
                                .iter()
                                .map(|name| GuiBuildParameter {
                                    name: name.clone(),
                                    values: String::new(),
                                })
                                .collect(),
                        });
                }
            }
        });
        for instance in &build_instances {
            ui.label(format!(
                "{} ({}) inputs: {}",
                instance.name,
                instance.primitive,
                instance
                    .non_voltage_inputs
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        let mut remove_override = None;
        for (override_index, override_) in self
            .candidates
            .primitive_build_overrides
            .iter_mut()
            .enumerate()
        {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Override");
                    egui::ComboBox::from_id_salt(format!(
                        "candidate_instance_override_{override_index}"
                    ))
                    .selected_text(
                        build_instances
                            .iter()
                            .find(|instance| instance.id == override_.instance_id)
                            .map(|instance| instance.name.as_str())
                            .unwrap_or("missing instance"),
                    )
                    .show_ui(ui, |ui| {
                        for instance in &build_instances {
                            ui.selectable_value(
                                &mut override_.instance_id,
                                instance.id,
                                &instance.name,
                            );
                        }
                    });
                    if ui.button("+ parameter").clicked() {
                        override_.parameters.push(GuiBuildParameter {
                            name: String::new(),
                            values: String::new(),
                        });
                    }
                    if ui.button("Delete").clicked() {
                        remove_override = Some(override_index);
                    }
                });

                let mut remove_override_parameter = None;
                for (parameter_index, parameter) in override_.parameters.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            egui::vec2(140.0, 20.0),
                            egui::TextEdit::singleline(&mut parameter.name).hint_text("current"),
                        );
                        ui.add_sized(
                            egui::vec2(320.0, 20.0),
                            egui::TextEdit::singleline(&mut parameter.values)
                                .hint_text("100e-6 or linspace(50e-6, 200e-6, 4)"),
                        );
                        if ui.button("Delete").clicked() {
                            remove_override_parameter = Some(parameter_index);
                        }
                    });
                }
                if let Some(parameter_index) = remove_override_parameter {
                    override_.parameters.remove(parameter_index);
                }
            });
        }
        if let Some(index) = remove_override {
            self.candidates.primitive_build_overrides.remove(index);
        }
    }

    fn show_project_browser_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Project");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Macros");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("+").clicked() {
                    self.add_circuit_document();
                }
            });
        });

        for index in 0..self.circuits.len() {
            self.show_circuit_browser_item(ui, index);
        }

        self.show_hierarchy_browser_ui(ui);

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Testbenches for active macro");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("+").clicked() {
                    self.add_testbench();
                }
            });
        });

        for index in 0..self.testbenches.len() {
            self.show_testbench_browser_item(ui, index);
        }

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Specs for active macro");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("+").clicked() {
                    self.add_spec();
                }
            });
        });

        for index in 0..self.specs.len() {
            self.show_spec_browser_item(ui, index);
        }

        ui.separator();
        ui.label("Exploration for active macro");
        if ui
            .selectable_label(
                self.active_document == ActiveDocument::Candidates,
                "Candidates",
            )
            .clicked()
        {
            self.active_document = ActiveDocument::Candidates;
            self.bottom_view = BottomView::Candidates;
        }
    }

    fn show_hierarchy_browser_ui(&mut self, ui: &mut egui::Ui) {
        let Some(document) = self.circuits.get(self.active_circuit) else {
            return;
        };
        let children = document
            .canvas_instances
            .iter()
            .filter_map(|instance| {
                let macro_name = instance.block.macro_name()?;
                let macro_index = self
                    .circuits
                    .iter()
                    .position(|circuit| circuit.name == macro_name)?;
                Some((
                    exported_instance_name(instance),
                    macro_name.to_string(),
                    macro_index,
                ))
            })
            .collect::<Vec<_>>();

        ui.separator();
        ui.label("Hierarchy");
        if children.is_empty() {
            ui.label("No child macro instances");
            return;
        }

        for (instance_name, macro_name, macro_index) in children {
            let label = format!("{instance_name}: {macro_name}");
            if ui.button(label).clicked() {
                self.switch_circuit_document(macro_index);
                self.active_document = ActiveDocument::Circuit;
            }
        }
    }

    fn gui_primitive_build_instances(&self) -> Vec<GuiPrimitiveBuildInstanceSummary> {
        let Some(catalog) = &self.catalog else {
            return Vec::new();
        };
        let Some(document) = self.circuits.get(self.active_circuit) else {
            return Vec::new();
        };

        document
            .canvas_instances
            .iter()
            .filter_map(|instance| {
                let primitive = instance
                    .block
                    .primitive_name()
                    .and_then(|name| catalog.get(name))?;
                let build = primitive.build.as_ref()?;
                Some(GuiPrimitiveBuildInstanceSummary {
                    id: instance.id,
                    name: exported_instance_name(instance),
                    primitive: primitive.name.clone(),
                    non_voltage_inputs: build
                        .inputs
                        .iter()
                        .filter(|input| input.source.as_deref() != Some("port_voltage"))
                        .map(|input| input.name.clone())
                        .collect(),
                })
            })
            .collect()
    }

    fn show_circuit_browser_item(&mut self, ui: &mut egui::Ui, index: usize) {
        if self.renaming_circuit == Some(index) {
            let old_name = self.circuits[index].name.clone();
            let response = ui.text_edit_singleline(&mut self.circuits[index].name);
            if self.circuits[index].subckt_name == old_name {
                self.circuits[index].subckt_name = self.circuits[index].name.clone();
            }
            let new_name = self.circuits[index].name.clone();
            if new_name != old_name {
                for testbench in &mut self.testbenches {
                    if testbench.dut_macro == old_name {
                        testbench.dut_macro = new_name.clone();
                    }
                }
                for workspace in &mut self.macro_workspaces {
                    for testbench in &mut workspace.testbenches {
                        if testbench.dut_macro == old_name {
                            testbench.dut_macro = new_name.clone();
                        }
                    }
                }
            }
            if response.lost_focus()
                || ui.input(|input| {
                    input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Escape)
                })
            {
                self.renaming_circuit = None;
            }
            return;
        }

        let selected = self.active_circuit == index;
        let name = self.circuits[index].name.clone();
        let response = ui.selectable_label(selected, name);

        if response.clicked() {
            self.switch_circuit_document(index);
            self.active_document = ActiveDocument::Circuit;
        }

        response.context_menu(|ui| {
            if ui.button("Rename").clicked() {
                self.renaming_circuit = Some(index);
                ui.close();
            }

            let can_delete = self.circuits.len() > 1;
            if ui
                .add_enabled(can_delete, egui::Button::new("Delete"))
                .clicked()
            {
                self.delete_circuit_document(index);
                ui.close();
            }
        });
    }

    fn show_testbench_browser_item(&mut self, ui: &mut egui::Ui, index: usize) {
        if self.renaming_testbench == Some(index) {
            let response = ui.text_edit_singleline(&mut self.testbenches[index].name);
            if response.lost_focus()
                || ui.input(|input| {
                    input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Escape)
                })
            {
                self.renaming_testbench = None;
            }
            return;
        }

        let label = if self.testbenches[index].name.trim().is_empty() {
            "(unnamed)"
        } else {
            self.testbenches[index].name.as_str()
        };
        let response = ui.selectable_label(self.selected_testbench == Some(index), label);

        if response.clicked() {
            self.selected_testbench = Some(index);
            self.active_document = ActiveDocument::Testbench;
            self.bottom_view = BottomView::Testbenches;
        }

        response.context_menu(|ui| {
            if ui.button("Rename").clicked() {
                self.renaming_testbench = Some(index);
                ui.close();
            }
            if ui.button("Delete").clicked() {
                self.delete_testbench(index);
                ui.close();
            }
        });
    }

    fn show_spec_browser_item(&mut self, ui: &mut egui::Ui, index: usize) {
        if self.renaming_spec == Some(index) {
            let response = ui.text_edit_singleline(&mut self.specs[index].name);
            if response.lost_focus()
                || ui.input(|input| {
                    input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Escape)
                })
            {
                self.renaming_spec = None;
            }
            return;
        }

        let label = if self.specs[index].name.trim().is_empty() {
            "(unnamed)"
        } else {
            self.specs[index].name.as_str()
        };
        let response = ui.selectable_label(self.selected_spec == Some(index), label);

        if response.clicked() {
            self.selected_spec = Some(index);
            self.active_document = ActiveDocument::Spec;
        }

        response.context_menu(|ui| {
            if ui.button("Rename").clicked() {
                self.renaming_spec = Some(index);
                ui.close();
            }
            if ui.button("Delete").clicked() {
                self.delete_spec(index);
                ui.close();
            }
        });
    }

    fn add_circuit_document(&mut self) {
        self.save_active_circuit_document();
        self.save_active_macro_workspace();

        let name = next_available_circuit_name(&self.circuits);
        self.circuits.push(GuiCircuitDocument::empty(name));
        self.macro_workspaces.push(GuiMacroWorkspace::default());
        self.active_circuit = self.circuits.len() - 1;
        self.active_document = ActiveDocument::Circuit;
        self.load_active_circuit_document();
        self.load_active_macro_workspace();
    }

    fn switch_circuit_document(&mut self, index: usize) {
        if index == self.active_circuit || index >= self.circuits.len() {
            return;
        }

        self.save_active_circuit_document();
        self.save_active_macro_workspace();
        self.active_circuit = index;
        self.load_active_circuit_document();
        self.load_active_macro_workspace();
    }

    fn delete_circuit_document(&mut self, index: usize) {
        if self.circuits.len() <= 1 || index >= self.circuits.len() {
            return;
        }

        self.save_active_circuit_document();
        self.save_active_macro_workspace();
        self.circuits.remove(index);
        if index < self.macro_workspaces.len() {
            self.macro_workspaces.remove(index);
        }

        if self.active_circuit == index {
            self.active_circuit = index.saturating_sub(1).min(self.circuits.len() - 1);
            self.load_active_circuit_document();
            self.load_active_macro_workspace();
        } else if self.active_circuit > index {
            self.active_circuit -= 1;
        }

        if self.renaming_circuit == Some(index) {
            self.renaming_circuit = None;
        } else if let Some(renaming_index) = self.renaming_circuit {
            if renaming_index > index {
                self.renaming_circuit = Some(renaming_index - 1);
            }
        }

        self.ensure_testbench_dut_macros();
    }

    fn delete_testbench(&mut self, index: usize) {
        if index >= self.testbenches.len() {
            return;
        }

        self.testbenches.remove(index);

        if self.selected_testbench == Some(index) {
            self.selected_testbench = None;
        } else if let Some(selected_index) = self.selected_testbench {
            if selected_index > index {
                self.selected_testbench = Some(selected_index - 1);
            }
        }

        if self.renaming_testbench == Some(index) {
            self.renaming_testbench = None;
        } else if let Some(renaming_index) = self.renaming_testbench {
            if renaming_index > index {
                self.renaming_testbench = Some(renaming_index - 1);
            }
        }
    }

    fn delete_spec(&mut self, index: usize) {
        if index >= self.specs.len() {
            return;
        }

        self.specs.remove(index);

        if self.selected_spec == Some(index) {
            self.selected_spec = None;
        } else if let Some(selected_index) = self.selected_spec {
            if selected_index > index {
                self.selected_spec = Some(selected_index - 1);
            }
        }

        if self.renaming_spec == Some(index) {
            self.renaming_spec = None;
        } else if let Some(renaming_index) = self.renaming_spec {
            if renaming_index > index {
                self.renaming_spec = Some(renaming_index - 1);
            }
        }
    }

    fn save_active_circuit_document(&mut self) {
        let Some(circuit) = self.circuits.get_mut(self.active_circuit) else {
            return;
        };

        circuit.canvas_instances = self.canvas_instances.clone();
        circuit.label_pins = self.label_pins.clone();
        circuit.macro_ports = self.macro_ports.clone();
        circuit.connections = self.connections.clone();
        circuit.next_instance_id = self.next_instance_id;
        circuit.next_label_pin_id = self.next_label_pin_id;
        circuit.next_macro_port_id = self.next_macro_port_id;
    }

    fn save_active_macro_workspace(&mut self) {
        self.ensure_macro_workspace_count();
        let Some(workspace) = self.macro_workspaces.get_mut(self.active_circuit) else {
            return;
        };

        workspace.testbenches = self.testbenches.clone();
        workspace.selected_testbench = self.selected_testbench;
        workspace.specs = self.specs.clone();
        workspace.selected_spec = self.selected_spec;
        workspace.derived_columns = self.derived_columns.clone();
        workspace.candidates = self.candidates.clone();
        workspace.output_prepared_specs = self.output_prepared_specs.clone();
        workspace.output_candidates = self.output_candidates.clone();
        workspace.output_results = self.output_results.clone();
        workspace.output_results_table = self.output_results_table.clone();
    }

    fn load_active_macro_workspace(&mut self) {
        self.ensure_macro_workspace_count();
        let Some(workspace) = self.macro_workspaces.get(self.active_circuit).cloned() else {
            return;
        };

        self.testbenches = workspace.testbenches;
        self.selected_testbench = workspace
            .selected_testbench
            .filter(|index| *index < self.testbenches.len());
        self.specs = workspace.specs;
        self.selected_spec = workspace
            .selected_spec
            .filter(|index| *index < self.specs.len());
        self.derived_columns = workspace.derived_columns;
        self.candidates = workspace.candidates;
        self.output_prepared_specs = workspace.output_prepared_specs;
        self.output_candidates = workspace.output_candidates;
        self.output_results = workspace.output_results;
        self.output_results_table = workspace.output_results_table;
        self.renaming_testbench = None;
        self.renaming_spec = None;
        self.ensure_testbench_dut_macros();
    }

    fn ensure_macro_workspace_count(&mut self) {
        while self.macro_workspaces.len() < self.circuits.len() {
            self.macro_workspaces.push(GuiMacroWorkspace::default());
        }
        self.macro_workspaces.truncate(self.circuits.len());
    }

    fn load_active_circuit_document(&mut self) {
        let Some(circuit) = self.circuits.get_mut(self.active_circuit) else {
            return;
        };

        self.canvas_instances = circuit.canvas_instances.clone();
        self.label_pins = circuit.label_pins.clone();
        self.macro_ports = circuit.macro_ports.clone();
        self.connections = circuit.connections.clone();
        self.next_instance_id = circuit.next_instance_id;
        self.next_label_pin_id = circuit.next_label_pin_id;
        self.next_macro_port_id = circuit.next_macro_port_id;
        self.selected_instance_id = None;
        self.selected_endpoint = None;
        self.pending_connection = None;
    }

    fn selected_instance_mut(&mut self) -> Option<&mut CanvasInstance> {
        let id = self.selected_instance_id?;

        self.canvas_instances
            .iter_mut()
            .find(|instance| instance.id == id)
    }

    fn selected_testbench_document(&self) -> Option<&GuiTestbenchDocument> {
        let index = self.selected_testbench?;

        self.testbenches.get(index)
    }

    fn selected_spec_document(&self) -> Option<&GuiSpecDocument> {
        let index = self.selected_spec?;

        self.specs.get(index)
    }

    fn show_insert_block_window(&mut self, ctx: &egui::Context) {
        if !self.show_insert_block_window {
            return;
        }

        let mut is_open = self.show_insert_block_window;
        let mut block_to_insert = None;
        let mut close_requested = false;
        let macro_blocks = self.available_macro_block_views();

        egui::Window::new("Insert block")
            .open(&mut is_open)
            .fixed_size(egui::vec2(520.0, 340.0))
            .resizable(false)
            .show(ctx, |ui| {
                if let Some(error) = &self.load_error {
                    ui.label(format!("Failed to load primitive catalog: {error}"));
                }

                if let Some(error) = &self.macro_load_error {
                    ui.label(format!("Failed to load macro catalog: {error}"));
                }

                if self.insert_block_selection.is_none() {
                    self.insert_block_selection = self
                        .catalog
                        .as_ref()
                        .and_then(|catalog| {
                            catalog.list().first().map(|primitive| {
                                GuiInsertBlockSelection::Primitive(primitive.name.clone())
                            })
                        })
                        .or_else(|| {
                            macro_blocks.first().map(|macro_block| {
                                GuiInsertBlockSelection::Macro(macro_block.name.clone())
                            })
                        });
                }

                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(180.0);
                        ui.heading("Primitives");
                        ui.separator();

                        egui::ScrollArea::vertical()
                            .id_salt("insert_primitive_catalog_scroll")
                            .max_height(220.0)
                            .show(ui, |ui| {
                                if let Some(catalog) = &self.catalog {
                                    for primitive in catalog.list() {
                                        ui.selectable_value(
                                            &mut self.insert_block_selection,
                                            Some(GuiInsertBlockSelection::Primitive(
                                                primitive.name.clone(),
                                            )),
                                            &primitive.name,
                                        );
                                    }
                                } else {
                                    ui.label("No primitives loaded");
                                }
                            });

                        ui.separator();
                        ui.heading("Macros");
                        egui::ScrollArea::vertical()
                            .id_salt("insert_macro_catalog_scroll")
                            .max_height(90.0)
                            .show(ui, |ui| {
                                if macro_blocks.is_empty() {
                                    ui.label("No insertable macros");
                                }
                                for macro_block in &macro_blocks {
                                    ui.selectable_value(
                                        &mut self.insert_block_selection,
                                        Some(GuiInsertBlockSelection::Macro(
                                            macro_block.name.clone(),
                                        )),
                                        &macro_block.name,
                                    );
                                }
                            });
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.set_width(220.0);
                        ui.heading("Preview");
                        ui.separator();

                        egui::ScrollArea::vertical()
                            .id_salt("insert_block_preview_scroll")
                            .max_height(250.0)
                            .show(ui, |ui| match &self.insert_block_selection {
                                Some(GuiInsertBlockSelection::Primitive(name)) => {
                                    if let Some(primitive) =
                                        self.catalog.as_ref().and_then(|catalog| catalog.get(name))
                                    {
                                        show_primitive_details(ui, primitive);
                                        ui.separator();
                                        draw_primitive_preview(ui, primitive);
                                    }
                                }
                                Some(GuiInsertBlockSelection::Macro(name)) => {
                                    if let Some(macro_block) = macro_blocks
                                        .iter()
                                        .find(|macro_block| &macro_block.name == name)
                                    {
                                        show_macro_block_details(ui, macro_block);
                                        ui.separator();
                                        draw_macro_block_preview(ui, macro_block);
                                    }
                                }
                                None => {
                                    ui.label("Select a block");
                                }
                            });
                    });
                });

                ui.separator();
                ui.horizontal(|ui| {
                    let can_insert = self.insert_block_selection.is_some();
                    if ui
                        .add_enabled(can_insert, egui::Button::new("Insert"))
                        .clicked()
                    {
                        block_to_insert = self.insert_block_selection.clone();
                    }
                    if ui.button("Cancel").clicked() {
                        close_requested = true;
                    }
                });
            });

        if let Some(block) = block_to_insert {
            self.add_canvas_instance(block.into_block_ref());
            is_open = false;
        } else if close_requested {
            is_open = false;
        }

        self.show_insert_block_window = is_open;
    }

    fn selected_label_pin_mut(&mut self) -> Option<&mut CanvasLabelPin> {
        let Some(CanvasEndpoint::LabelPin { label_id }) = self.selected_endpoint.as_ref() else {
            return None;
        };
        let label_id = *label_id;

        self.label_pins
            .iter_mut()
            .find(|label_pin| label_pin.id == label_id)
    }

    fn selected_macro_port_mut(&mut self) -> Option<&mut CanvasMacroPort> {
        let Some(CanvasEndpoint::MacroPort { port_id }) = self.selected_endpoint.as_ref() else {
            return None;
        };
        let port_id = *port_id;

        self.macro_ports
            .iter_mut()
            .find(|macro_port| macro_port.id == port_id)
    }

    fn add_canvas_instance(&mut self, block: GuiBlockRef) {
        let block = match block {
            GuiBlockRef::Macro { name } => GuiBlockRef::Macro {
                name: self.import_macro_for_project(&name),
            },
            block => block,
        };
        let offset = 28.0 * self.canvas_instances.len() as f32;
        let id = self.next_instance_id;
        let instance_name = next_available_instance_name(&self.canvas_instances);

        self.canvas_instances.push(CanvasInstance {
            id,
            instance_name,
            block,
            position: egui::pos2(40.0 + offset, 40.0 + offset),
            orientation: GuiOrientation::R0,
        });

        self.selected_instance_id = Some(id);
        self.selected_endpoint = None;
        self.pending_connection = None;
        self.next_instance_id += 1;
    }

    fn import_macro_for_project(&mut self, macro_name: &str) -> String {
        if self
            .circuits
            .iter()
            .any(|circuit| circuit.name == macro_name)
        {
            return macro_name.to_string();
        }

        let mut importing = HashSet::new();
        self.import_macro_for_project_inner(macro_name, &mut importing)
            .unwrap_or_else(|| macro_name.to_string())
    }

    fn import_macro_for_project_inner(
        &mut self,
        macro_name: &str,
        importing: &mut HashSet<String>,
    ) -> Option<String> {
        if self
            .circuits
            .iter()
            .any(|circuit| circuit.name == macro_name)
        {
            return Some(macro_name.to_string());
        }
        if !importing.insert(macro_name.to_string()) {
            return Some(macro_name.to_string());
        }

        let macro_model = self
            .macro_catalog
            .as_ref()
            .and_then(|catalog| catalog.get(macro_name))
            .cloned()?;

        for instance in &macro_model.circuit.instances {
            if let Some(child_macro_name) = instance.macro_name() {
                self.import_macro_for_project_inner(child_macro_name, importing);
            }
        }

        let document = GuiCircuitDocument::from_macro_model(&macro_model);
        let imported_name = document.name.clone();
        self.circuits.push(document);
        self.macro_workspaces.push(GuiMacroWorkspace::default());
        importing.remove(macro_name);

        Some(imported_name)
    }

    fn add_label_pin(&mut self) {
        let offset = 24.0 * self.label_pins.len() as f32;
        let id = self.next_label_pin_id;

        self.label_pins.push(CanvasLabelPin {
            id,
            name: format!("NET{id}"),
            position: egui::pos2(80.0 + offset, 80.0 + offset),
        });

        self.selected_instance_id = None;
        self.selected_endpoint = Some(CanvasEndpoint::LabelPin { label_id: id });
        self.pending_connection = None;
        self.next_label_pin_id += 1;
    }

    fn add_macro_port(&mut self) {
        let offset = 24.0 * self.macro_ports.len() as f32;
        let id = self.next_macro_port_id;
        let role = GuiMacroPortRole::Inout;

        self.macro_ports.push(CanvasMacroPort {
            id,
            name: format!("PORT{id}"),
            role,
            symbol_side: default_symbol_side_for_role(role),
            symbol_offset: 0.5,
            position: egui::pos2(112.0 + offset, 112.0 + offset),
        });

        self.selected_instance_id = None;
        self.selected_endpoint = Some(CanvasEndpoint::MacroPort { port_id: id });
        self.pending_connection = None;
        self.next_macro_port_id += 1;
    }

    fn add_testbench(&mut self) {
        let index = self.testbenches.len() + 1;
        let dut_macro = self.default_dut_macro_name();

        self.testbenches.push(GuiTestbenchDocument {
            name: format!("tb_{index}"),
            dut_macro,
            dut_position: default_dut_position(),
            small_signal_mode: GuiSmallSignalMode::CompactWhenAvailable,
            compact_outputs: Vec::new(),
            elements: Vec::new(),
            connections: Vec::new(),
            selected_endpoint: None,
            pending_connection: None,
            extra_body: String::new(),
            next_element_id: 1,
        });
        self.selected_testbench = Some(self.testbenches.len() - 1);
        self.active_document = ActiveDocument::Testbench;
        self.bottom_view = BottomView::Testbenches;
    }

    fn add_spec(&mut self) {
        let index = self.specs.len() + 1;
        let testbench = self
            .selected_testbench
            .and_then(|index| self.testbenches.get(index))
            .or_else(|| self.testbenches.first())
            .map(|testbench| testbench.name.clone())
            .unwrap_or_default();

        self.specs.push(GuiSpecDocument {
            name: format!("spec_{index}"),
            source_kind: GuiSpecSourceKind::TransferFunction,
            testbench,
            input: String::new(),
            output: String::new(),
            node: String::new(),
            min: String::new(),
            max: String::new(),
            parameter_map: Vec::new(),
        });
        self.selected_spec = Some(self.specs.len() - 1);
        self.active_document = ActiveDocument::Spec;
    }

    fn default_dut_macro_name(&self) -> String {
        self.circuits
            .get(self.active_circuit)
            .or_else(|| self.circuits.first())
            .map(|circuit| circuit.name.clone())
            .unwrap_or_default()
    }

    fn macro_names(&self) -> Vec<String> {
        self.circuits
            .iter()
            .map(|circuit| circuit.name.clone())
            .collect()
    }

    fn available_macro_block_views(&self) -> Vec<GuiDutMacroView> {
        let active_name = self
            .circuits
            .get(self.active_circuit)
            .map(|document| document.name.as_str());
        let mut macros = Vec::new();

        for document in &self.circuits {
            if Some(document.name.as_str()) == active_name {
                continue;
            }
            macros.push(GuiDutMacroView::from_circuit_document(document));
        }

        if let Some(catalog) = &self.macro_catalog {
            for macro_model in catalog.list() {
                if Some(macro_model.name.as_str()) == active_name {
                    continue;
                }
                if macros.iter().any(|view| view.name == macro_model.name) {
                    continue;
                }
                macros.push(GuiDutMacroView::from_macro_model(macro_model));
            }
        }

        macros
    }

    fn runtime_macro_catalog(&self, macro_models: Vec<MacroModel>) -> MacroCatalog {
        let mut catalog = MacroCatalog::new();

        if let Some(library_catalog) = &self.macro_catalog {
            for macro_model in library_catalog.list() {
                catalog.register(macro_model.clone());
            }
        }

        for macro_model in macro_models {
            catalog.register(macro_model);
        }

        catalog
    }

    fn delete_selected_canvas_item(&mut self) {
        if self.delete_selected_macro_port() {
            return;
        }

        self.delete_selected_instance();
    }

    fn delete_selected_instance(&mut self) {
        let Some(instance_id) = self.selected_instance_id else {
            return;
        };

        self.canvas_instances
            .retain(|instance| instance.id != instance_id);
        self.connections
            .retain(|connection| !connection.references_instance(instance_id));

        if self
            .selected_endpoint
            .as_ref()
            .is_some_and(|endpoint| endpoint.references_instance(instance_id))
        {
            self.selected_endpoint = None;
        }

        if self
            .pending_connection
            .as_ref()
            .is_some_and(|endpoint| endpoint.references_instance(instance_id))
        {
            self.pending_connection = None;
        }

        self.selected_instance_id = None;
    }

    fn delete_selected_macro_port(&mut self) -> bool {
        let Some(CanvasEndpoint::MacroPort { port_id }) = self.selected_endpoint.as_ref() else {
            return false;
        };
        let port_id = *port_id;

        self.macro_ports
            .retain(|macro_port| macro_port.id != port_id);
        self.connections
            .retain(|connection| !connection.references_macro_port(port_id));

        if self
            .pending_connection
            .as_ref()
            .is_some_and(|endpoint| endpoint.references_macro_port(port_id))
        {
            self.pending_connection = None;
        }

        self.selected_endpoint = None;
        true
    }

    fn rotate_selected_canvas_item(&mut self) {
        match self.active_document {
            ActiveDocument::Circuit => {
                if let Some(instance) = self.selected_instance_mut() {
                    instance.orientation = instance.orientation.rotated_clockwise();
                }
            }
            ActiveDocument::Testbench => {
                let Some(selected_index) = self.selected_testbench else {
                    return;
                };
                let Some(testbench) = self.testbenches.get_mut(selected_index) else {
                    return;
                };
                let Some(TestbenchEndpoint::ElementPin { element_id, .. }) =
                    testbench.selected_endpoint.as_ref()
                else {
                    return;
                };
                let element_id = *element_id;
                if let Some(element) = testbench
                    .elements
                    .iter_mut()
                    .find(|element| element.id == element_id)
                {
                    element.orientation = element.orientation.rotated_clockwise();
                }
            }
            ActiveDocument::Spec | ActiveDocument::Candidates => {}
        }
    }

    fn delete_selected_testbench_element(&mut self) {
        let Some(selected_index) = self.selected_testbench else {
            return;
        };
        let Some(testbench) = self.testbenches.get_mut(selected_index) else {
            return;
        };
        let Some(TestbenchEndpoint::ElementPin { element_id, .. }) =
            testbench.selected_endpoint.as_ref()
        else {
            return;
        };
        let element_id = *element_id;
        let Some(element_index) = testbench
            .elements
            .iter()
            .position(|element| element.id == element_id)
        else {
            return;
        };

        testbench.remove_element(element_index);
    }

    fn run_mna_from_canvas(&self) -> String {
        let Some(catalog) = &self.catalog else {
            return "Cannot run MNA: primitive catalog is not loaded".to_string();
        };

        if self.canvas_instances.is_empty() {
            return "Cannot run MNA: the canvas has no instances".to_string();
        }

        let circuit = self.build_circuit_from_canvas();
        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let circuit_path = output_dir.join("generated_circuit.json");

        if let Err(error) = std::fs::create_dir_all(&output_dir) {
            return format!(
                "Cannot run MNA: failed to create output directory '{}'\n\n{error}",
                output_dir.display()
            );
        }

        if let Err(error) = save_circuit(&circuit_path, &circuit) {
            return format!(
                "Cannot run MNA: failed to save generated circuit JSON '{}'\n\n{error:?}",
                circuit_path.display()
            );
        }

        match analyze_circuit_mna(&circuit, catalog, &output_dir, false) {
            Ok(analysis) => {
                format_mna_output(&CircuitMnaOutput::from_analysis(&analysis), &circuit_path)
            }
            Err(error) => format!(
                "MNA failed for macro '{}'\n\nGenerated circuit JSON: {}\n\n{error:?}\n\nGenerated circuit summary:\n{}",
                circuit.name,
                circuit_path.display(),
                format_circuit_summary(&circuit)
            ),
        }
    }

    fn save_active_macro_model(&mut self) -> String {
        self.save_active_circuit_document();

        let macro_model = match self.build_macro_from_canvas() {
            Ok(macro_model) => macro_model,
            Err(error) => return format!("Cannot save macro: {error}"),
        };
        let output_dir = std::env::temp_dir()
            .join("sstadex-gui-mna")
            .join("gui_macros")
            .join(&macro_model.name);
        let macro_path = output_dir.join("macro.json");

        if let Err(error) = save_macro_model(&macro_path, &macro_model) {
            return format!(
                "Cannot save macro: failed to write macro JSON '{}'\n\n{error:?}",
                macro_path.display()
            );
        }

        format!(
            "Saved macro\n\nMacro JSON: {}\nName: {}\nSubckt: {}\nPorts: {}\nInstances: {}\nConnections: {}",
            macro_path.display(),
            macro_model.name,
            macro_model.subckt_name,
            macro_model.ports.len(),
            macro_model.circuit.instances.len(),
            macro_model.circuit.connections.len()
        )
    }

    fn save_flow_inputs(&mut self) -> String {
        self.save_active_circuit_document();
        self.save_active_macro_workspace();

        let Some(catalog) = &self.catalog else {
            return "Cannot save flow inputs: primitive catalog is not loaded".to_string();
        };
        let Some(document) = self.circuits.get(self.active_circuit) else {
            return "Cannot save flow inputs: no active macro document".to_string();
        };

        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let macro_dir = output_dir.join("gui_macros");
        let testbench_path = output_dir.join("gui_testbenches.json");
        let specs_path = output_dir.join("gui_specs.json");
        let candidates_path = output_dir.join("gui_candidates.json");

        if let Err(error) = std::fs::create_dir_all(&output_dir) {
            return format!(
                "Cannot save flow inputs: failed to create output directory '{}'\n\n{error}",
                output_dir.display()
            );
        }

        let macro_models = match self.build_macro_models() {
            Ok(macro_models) => macro_models,
            Err(error) => return format!("Cannot save flow inputs: {error}"),
        };
        let testbenches = match gui_testbenches_to_specs(&self.testbenches, &self.circuits) {
            Ok(testbenches) => testbenches,
            Err(error) => return format!("Cannot save flow inputs: {error}"),
        };
        let specs = match gui_specs_to_exploration_specs(&self.specs, &testbenches) {
            Ok(specs) => specs,
            Err(error) => return format!("Cannot save flow inputs: {error}"),
        };
        let macro_blocks = self.available_macro_block_views();
        let candidates =
            match gui_candidate_input(&self.candidates, document, catalog, &macro_blocks) {
                Ok(candidates) => candidates,
                Err(error) => return format!("Cannot save flow inputs: {error}"),
            };

        for macro_model in &macro_models {
            let macro_path = macro_dir.join(&macro_model.name).join("macro.json");
            if let Err(error) = save_macro_model(&macro_path, macro_model) {
                return format!(
                    "Cannot save flow inputs: failed to write macro JSON '{}'\n\n{error:?}",
                    macro_path.display()
                );
            }
        }

        if let Err(error) = save_testbenches(&testbench_path, &testbenches) {
            return format!(
                "Cannot save flow inputs: failed to write testbench JSON '{}'\n\n{error:?}",
                testbench_path.display()
            );
        }

        if let Err(error) = save_exploration_specs(&specs_path, &specs) {
            return format!(
                "Cannot save flow inputs: failed to write specs JSON '{}'\n\n{error:?}",
                specs_path.display()
            );
        }

        if let Err(error) = save_exploration_candidates(&candidates_path, &candidates) {
            return format!(
                "Cannot save flow inputs: failed to write candidates JSON '{}'\n\n{error:?}",
                candidates_path.display()
            );
        }

        format!(
            "Saved flow inputs\n\nMacros: {}\nTestbenches: {}\nSpecs: {}\nCandidate axes: {}\nMacro dir: {}\nTestbenches: {}\nSpecs: {}\nCandidates: {}",
            macro_models.len(),
            testbenches.len(),
            specs.len(),
            candidates.axes.len(),
            macro_dir.display(),
            testbench_path.display(),
            specs_path.display(),
            candidates_path.display()
        )
    }

    fn run_selected_testbench_mna(&mut self) -> String {
        self.save_active_circuit_document();

        let Some(selected_index) = self.selected_testbench else {
            return "Cannot run testbench MNA: no testbench selected".to_string();
        };
        let Some(catalog) = &self.catalog else {
            return "Cannot run testbench MNA: primitive catalog is not loaded".to_string();
        };

        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let macro_dir = output_dir.join("gui_macros");
        let testbench_path = output_dir.join("gui_testbenches.json");
        let mna_dir = output_dir.join("testbench_mna");

        if let Err(error) = std::fs::create_dir_all(&output_dir) {
            return format!(
                "Cannot run testbench MNA: failed to create output directory '{}'\n\n{error}",
                output_dir.display()
            );
        }

        let macro_models = match self.build_macro_models() {
            Ok(macro_models) => macro_models,
            Err(error) => return format!("Cannot run testbench MNA: {error}"),
        };
        let testbenches = match gui_testbenches_to_specs(&self.testbenches, &self.circuits) {
            Ok(testbenches) => testbenches,
            Err(error) => return format!("Cannot run testbench MNA: {error}"),
        };
        let Some(testbench) = testbenches.get(selected_index) else {
            return "Cannot run testbench MNA: selected testbench no longer exists".to_string();
        };
        let Some(gui_testbench) = self.testbenches.get(selected_index) else {
            return "Cannot run testbench MNA: selected GUI testbench no longer exists".to_string();
        };
        let gui_mode = gui_testbench.small_signal_mode;
        let mode = macro_small_signal_mode(gui_mode);

        for macro_model in &macro_models {
            let macro_path = macro_dir.join(&macro_model.name).join("macro.json");
            if let Err(error) = save_macro_model(&macro_path, macro_model) {
                return format!(
                    "Cannot run testbench MNA: failed to write macro JSON '{}'\n\n{error:?}",
                    macro_path.display()
                );
            }
        }

        if let Err(error) = save_testbenches(&testbench_path, &testbenches) {
            return format!(
                "Cannot run testbench MNA: failed to write testbench JSON '{}'\n\n{error:?}",
                testbench_path.display()
            );
        }

        let macro_catalog = self.runtime_macro_catalog(macro_models);

        match analyze_macro_testbench_mna_with_mode(
            testbench,
            catalog,
            &macro_catalog,
            &mna_dir,
            false,
            mode,
        ) {
            Ok(analysis) => {
                let output = CircuitMnaOutput::from_analysis(&analysis);
                self.output_netlist = analysis.small_signal_netlist.clone();
                self.output_equations = format_equations_output(&output);
                self.output_artifacts =
                    format_artifacts_output(&output, &macro_dir, &testbench_path, gui_mode);

                format_testbench_mna_output(
                    &output,
                    &analysis.small_signal_netlist,
                    &macro_dir,
                    &testbench_path,
                    gui_mode,
                )
            }
            Err(error) => format!(
                "Testbench MNA failed for '{}'\n\nMacro dir: {}\nTestbenches: {}\n\n{error:?}",
                testbench.name,
                macro_dir.display(),
                testbench_path.display()
            ),
        }
    }

    fn choose_open_project_path(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_directory(project_dialog_dir(&self.project_path))
            .add_filter("SSTADEx GUI project", &["json"])
            .pick_file()
        {
            self.project_path = path.display().to_string();
        }
    }

    fn choose_save_project_path(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_directory(project_dialog_dir(&self.project_path))
            .set_file_name(project_dialog_file_name(&self.project_path))
            .add_filter("SSTADEx GUI project", &["json"])
            .save_file()
        {
            self.project_path = path.display().to_string();
        }
    }

    fn save_gui_project(&mut self) -> String {
        self.save_active_circuit_document();
        self.save_active_macro_workspace();

        let project_path = match project_path_from_input(&self.project_path) {
            Ok(path) => path,
            Err(error) => return format!("Cannot save project: {error}"),
        };
        let output_dir = project_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(default_project_dir);
        let circuit_path = output_dir.join("generated_circuit.json");

        if let Err(error) = std::fs::create_dir_all(&output_dir) {
            return format!(
                "Cannot save project: failed to create output directory '{}'\n\n{error}",
                output_dir.display()
            );
        }

        let project = self.gui_project();
        let project_content = match serde_json::to_string_pretty(&project) {
            Ok(content) => content,
            Err(error) => {
                return format!("Cannot save project: failed to serialize GUI project\n\n{error}");
            }
        };

        if let Err(error) = std::fs::write(&project_path, project_content) {
            return format!(
                "Cannot save project: failed to write GUI project '{}'\n\n{error}",
                project_path.display()
            );
        }

        let circuit = self.build_circuit_from_canvas();
        if let Err(error) = save_circuit(&circuit_path, &circuit) {
            return format!(
                "Saved GUI project but failed to export generated circuit JSON '{}'\n\n{error:?}",
                circuit_path.display()
            );
        }

        format!(
            "Saved macro project\n\nProject: {}\nGenerated circuit JSON: {}\nInstances: {}\nConnections: {}",
            project_path.display(),
            circuit_path.display(),
            self.canvas_instances.len(),
            self.connections.len()
        )
    }

    fn open_gui_project(&mut self) -> String {
        let project_path = match project_path_from_input(&self.project_path) {
            Ok(path) => path,
            Err(error) => return format!("Cannot open project: {error}"),
        };
        let content = match std::fs::read_to_string(&project_path) {
            Ok(content) => content,
            Err(error) => {
                return format!(
                    "Cannot open project: failed to read GUI project '{}'\n\n{error}",
                    project_path.display()
                );
            }
        };

        let project: GuiProject = match serde_json::from_str(&content) {
            Ok(project) => project,
            Err(error) => {
                return format!(
                    "Cannot open project: invalid GUI project '{}'\n\n{error}",
                    project_path.display()
                );
            }
        };

        if !(4..=9).contains(&project.version) {
            return format!(
                "Cannot open project: unsupported GUI project version {}",
                project.version
            );
        }

        self.apply_gui_project(project);

        format!(
            "Opened macro project\n\nProject: {}\nInstances: {}\nLab pins: {}\nConnections: {}",
            project_path.display(),
            self.canvas_instances.len(),
            self.label_pins.len(),
            self.connections.len()
        )
    }

    fn gui_project(&self) -> GuiProject {
        GuiProject {
            version: 10,
            active_circuit: self.active_circuit,
            circuits: self
                .circuits
                .iter()
                .map(GuiProjectCircuit::from_circuit_document)
                .collect(),
            macro_workspaces: self
                .macro_workspaces
                .iter()
                .map(GuiProjectMacroWorkspace::from_macro_workspace)
                .collect(),
            selected_testbench: self.selected_testbench,
            testbenches: self
                .testbenches
                .iter()
                .map(GuiProjectTestbench::from_testbench_document)
                .collect(),
            selected_spec: self.selected_spec,
            specs: self
                .specs
                .iter()
                .map(GuiProjectSpec::from_spec_document)
                .collect(),
            derived_columns: self
                .derived_columns
                .iter()
                .map(GuiProjectDerivedColumn::from_derived_column_document)
                .collect(),
            candidates: GuiProjectCandidateDocument::from_candidate_document(&self.candidates),
        }
    }

    fn apply_gui_project(&mut self, project: GuiProject) {
        let mut circuits = project
            .circuits
            .into_iter()
            .map(GuiProjectCircuit::into_circuit_document)
            .collect::<Vec<_>>();

        if circuits.is_empty() {
            circuits.push(GuiCircuitDocument::empty("macro_1"));
        }

        self.circuits = circuits;
        self.macro_workspaces = if project.macro_workspaces.is_empty() {
            let mut workspaces = (0..self.circuits.len())
                .map(|_| GuiMacroWorkspace::default())
                .collect::<Vec<_>>();
            if let Some(workspace) =
                workspaces.get_mut(project.active_circuit.min(self.circuits.len() - 1))
            {
                workspace.testbenches = project
                    .testbenches
                    .into_iter()
                    .map(GuiProjectTestbench::into_testbench_document)
                    .collect();
                workspace.selected_testbench = project
                    .selected_testbench
                    .filter(|index| *index < workspace.testbenches.len());
                workspace.specs = project
                    .specs
                    .into_iter()
                    .map(GuiProjectSpec::into_spec_document)
                    .collect();
                workspace.selected_spec = project
                    .selected_spec
                    .filter(|index| *index < workspace.specs.len());
                workspace.derived_columns = project
                    .derived_columns
                    .into_iter()
                    .map(GuiProjectDerivedColumn::into_derived_column_document)
                    .collect();
                workspace.candidates = project.candidates.into_candidate_document();
            }
            workspaces
        } else {
            let mut workspaces = project
                .macro_workspaces
                .into_iter()
                .map(GuiProjectMacroWorkspace::into_macro_workspace)
                .collect::<Vec<_>>();
            while workspaces.len() < self.circuits.len() {
                workspaces.push(GuiMacroWorkspace::default());
            }
            workspaces.truncate(self.circuits.len());
            workspaces
        };
        self.active_circuit = project.active_circuit.min(self.circuits.len() - 1);
        self.ensure_testbench_dut_macros();
        self.load_active_circuit_document();
        self.load_active_macro_workspace();

        self.selected_instance_id = None;
        self.selected_endpoint = None;
        self.pending_connection = None;
    }

    fn ensure_testbench_dut_macros(&mut self) {
        let default_dut = self.default_dut_macro_name();
        let macro_names = self.macro_names();
        ensure_testbench_dut_macros_for(&mut self.testbenches, &macro_names, &default_dut);
        for workspace in &mut self.macro_workspaces {
            ensure_testbench_dut_macros_for(&mut workspace.testbenches, &macro_names, &default_dut);
        }
    }

    fn build_macro_from_canvas(&self) -> Result<MacroModel, String> {
        let Some(document) = self.circuits.get(self.active_circuit) else {
            return Err("no active macro document".to_string());
        };
        self.build_macro_from_document(document)
    }

    fn build_macro_models(&self) -> Result<Vec<MacroModel>, String> {
        self.circuits
            .iter()
            .map(|document| self.build_macro_from_document(document))
            .collect()
    }

    fn build_macro_from_document(
        &self,
        document: &GuiCircuitDocument,
    ) -> Result<MacroModel, String> {
        let name = document.name.trim();
        if name.is_empty() {
            return Err("macro name cannot be empty".to_string());
        }
        let subckt_name = document.subckt_name.trim();
        if subckt_name.is_empty() {
            return Err("macro subckt name cannot be empty".to_string());
        }
        let macro_ports = document_macro_ports(document);
        if let Some(index) = macro_ports
            .iter()
            .position(|port| macro_port_name(*port).trim().is_empty())
        {
            return Err(format!("macro port {} name cannot be empty", index + 1));
        }

        let mut macro_model = MacroModel::new(
            name,
            macro_ports
                .iter()
                .map(|port| {
                    MacroPort::new(
                        macro_port_name(*port).trim(),
                        macro_port_role(macro_port_role_value(*port)),
                    )
                })
                .collect(),
            self.build_circuit_from_document(document),
        );
        macro_model.subckt_name = subckt_name.to_string();
        macro_model.small_signal = document.small_signal.clone();
        macro_model.metadata = MacroMetadata {
            version: "1.0".to_string(),
            description: Some("Generated by SSTADEx GUI".to_string()),
        };
        macro_model.symbol = Some(MacroSymbol {
            pins: macro_ports
                .iter()
                .map(|port| MacroSymbolPin {
                    name: macro_port_name(*port).trim().to_string(),
                    side: macro_port_symbol_side(*port),
                    offset: macro_port_symbol_offset(*port).clamp(0.0, 1.0),
                })
                .collect(),
        });

        Ok(macro_model)
    }

    fn build_circuit_from_canvas(&self) -> Circuit {
        let Some(document) = self.circuits.get(self.active_circuit) else {
            return Circuit::new("macro_1");
        };
        self.build_circuit_from_document(document)
    }

    fn build_circuit_from_document(&self, document: &GuiCircuitDocument) -> Circuit {
        let circuit_name = document.name.as_str();
        let mut circuit = Circuit::new(circuit_name);

        for instance in &document.canvas_instances {
            let instance_name = exported_instance_name(instance);
            match &instance.block {
                GuiBlockRef::Primitive { name } => {
                    circuit.add_instance(Instance::primitive(instance_name, name.clone()));
                }
                GuiBlockRef::Macro { name } => {
                    circuit.add_instance(Instance::macro_instance(instance_name, name.clone()));
                }
            }
        }

        for (net_index, endpoints) in connected_endpoint_groups(&document.connections)
            .into_iter()
            .enumerate()
        {
            let net = canvas_net_name(&endpoints, &document.label_pins, &document.macro_ports)
                .unwrap_or_else(|| format!("N{}", net_index + 1));

            for endpoint in endpoints {
                if let CanvasEndpoint::PrimitivePin {
                    instance_id,
                    pin_name,
                } = endpoint
                {
                    let instance_name = document
                        .canvas_instances
                        .iter()
                        .find(|instance| instance.id == instance_id)
                        .map(exported_instance_name)
                        .unwrap_or_else(|| circuit_instance_id(instance_id));

                    circuit.connect(Connection::new(
                        PinRef::new(instance_name, pin_name),
                        net.clone(),
                    ));
                }
            }
        }

        circuit
    }

    fn show_logs_ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!(
            "Selected instance: {}",
            self.selected_instance_id
                .map(|id| id.to_string())
                .unwrap_or_else(|| "none".to_string())
        ));
        ui.label(format!(
            "Selected endpoint: {}",
            self.selected_endpoint
                .as_ref()
                .map(format_endpoint)
                .unwrap_or_else(|| "none".to_string())
        ));
        ui.label(format!(
            "Pending connection: {}",
            self.pending_connection
                .as_ref()
                .map(format_endpoint)
                .unwrap_or_else(|| "none".to_string())
        ));
        ui.label(format!("Connections: {}", self.connections.len()));
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(&self.output_log);
        });
    }

    fn show_testbenches_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Add testbench").clicked() {
                self.add_testbench();
            }
            if ui.button("Save testbenches").clicked() {
                self.output_log = self.save_gui_testbenches();
                self.bottom_view = BottomView::Logs;
            }
        });
        ui.separator();

        if self.testbenches.is_empty() {
            ui.label("No testbenches yet");
            return;
        }

        let Some(selected_index) = self.selected_testbench else {
            ui.label("Select a testbench from the project browser");
            return;
        };
        let Some(testbench) = self.testbenches.get(selected_index) else {
            self.selected_testbench = None;
            return;
        };

        ui.label(format!("Selected testbench: {}", testbench.name));
        ui.label(format!(
            "DUT macro: {}",
            display_optional_name(&testbench.dut_macro)
        ));
        ui.label(format!(
            "Small-signal mode: {}",
            testbench.small_signal_mode.label()
        ));
        ui.label(format!("Elements: {}", testbench.elements.len()));
        ui.label("Edit the selected testbench in the central panel.");
    }

    fn save_gui_testbenches(&self) -> String {
        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let output_path = output_dir.join("gui_testbenches.json");

        if let Err(error) = std::fs::create_dir_all(&output_dir) {
            return format!(
                "Cannot save testbenches: failed to create output directory '{}'\n\n{error}",
                output_dir.display()
            );
        }

        let testbenches = match gui_testbenches_to_specs(&self.testbenches, &self.circuits) {
            Ok(testbenches) => testbenches,
            Err(error) => return format!("Cannot save testbenches: {error}"),
        };

        match save_testbenches(&output_path, &testbenches) {
            Ok(()) => format!(
                "Saved {} macro-linked testbench(es)\n\nPath: {}",
                testbenches.len(),
                output_path.display()
            ),
            Err(error) => format!(
                "Cannot save testbenches: failed to write '{}'\n\n{error:?}",
                output_path.display()
            ),
        }
    }

    fn save_gui_specs(&self) -> String {
        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let output_path = output_dir.join("gui_specs.json");

        if let Err(error) = std::fs::create_dir_all(&output_dir) {
            return format!(
                "Cannot save specs: failed to create output directory '{}'\n\n{error}",
                output_dir.display()
            );
        }

        let testbenches = match gui_testbenches_to_specs(&self.testbenches, &self.circuits) {
            Ok(testbenches) => testbenches,
            Err(error) => return format!("Cannot save specs: {error}"),
        };
        let specs = match gui_specs_to_exploration_specs(&self.specs, &testbenches) {
            Ok(specs) => specs,
            Err(error) => return format!("Cannot save specs: {error}"),
        };

        match save_exploration_specs(&output_path, &specs) {
            Ok(()) => format!(
                "Saved {} exploration spec(s)\n\nPath: {}",
                specs.len(),
                output_path.display()
            ),
            Err(error) => format!(
                "Cannot save specs: failed to write '{}'\n\n{error:?}",
                output_path.display()
            ),
        }
    }

    fn generate_gui_candidates(&mut self) -> String {
        self.save_active_circuit_document();
        self.save_active_macro_workspace();

        let Some(catalog) = &self.catalog else {
            return "Cannot generate candidates: primitive catalog is not loaded".to_string();
        };
        let Some(document) = self.circuits.get(self.active_circuit) else {
            return "Cannot generate candidates: no active macro document".to_string();
        };

        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let candidates_path = output_dir.join("gui_candidates.json");

        let macro_blocks = self.available_macro_block_views();
        let candidate_input =
            match gui_candidate_input(&self.candidates, document, catalog, &macro_blocks) {
                Ok(candidate_input) => candidate_input,
                Err(error) => return format!("Cannot generate candidates: {error}"),
            };

        if let Err(error) = save_exploration_candidates(&candidates_path, &candidate_input) {
            return format!(
                "Cannot generate candidates: failed to write candidates JSON '{}'\n\n{error:?}",
                candidates_path.display()
            );
        }

        match build_filtered_candidates(
            &candidate_input.axes,
            &candidate_input.sets,
            &candidate_input.filters,
        ) {
            Ok(candidates) => {
                self.output_candidates = format_candidates_output(&candidates);
                self.output_artifacts = format!(
                    "Candidates: {}\nCandidate axes: {}\nPrimitive candidate sets: {}\nGenerated candidate points: {}",
                    candidates_path.display(),
                    candidate_input.axes.len(),
                    candidate_input.sets.len(),
                    candidates.len()
                );
                self.save_active_macro_workspace();

                format!(
                    "Generated {} candidate point(s)\n\nAxes: {}\nPrimitive sets: {}\nCandidates JSON: {}",
                    candidates.len(),
                    candidate_input.axes.len(),
                    candidate_input.sets.len(),
                    candidates_path.display()
                )
            }
            Err(error) => format!(
                "Generate candidates failed\n\nCandidates JSON: {}\n\n{error:?}",
                candidates_path.display()
            ),
        }
    }

    fn evaluate_gui_specs(&mut self) -> String {
        self.save_active_circuit_document();
        self.save_active_macro_workspace();
        self.output_results_table = None;
        self.save_active_macro_workspace();

        let Some(catalog) = &self.catalog else {
            return "Cannot evaluate specs: primitive catalog is not loaded".to_string();
        };

        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let macro_dir = output_dir.join("gui_macros");
        let testbench_path = output_dir.join("gui_testbenches.json");
        let specs_path = output_dir.join("gui_specs.json");
        let candidates_path = output_dir.join("gui_candidates.json");
        let results_path = output_dir.join("gui_results.csv");
        let prepared_dir = output_dir.join("prepared_specs");

        if let Err(error) = std::fs::create_dir_all(&prepared_dir) {
            return format!(
                "Cannot evaluate specs: failed to create output directory '{}'\n\n{error}",
                prepared_dir.display()
            );
        }

        let macro_models = match self.build_macro_models() {
            Ok(macro_models) => macro_models,
            Err(error) => return format!("Cannot evaluate specs: {error}"),
        };
        let testbenches = match gui_testbenches_to_specs(&self.testbenches, &self.circuits) {
            Ok(testbenches) => testbenches,
            Err(error) => return format!("Cannot evaluate specs: {error}"),
        };
        let specs = match gui_specs_to_exploration_specs(&self.specs, &testbenches) {
            Ok(specs) => specs,
            Err(error) => return format!("Cannot evaluate specs: {error}"),
        };
        let Some(document) = self.circuits.get(self.active_circuit) else {
            return "Cannot evaluate specs: no active macro document".to_string();
        };
        let macro_blocks = self.available_macro_block_views();
        let candidate_input =
            match gui_candidate_input(&self.candidates, document, catalog, &macro_blocks) {
                Ok(candidate_input) => candidate_input,
                Err(error) => return format!("Cannot evaluate specs: {error}"),
            };
        let gui_mode = match gui_specs_small_signal_mode(&self.specs, &self.testbenches) {
            Ok(mode) => mode,
            Err(error) => return format!("Cannot evaluate specs: {error}"),
        };
        let mode = macro_small_signal_mode(gui_mode);

        for macro_model in &macro_models {
            let macro_path = macro_dir.join(&macro_model.name).join("macro.json");
            if let Err(error) = save_macro_model(&macro_path, macro_model) {
                return format!(
                    "Cannot evaluate specs: failed to write macro JSON '{}'\n\n{error:?}",
                    macro_path.display()
                );
            }
        }

        if let Err(error) = save_testbenches(&testbench_path, &testbenches) {
            return format!(
                "Cannot evaluate specs: failed to write testbench JSON '{}'\n\n{error:?}",
                testbench_path.display()
            );
        }

        if let Err(error) = save_exploration_specs(&specs_path, &specs) {
            return format!(
                "Cannot evaluate specs: failed to write specs JSON '{}'\n\n{error:?}",
                specs_path.display()
            );
        }

        if let Err(error) = save_exploration_candidates(&candidates_path, &candidate_input) {
            return format!(
                "Cannot evaluate specs: failed to write candidates JSON '{}'\n\n{error:?}",
                candidates_path.display()
            );
        }

        let macro_catalog = self.runtime_macro_catalog(macro_models);

        let prepared_specs = match prepare_macro_testbench_specs_with_mode(
            &specs,
            catalog,
            &macro_catalog,
            &prepared_dir,
            mode,
        ) {
            Ok(prepared_specs) => prepared_specs,
            Err(error) => {
                return format!(
                    "Evaluate specs failed while preparing specs\n\nMacro dir: {}\nTestbenches: {}\nSpecs: {}\nCandidates: {}\nPrepared dir: {}\n\n{error:?}",
                    macro_dir.display(),
                    testbench_path.display(),
                    specs_path.display(),
                    candidates_path.display(),
                    prepared_dir.display()
                );
            }
        };

        self.output_prepared_specs = format_prepared_specs_output(&prepared_specs);

        let derived_columns = match gui_derived_columns_to_specs(&self.derived_columns) {
            Ok(derived_columns) => derived_columns,
            Err(error) => {
                return format!("Evaluate specs failed while reading derived columns\n\n{error}");
            }
        };

        match run_prepared_expression_flow_with_derived_columns(
            &candidate_input.axes,
            &candidate_input.sets,
            &candidate_input.filters,
            &prepared_specs,
            &derived_columns,
        ) {
            Ok(mut table) => {
                if let Err(error) = add_automatic_area_column(&mut table) {
                    return format!(
                        "Evaluate specs completed but failed to add automatic area column\n\n{error:?}"
                    );
                }
                self.output_results = format_exploration_table_output(&table);
                self.output_results_table = Some(table.clone());
                if let Err(error) = std::fs::write(&results_path, exploration_table_to_csv(&table))
                {
                    return format!(
                        "Evaluate specs completed but failed to write results CSV '{}'\n\n{error}",
                        results_path.display()
                    );
                }
                self.output_artifacts = format!(
                    "Macro dir: {}\nTestbenches: {}\nSpecs: {}\nCandidates: {}\nResults CSV: {}\nPrepared specs dir: {}\nSmall-signal mode: {}",
                    macro_dir.display(),
                    testbench_path.display(),
                    specs_path.display(),
                    candidates_path.display(),
                    results_path.display(),
                    prepared_dir.display(),
                    gui_mode.label()
                );
                self.save_active_macro_workspace();

                format!(
                    "Evaluated exploration specs\n\nRows kept: {}\nColumns: {}\nSpecs: {}\nSmall-signal mode: {}\nResults CSV: {}",
                    table.row_count,
                    table.columns.len(),
                    prepared_specs.len(),
                    gui_mode.label(),
                    results_path.display()
                )
            }
            Err(error) => format!(
                "Evaluate specs failed\n\nMacro dir: {}\nTestbenches: {}\nSpecs: {}\nCandidates: {}\nPrepared dir: {}\n\n{error:?}",
                macro_dir.display(),
                testbench_path.display(),
                specs_path.display(),
                candidates_path.display(),
                prepared_dir.display()
            ),
        }
    }

    fn evaluate_gui_hierarchy(&mut self) -> String {
        self.save_active_circuit_document();
        self.save_active_macro_workspace();
        self.output_results_table = None;
        self.save_active_macro_workspace();

        let Some(catalog) = self.catalog.clone() else {
            return "Cannot evaluate hierarchy: primitive catalog is not loaded".to_string();
        };
        let Some(top_document) = self.circuits.get(self.active_circuit) else {
            return "Cannot evaluate hierarchy: no active macro document".to_string();
        };

        let top_name = top_document.name.clone();
        let order = match self.hierarchy_evaluation_order(self.active_circuit) {
            Ok(order) => order,
            Err(error) => return format!("Cannot evaluate hierarchy: {error}"),
        };
        let macro_models = match self.build_macro_models() {
            Ok(macro_models) => macro_models,
            Err(error) => return format!("Cannot evaluate hierarchy: {error}"),
        };
        let macro_catalog = self.runtime_macro_catalog(macro_models);
        let macro_blocks = self.available_macro_block_views();
        let output_dir = std::env::temp_dir()
            .join("sstadex-gui-mna")
            .join("hierarchy");
        let mut results_by_macro: HashMap<String, ExplorationTable> = HashMap::new();
        let mut evaluated = Vec::new();

        for macro_index in order {
            let macro_name = self.circuits[macro_index].name.clone();
            let child_sets =
                match self.compact_candidate_sets_for_children(macro_index, &results_by_macro) {
                    Ok(child_sets) => child_sets,
                    Err(error) => {
                        return format!("Evaluate hierarchy failed for '{macro_name}': {error}");
                    }
                };
            let prepared_dir = output_dir.join(&macro_name).join("prepared_specs");
            let evaluation = match self.evaluate_workspace_index(
                macro_index,
                &catalog,
                &macro_catalog,
                &macro_blocks,
                &prepared_dir,
                child_sets,
            ) {
                Ok(evaluation) => evaluation,
                Err(error) => {
                    return format!("Evaluate hierarchy failed for '{macro_name}': {error}");
                }
            };

            if let Some(workspace) = self.macro_workspaces.get_mut(macro_index) {
                workspace.output_prepared_specs =
                    format_prepared_specs_output(&evaluation.prepared_specs);
                workspace.output_candidates =
                    format_hierarchy_candidates_output(&evaluation.candidate_input);
                workspace.output_results = format_exploration_table_output(&evaluation.table);
                workspace.output_results_table = Some(evaluation.table.clone());
            }

            results_by_macro.insert(macro_name.clone(), evaluation.table);
            evaluated.push(macro_name);
        }

        if let Some(top_table) = results_by_macro.get(&top_name).cloned() {
            self.output_results = format_exploration_table_output(&top_table);
            self.output_results_table = Some(top_table);
        }
        self.output_artifacts = format!(
            "Hierarchy evaluated\nTop macro: {}\nEvaluated macros: {}\nOutput dir: {}",
            top_name,
            evaluated.join(", "),
            output_dir.display()
        );
        self.save_active_macro_workspace();

        format!(
            "Evaluated hierarchy for '{}'\n\nMacros: {}\nOrder: {}",
            top_name,
            evaluated.len(),
            evaluated.join(" -> ")
        )
    }

    fn evaluate_workspace_index(
        &self,
        macro_index: usize,
        catalog: &PrimitiveCatalog,
        macro_catalog: &MacroCatalog,
        macro_blocks: &[GuiDutMacroView],
        prepared_dir: &std::path::Path,
        extra_candidate_sets: Vec<CandidateSet>,
    ) -> Result<GuiWorkspaceEvaluation, String> {
        let document = self
            .circuits
            .get(macro_index)
            .ok_or_else(|| format!("missing macro document at index {}", macro_index + 1))?;
        let workspace = self
            .macro_workspaces
            .get(macro_index)
            .ok_or_else(|| format!("missing macro workspace for '{}'", document.name))?;

        std::fs::create_dir_all(prepared_dir).map_err(|error| {
            format!(
                "failed to create prepared specs dir '{}'\n\n{error}",
                prepared_dir.display()
            )
        })?;

        let testbenches = gui_testbenches_to_specs(&workspace.testbenches, &self.circuits)?;
        let specs = gui_specs_to_exploration_specs(&workspace.specs, &testbenches)?;
        let derived_columns = gui_derived_columns_to_specs(&workspace.derived_columns)?;
        let mut candidate_input =
            gui_candidate_input(&workspace.candidates, document, catalog, macro_blocks)?;
        candidate_input.sets.extend(extra_candidate_sets);
        let gui_mode = gui_specs_small_signal_mode(&workspace.specs, &workspace.testbenches)?;
        let mode = macro_small_signal_mode(gui_mode);
        let prepared_specs = prepare_macro_testbench_specs_with_mode(
            &specs,
            catalog,
            macro_catalog,
            prepared_dir,
            mode,
        )
        .map_err(|error| format!("failed to prepare specs\n\n{error:?}"))?;
        let mut table = run_prepared_expression_flow_with_derived_columns(
            &candidate_input.axes,
            &candidate_input.sets,
            &candidate_input.filters,
            &prepared_specs,
            &derived_columns,
        )
        .map_err(|error| format!("failed to evaluate specs\n\n{error:?}"))?;

        add_automatic_area_column(&mut table)
            .map_err(|error| format!("failed to add automatic area column\n\n{error:?}"))?;

        Ok(GuiWorkspaceEvaluation {
            prepared_specs,
            candidate_input,
            table,
        })
    }

    fn hierarchy_evaluation_order(&self, top_index: usize) -> Result<Vec<usize>, String> {
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        self.push_hierarchy_evaluation_order(top_index, &mut visiting, &mut visited, &mut order)?;
        Ok(order)
    }

    fn push_hierarchy_evaluation_order(
        &self,
        macro_index: usize,
        visiting: &mut HashSet<usize>,
        visited: &mut HashSet<usize>,
        order: &mut Vec<usize>,
    ) -> Result<(), String> {
        if visited.contains(&macro_index) {
            return Ok(());
        }
        if !visiting.insert(macro_index) {
            let name = self
                .circuits
                .get(macro_index)
                .map(|document| document.name.as_str())
                .unwrap_or("<missing>");
            return Err(format!("macro hierarchy contains a cycle at '{name}'"));
        }

        for (_, child_macro_name) in self.direct_macro_instances(macro_index)? {
            let child_index = self
                .circuit_index_by_name(&child_macro_name)
                .ok_or_else(|| format!("submacro '{child_macro_name}' is not imported locally"))?;
            self.push_hierarchy_evaluation_order(child_index, visiting, visited, order)?;
        }

        visiting.remove(&macro_index);
        visited.insert(macro_index);
        order.push(macro_index);
        Ok(())
    }

    fn compact_candidate_sets_for_children(
        &self,
        macro_index: usize,
        results_by_macro: &HashMap<String, ExplorationTable>,
    ) -> Result<Vec<CandidateSet>, String> {
        let mut sets = Vec::new();

        for (instance_name, child_macro_name) in self.direct_macro_instances(macro_index)? {
            let child_index = self
                .circuit_index_by_name(&child_macro_name)
                .ok_or_else(|| format!("submacro '{child_macro_name}' is not imported locally"))?;
            let child_workspace = self
                .macro_workspaces
                .get(child_index)
                .ok_or_else(|| format!("missing workspace for submacro '{child_macro_name}'"))?;
            let child_table = results_by_macro.get(&child_macro_name).ok_or_else(|| {
                format!("submacro '{child_macro_name}' has not been evaluated yet")
            })?;
            let bindings = compact_output_bindings_for_workspace(child_workspace)?;
            let candidate_set =
                submacro_results_to_compact_candidate_set(&instance_name, &bindings, child_table)
                    .map_err(|error| {
                    format!(
                        "failed to map results from submacro instance '{}' ({})\n\n{error:?}",
                        instance_name, child_macro_name
                    )
                })?;
            sets.push(candidate_set);
        }

        Ok(sets)
    }

    fn direct_macro_instances(&self, macro_index: usize) -> Result<Vec<(String, String)>, String> {
        let document = self
            .circuits
            .get(macro_index)
            .ok_or_else(|| format!("missing macro document at index {}", macro_index + 1))?;

        Ok(document
            .canvas_instances
            .iter()
            .filter_map(|instance| {
                instance
                    .block
                    .macro_name()
                    .map(|macro_name| (instance.instance_name.clone(), macro_name.to_string()))
            })
            .collect())
    }

    fn circuit_index_by_name(&self, name: &str) -> Option<usize> {
        self.circuits
            .iter()
            .position(|document| document.name == name)
    }

    fn prepare_gui_specs(&mut self) -> String {
        self.save_active_circuit_document();
        self.save_active_macro_workspace();

        let Some(catalog) = &self.catalog else {
            return "Cannot prepare specs: primitive catalog is not loaded".to_string();
        };

        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let macro_dir = output_dir.join("gui_macros");
        let testbench_path = output_dir.join("gui_testbenches.json");
        let specs_path = output_dir.join("gui_specs.json");
        let prepared_dir = output_dir.join("prepared_specs");

        if let Err(error) = std::fs::create_dir_all(&prepared_dir) {
            return format!(
                "Cannot prepare specs: failed to create output directory '{}'\n\n{error}",
                prepared_dir.display()
            );
        }

        let macro_models = match self.build_macro_models() {
            Ok(macro_models) => macro_models,
            Err(error) => return format!("Cannot prepare specs: {error}"),
        };
        let testbenches = match gui_testbenches_to_specs(&self.testbenches, &self.circuits) {
            Ok(testbenches) => testbenches,
            Err(error) => return format!("Cannot prepare specs: {error}"),
        };
        let specs = match gui_specs_to_exploration_specs(&self.specs, &testbenches) {
            Ok(specs) => specs,
            Err(error) => return format!("Cannot prepare specs: {error}"),
        };
        let gui_mode = match gui_specs_small_signal_mode(&self.specs, &self.testbenches) {
            Ok(mode) => mode,
            Err(error) => return format!("Cannot prepare specs: {error}"),
        };
        let mode = macro_small_signal_mode(gui_mode);

        for macro_model in &macro_models {
            let macro_path = macro_dir.join(&macro_model.name).join("macro.json");
            if let Err(error) = save_macro_model(&macro_path, macro_model) {
                return format!(
                    "Cannot prepare specs: failed to write macro JSON '{}'\n\n{error:?}",
                    macro_path.display()
                );
            }
        }

        if let Err(error) = save_testbenches(&testbench_path, &testbenches) {
            return format!(
                "Cannot prepare specs: failed to write testbench JSON '{}'\n\n{error:?}",
                testbench_path.display()
            );
        }

        if let Err(error) = save_exploration_specs(&specs_path, &specs) {
            return format!(
                "Cannot prepare specs: failed to write specs JSON '{}'\n\n{error:?}",
                specs_path.display()
            );
        }

        let macro_count = macro_models.len();
        let spec_count = specs.len();
        let macro_catalog = self.runtime_macro_catalog(macro_models);

        match prepare_macro_testbench_specs_with_mode(
            &specs,
            catalog,
            &macro_catalog,
            &prepared_dir,
            mode,
        ) {
            Ok(prepared_specs) => {
                self.output_prepared_specs = format_prepared_specs_output(&prepared_specs);
                self.output_artifacts = format!(
                    "Macro dir: {}\nTestbenches: {}\nSpecs: {}\nPrepared specs dir: {}\nSmall-signal mode: {}",
                    macro_dir.display(),
                    testbench_path.display(),
                    specs_path.display(),
                    prepared_dir.display(),
                    gui_mode.label()
                );
                self.save_active_macro_workspace();

                format!(
                    "Prepared {} exploration spec(s)\n\nMacros: {}\nSpecs: {}\nSmall-signal mode: {}\nPrepared dir: {}",
                    prepared_specs.len(),
                    macro_count,
                    spec_count,
                    gui_mode.label(),
                    prepared_dir.display()
                )
            }
            Err(error) => format!(
                "Prepare specs failed\n\nMacro dir: {}\nTestbenches: {}\nSpecs: {}\nPrepared dir: {}\n\n{error:?}",
                macro_dir.display(),
                testbench_path.display(),
                specs_path.display(),
                prepared_dir.display()
            ),
        }
    }
}

const MAX_VISIBLE_RESULT_ROWS: usize = 200;

fn show_results_output(ui: &mut egui::Ui, fallback_text: &str, table: Option<&ExplorationTable>) {
    let Some(table) = table else {
        show_text_output(ui, fallback_text);
        return;
    };

    ui.horizontal(|ui| {
        for line in exploration_table_summary_lines(table) {
            ui.label(line);
        }
    });
    ui.separator();

    egui::ScrollArea::both().show(ui, |ui| {
        egui::Grid::new("exploration_results_table")
            .striped(true)
            .spacing(egui::vec2(16.0, 4.0))
            .show(ui, |ui| {
                ui.strong("index");
                for column in &table.columns {
                    ui.strong(&column.name);
                }
                ui.end_row();

                for row in 0..results_table_shown_rows(table) {
                    ui.monospace(row.to_string());
                    for column in &table.columns {
                        if let Some(value) = column.values.get(row) {
                            ui.monospace(format!("{value:.6e}"));
                        } else {
                            ui.label("");
                        }
                    }
                    ui.end_row();
                }
            });
    });
}

fn exploration_table_summary_lines(table: &ExplorationTable) -> Vec<String> {
    let mut lines = vec![
        format!("rows: {}", table.row_count),
        format!("columns: {}", table.columns.len()),
    ];

    let shown_rows = results_table_shown_rows(table);
    if table.row_count > shown_rows {
        lines.push(format!("showing first {shown_rows} rows"));
    }

    lines
}

fn results_table_shown_rows(table: &ExplorationTable) -> usize {
    table.row_count.min(MAX_VISIBLE_RESULT_ROWS)
}

fn show_text_output(ui: &mut egui::Ui, text: &str) {
    egui::ScrollArea::both().show(ui, |ui| {
        let mut display = text.to_string();
        ui.add(
            egui::TextEdit::multiline(&mut display)
                .desired_width(f32::INFINITY)
                .code_editor()
                .interactive(false),
        );
    });
}

impl CanvasView {
    fn instance_rect(&self, instance: &CanvasInstance) -> egui::Rect {
        egui::Rect::from_min_size(self.to_screen(instance.position), egui::vec2(160.0, 72.0))
    }
}

impl CanvasConnection {
    fn references_instance(&self, instance_id: usize) -> bool {
        self.from.references_instance(instance_id) || self.to.references_instance(instance_id)
    }

    fn references_macro_port(&self, port_id: usize) -> bool {
        self.from.references_macro_port(port_id) || self.to.references_macro_port(port_id)
    }
}

impl CanvasEndpoint {
    fn references_instance(&self, instance_id: usize) -> bool {
        match self {
            CanvasEndpoint::PrimitivePin {
                instance_id: endpoint_instance_id,
                ..
            } => *endpoint_instance_id == instance_id,
            CanvasEndpoint::LabelPin { .. } | CanvasEndpoint::MacroPort { .. } => false,
        }
    }

    fn references_macro_port(&self, port_id: usize) -> bool {
        match self {
            CanvasEndpoint::MacroPort {
                port_id: endpoint_port_id,
            } => *endpoint_port_id == port_id,
            CanvasEndpoint::PrimitivePin { .. } | CanvasEndpoint::LabelPin { .. } => false,
        }
    }
}

impl GuiCircuitDocument {
    fn empty(name: impl Into<String>) -> Self {
        let name = name.into();

        Self {
            subckt_name: name.clone(),
            name,
            small_signal: None,
            ports: Vec::new(),
            canvas_instances: Vec::new(),
            label_pins: Vec::new(),
            macro_ports: Vec::new(),
            connections: Vec::new(),
            next_instance_id: 1,
            next_label_pin_id: 1,
            next_macro_port_id: 1,
        }
    }

    fn from_macro_model(macro_model: &MacroModel) -> Self {
        let ports = GuiDutMacroView::from_macro_model(macro_model).ports;
        let macro_ports = ports
            .iter()
            .enumerate()
            .map(|(index, port)| CanvasMacroPort {
                id: index + 1,
                name: port.name.clone(),
                role: port.role,
                symbol_side: port.symbol_side,
                symbol_offset: port.symbol_offset,
                position: imported_macro_port_position(port.symbol_side, port.symbol_offset),
            })
            .collect::<Vec<_>>();
        let macro_port_ids = macro_ports
            .iter()
            .map(|port| (port.name.clone(), port.id))
            .collect::<HashMap<_, _>>();
        let canvas_instances = macro_model
            .circuit
            .instances
            .iter()
            .enumerate()
            .map(|(index, instance)| {
                let block = if let Some(primitive_name) = instance.primitive_name() {
                    GuiBlockRef::Primitive {
                        name: primitive_name.to_string(),
                    }
                } else if let Some(macro_name) = instance.macro_name() {
                    GuiBlockRef::Macro {
                        name: macro_name.to_string(),
                    }
                } else {
                    GuiBlockRef::Primitive {
                        name: String::new(),
                    }
                };

                CanvasInstance {
                    id: index + 1,
                    instance_name: instance.id.clone(),
                    block,
                    position: imported_instance_position(index),
                    orientation: GuiOrientation::R0,
                }
            })
            .collect::<Vec<_>>();
        let instance_ids = canvas_instances
            .iter()
            .map(|instance| (instance.instance_name.clone(), instance.id))
            .collect::<HashMap<_, _>>();
        let mut net_endpoints: HashMap<String, Vec<CanvasEndpoint>> = HashMap::new();

        for connection in &macro_model.circuit.connections {
            let Some(instance_id) = instance_ids.get(&connection.from.instance) else {
                continue;
            };
            net_endpoints
                .entry(connection.net.clone())
                .or_default()
                .push(CanvasEndpoint::PrimitivePin {
                    instance_id: *instance_id,
                    pin_name: connection.from.pin.clone(),
                });
        }

        let mut label_pins = Vec::new();
        let mut connections = Vec::new();
        let mut net_names = net_endpoints.keys().cloned().collect::<Vec<_>>();
        net_names.sort();

        for net_name in net_names {
            let mut endpoints = net_endpoints.remove(&net_name).unwrap_or_default();
            if let Some(port_id) = macro_port_ids.get(&net_name) {
                endpoints.push(CanvasEndpoint::MacroPort { port_id: *port_id });
            } else {
                let label_id = label_pins.len() + 1;
                label_pins.push(CanvasLabelPin {
                    id: label_id,
                    name: net_name,
                    position: imported_label_pin_position(label_id - 1),
                });
                endpoints.push(CanvasEndpoint::LabelPin { label_id });
            }

            if let Some(first) = endpoints.first().cloned() {
                for endpoint in endpoints.into_iter().skip(1) {
                    connections.push(CanvasConnection {
                        from: first.clone(),
                        to: endpoint,
                    });
                }
            }
        }

        Self {
            name: macro_model.name.clone(),
            subckt_name: macro_model.subckt_name.clone(),
            small_signal: macro_model.small_signal.clone(),
            ports,
            canvas_instances,
            next_instance_id: macro_model.circuit.instances.len() + 1,
            next_label_pin_id: label_pins.len() + 1,
            next_macro_port_id: macro_ports.len() + 1,
            label_pins,
            macro_ports,
            connections,
        }
    }
}

impl GuiMacroPortRole {
    fn all() -> [Self; 6] {
        [
            Self::Input,
            Self::Output,
            Self::Inout,
            Self::Bias,
            Self::Supply,
            Self::Ground,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Output => "output",
            Self::Inout => "inout",
            Self::Bias => "bias",
            Self::Supply => "supply",
            Self::Ground => "ground",
        }
    }
}

impl GuiSmallSignalMode {
    fn all() -> [Self; 2] {
        [Self::CompactWhenAvailable, Self::Expand]
    }

    fn label(self) -> &'static str {
        match self {
            Self::CompactWhenAvailable => "compact when available",
            Self::Expand => "expand",
        }
    }
}

impl GuiOrientation {
    fn rotated_clockwise(self) -> Self {
        match self {
            Self::R0 => Self::R90,
            Self::R90 => Self::R180,
            Self::R180 => Self::R270,
            Self::R270 => Self::R0,
        }
    }

    fn degrees(self) -> u16 {
        match self {
            Self::R0 => 0,
            Self::R90 => 90,
            Self::R180 => 180,
            Self::R270 => 270,
        }
    }
}

impl GuiDutMacroView {
    fn from_circuit_document(circuit: &GuiCircuitDocument) -> Self {
        Self {
            name: circuit.name.clone(),
            ports: document_macro_ports(circuit)
                .into_iter()
                .map(gui_macro_port_from_view)
                .collect(),
        }
    }

    fn from_macro_model(macro_model: &MacroModel) -> Self {
        let ports = macro_model
            .ports
            .iter()
            .enumerate()
            .map(|(index, port)| {
                let symbol_pin = macro_model
                    .symbol
                    .as_ref()
                    .and_then(|symbol| symbol.pins.iter().find(|pin| pin.name == port.name));
                GuiMacroPort {
                    name: port.name.clone(),
                    role: gui_macro_port_role(port.role.clone()),
                    symbol_side: symbol_pin
                        .map(|pin| pin.side)
                        .unwrap_or_else(|| default_symbol_side_for_index(index)),
                    symbol_offset: symbol_pin
                        .map(|pin| pin.offset)
                        .unwrap_or_else(|| default_symbol_offset_for_index(index)),
                }
            })
            .collect();

        Self {
            name: macro_model.name.clone(),
            ports,
        }
    }
}

impl GuiTestbenchDocument {
    fn add_element(&mut self, kind: GuiTestbenchElementKind) {
        let id = self.next_element_id;
        let index = self.elements.len() + 1;

        self.elements
            .push(GuiTestbenchElement::new(kind, id, index));
        self.next_element_id += 1;
    }

    fn remove_element(&mut self, element_index: usize) {
        let Some(element) = self.elements.get(element_index) else {
            return;
        };
        let element_id = element.id;

        self.elements.remove(element_index);
        self.connections
            .retain(|connection| !connection.references_element(element_id));

        if self
            .selected_endpoint
            .as_ref()
            .is_some_and(|endpoint| endpoint.references_element(element_id))
        {
            self.selected_endpoint = None;
        }
        if self
            .pending_connection
            .as_ref()
            .is_some_and(|endpoint| endpoint.references_element(element_id))
        {
            self.pending_connection = None;
        }
    }
}

impl GuiTestbenchConnection {
    fn references_element(&self, element_id: usize) -> bool {
        self.from.references_element(element_id) || self.to.references_element(element_id)
    }
}

impl TestbenchEndpoint {
    fn references_element(&self, id: usize) -> bool {
        match self {
            TestbenchEndpoint::ElementPin { element_id, .. } => *element_id == id,
            TestbenchEndpoint::DutPort { .. } => false,
        }
    }
}

impl GuiTestbenchElementKind {
    fn label(self) -> &'static str {
        match self {
            Self::VoltageSource => "Voltage source",
            Self::CurrentSource => "Current source",
            Self::Resistor => "Resistor",
            Self::Capacitor => "Capacitor",
        }
    }

    fn default_name_prefix(self) -> &'static str {
        match self {
            Self::VoltageSource => "V",
            Self::CurrentSource => "I",
            Self::Resistor => "R",
            Self::Capacitor => "C",
        }
    }
}

impl GuiTestbenchElement {
    fn new(kind: GuiTestbenchElementKind, id: usize, index: usize) -> Self {
        let offset = 24.0 * (index.saturating_sub(1)) as f32;

        Self {
            id,
            kind,
            name: format!("{}{}", kind.default_name_prefix(), index),
            nplus: String::new(),
            nminus: "0".to_string(),
            value: String::new(),
            position: egui::pos2(48.0 + offset, 48.0 + offset),
            orientation: GuiOrientation::R0,
        }
    }
}

impl GuiProjectPosition {
    fn from_pos(position: egui::Pos2) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }

    fn to_pos(&self) -> egui::Pos2 {
        egui::pos2(self.x, self.y)
    }
}

impl GuiProjectOrientation {
    fn from_gui_orientation(orientation: GuiOrientation) -> Self {
        match orientation {
            GuiOrientation::R0 => Self::R0,
            GuiOrientation::R90 => Self::R90,
            GuiOrientation::R180 => Self::R180,
            GuiOrientation::R270 => Self::R270,
        }
    }

    fn into_gui_orientation(self) -> GuiOrientation {
        match self {
            Self::R0 => GuiOrientation::R0,
            Self::R90 => GuiOrientation::R90,
            Self::R180 => GuiOrientation::R180,
            Self::R270 => GuiOrientation::R270,
        }
    }
}

impl GuiProjectBlockRef {
    fn from_gui_block_ref(block: &GuiBlockRef) -> Self {
        match block {
            GuiBlockRef::Primitive { name } => Self::Primitive { name: name.clone() },
            GuiBlockRef::Macro { name } => Self::Macro { name: name.clone() },
        }
    }

    fn into_gui_block_ref(self) -> GuiBlockRef {
        match self {
            Self::Primitive { name } => GuiBlockRef::Primitive { name },
            Self::Macro { name } => GuiBlockRef::Macro { name },
        }
    }
}

impl GuiProjectInstance {
    fn into_gui_block_ref(self) -> GuiBlockRef {
        if let Some(block) = self.block {
            return block.into_gui_block_ref();
        }

        GuiBlockRef::Primitive {
            name: self.primitive,
        }
    }
}

impl GuiProjectCircuit {
    fn from_circuit_document(circuit: &GuiCircuitDocument) -> Self {
        Self {
            name: circuit.name.clone(),
            subckt_name: circuit.subckt_name.clone(),
            small_signal: circuit.small_signal.clone(),
            ports: circuit
                .ports
                .iter()
                .map(GuiProjectMacroPort::from_macro_port)
                .collect(),
            instances: circuit
                .canvas_instances
                .iter()
                .map(|instance| GuiProjectInstance {
                    id: instance.id,
                    name: exported_instance_name(instance),
                    primitive: instance
                        .block
                        .primitive_name()
                        .unwrap_or_default()
                        .to_string(),
                    block: Some(GuiProjectBlockRef::from_gui_block_ref(&instance.block)),
                    position: GuiProjectPosition::from_pos(instance.position),
                    orientation: GuiProjectOrientation::from_gui_orientation(instance.orientation),
                })
                .collect(),
            label_pins: circuit
                .label_pins
                .iter()
                .map(|label_pin| GuiProjectLabelPin {
                    id: label_pin.id,
                    name: label_pin.name.clone(),
                    position: GuiProjectPosition::from_pos(label_pin.position),
                })
                .collect(),
            macro_ports: circuit
                .macro_ports
                .iter()
                .map(GuiProjectCanvasMacroPort::from_canvas_macro_port)
                .collect(),
            connections: circuit
                .connections
                .iter()
                .map(GuiProjectConnection::from_canvas_connection)
                .collect(),
            next_instance_id: circuit.next_instance_id,
            next_label_pin_id: circuit.next_label_pin_id,
            next_macro_port_id: circuit.next_macro_port_id,
        }
    }

    fn into_circuit_document(self) -> GuiCircuitDocument {
        let legacy_ports = self
            .ports
            .into_iter()
            .map(GuiProjectMacroPort::into_macro_port)
            .collect::<Vec<_>>();
        let canvas_instances = self
            .instances
            .into_iter()
            .map(|instance| {
                let id = instance.id;
                let instance_name = instance.name.clone();
                let position = instance.position.to_pos();
                let orientation = instance.orientation.into_gui_orientation();
                let block = instance.into_gui_block_ref();

                CanvasInstance {
                    id,
                    instance_name,
                    block,
                    position,
                    orientation,
                }
            })
            .collect::<Vec<_>>();
        let label_pins = self
            .label_pins
            .into_iter()
            .map(|label_pin| CanvasLabelPin {
                id: label_pin.id,
                name: label_pin.name,
                position: label_pin.position.to_pos(),
            })
            .collect::<Vec<_>>();
        let mut macro_ports = self
            .macro_ports
            .into_iter()
            .map(GuiProjectCanvasMacroPort::into_canvas_macro_port)
            .collect::<Vec<_>>();
        if macro_ports.is_empty() {
            macro_ports = legacy_ports
                .iter()
                .enumerate()
                .map(|(index, port)| CanvasMacroPort {
                    id: index + 1,
                    name: port.name.clone(),
                    role: port.role,
                    symbol_side: port.symbol_side,
                    symbol_offset: port.symbol_offset,
                    position: egui::pos2(112.0 + 24.0 * index as f32, 112.0 + 24.0 * index as f32),
                })
                .collect();
        }
        let next_macro_port_id = if self.next_macro_port_id == 0 {
            legacy_ports.len() + 1
        } else {
            self.next_macro_port_id
        };
        let connections = self
            .connections
            .into_iter()
            .map(GuiProjectConnection::into_canvas_connection)
            .filter(|connection| {
                project_connection_endpoint_exists(
                    &connection.from,
                    &canvas_instances,
                    &label_pins,
                    &macro_ports,
                ) && project_connection_endpoint_exists(
                    &connection.to,
                    &canvas_instances,
                    &label_pins,
                    &macro_ports,
                )
            })
            .collect();

        let subckt_name = if self.subckt_name.trim().is_empty() {
            self.name.clone()
        } else {
            self.subckt_name
        };

        GuiCircuitDocument {
            name: self.name,
            subckt_name,
            small_signal: self.small_signal,
            ports: legacy_ports,
            canvas_instances,
            label_pins,
            macro_ports,
            connections,
            next_instance_id: self.next_instance_id,
            next_label_pin_id: self.next_label_pin_id,
            next_macro_port_id,
        }
    }
}

impl GuiProjectCanvasMacroPort {
    fn from_canvas_macro_port(port: &CanvasMacroPort) -> Self {
        Self {
            id: port.id,
            name: port.name.clone(),
            role: GuiProjectMacroPortRole::from_macro_port_role(port.role),
            symbol_side: port.symbol_side,
            symbol_offset: port.symbol_offset,
            position: GuiProjectPosition::from_pos(port.position),
        }
    }

    fn into_canvas_macro_port(self) -> CanvasMacroPort {
        CanvasMacroPort {
            id: self.id,
            name: self.name,
            role: self.role.into_macro_port_role(),
            symbol_side: self.symbol_side,
            symbol_offset: self.symbol_offset.clamp(0.0, 1.0),
            position: self.position.to_pos(),
        }
    }
}

impl GuiProjectMacroPort {
    fn from_macro_port(port: &GuiMacroPort) -> Self {
        Self {
            name: port.name.clone(),
            role: GuiProjectMacroPortRole::from_macro_port_role(port.role),
            symbol_side: Some(port.symbol_side),
            symbol_offset: Some(port.symbol_offset),
        }
    }

    fn into_macro_port(self) -> GuiMacroPort {
        let role = self.role.into_macro_port_role();
        GuiMacroPort {
            name: self.name,
            role,
            symbol_side: self
                .symbol_side
                .unwrap_or_else(|| default_symbol_side_for_role(role)),
            symbol_offset: self.symbol_offset.unwrap_or(0.5).clamp(0.0, 1.0),
        }
    }
}

impl GuiProjectMacroPortRole {
    fn from_macro_port_role(role: GuiMacroPortRole) -> Self {
        match role {
            GuiMacroPortRole::Input => Self::Input,
            GuiMacroPortRole::Output => Self::Output,
            GuiMacroPortRole::Inout => Self::Inout,
            GuiMacroPortRole::Bias => Self::Bias,
            GuiMacroPortRole::Supply => Self::Supply,
            GuiMacroPortRole::Ground => Self::Ground,
        }
    }

    fn into_macro_port_role(self) -> GuiMacroPortRole {
        match self {
            Self::Input => GuiMacroPortRole::Input,
            Self::Output => GuiMacroPortRole::Output,
            Self::Inout => GuiMacroPortRole::Inout,
            Self::Bias => GuiMacroPortRole::Bias,
            Self::Supply => GuiMacroPortRole::Supply,
            Self::Ground => GuiMacroPortRole::Ground,
        }
    }
}

impl GuiProjectMacroWorkspace {
    fn from_macro_workspace(workspace: &GuiMacroWorkspace) -> Self {
        Self {
            selected_testbench: workspace.selected_testbench,
            testbenches: workspace
                .testbenches
                .iter()
                .map(GuiProjectTestbench::from_testbench_document)
                .collect(),
            selected_spec: workspace.selected_spec,
            specs: workspace
                .specs
                .iter()
                .map(GuiProjectSpec::from_spec_document)
                .collect(),
            derived_columns: workspace
                .derived_columns
                .iter()
                .map(GuiProjectDerivedColumn::from_derived_column_document)
                .collect(),
            candidates: GuiProjectCandidateDocument::from_candidate_document(&workspace.candidates),
        }
    }

    fn into_macro_workspace(self) -> GuiMacroWorkspace {
        let testbenches = self
            .testbenches
            .into_iter()
            .map(GuiProjectTestbench::into_testbench_document)
            .collect::<Vec<_>>();
        let specs = self
            .specs
            .into_iter()
            .map(GuiProjectSpec::into_spec_document)
            .collect::<Vec<_>>();
        let derived_columns = self
            .derived_columns
            .into_iter()
            .map(GuiProjectDerivedColumn::into_derived_column_document)
            .collect::<Vec<_>>();

        GuiMacroWorkspace {
            selected_testbench: self
                .selected_testbench
                .filter(|index| *index < testbenches.len()),
            testbenches,
            selected_spec: self.selected_spec.filter(|index| *index < specs.len()),
            specs,
            derived_columns,
            candidates: self.candidates.into_candidate_document(),
            ..GuiMacroWorkspace::default()
        }
    }
}

impl GuiProjectTestbench {
    fn from_testbench_document(testbench: &GuiTestbenchDocument) -> Self {
        Self {
            name: testbench.name.clone(),
            dut_macro: testbench.dut_macro.clone(),
            small_signal_mode: GuiProjectSmallSignalMode::from_gui_mode(
                testbench.small_signal_mode,
            ),
            dut_position: Some(GuiProjectPosition::from_pos(testbench.dut_position)),
            compact_outputs: testbench
                .compact_outputs
                .iter()
                .map(GuiProjectCompactOutputBinding::from_binding)
                .collect(),
            elements: testbench
                .elements
                .iter()
                .map(GuiProjectTestbenchElement::from_testbench_element)
                .collect(),
            connections: testbench
                .connections
                .iter()
                .map(GuiProjectTestbenchConnection::from_testbench_connection)
                .collect(),
            extra_body: testbench.extra_body.clone(),
            next_element_id: testbench.next_element_id,
        }
    }

    fn into_testbench_document(self) -> GuiTestbenchDocument {
        let elements = self
            .elements
            .into_iter()
            .map(GuiProjectTestbenchElement::into_testbench_element)
            .collect::<Vec<_>>();
        let connections = self
            .connections
            .into_iter()
            .map(GuiProjectTestbenchConnection::into_testbench_connection)
            .filter(|connection| {
                testbench_connection_endpoint_exists(&connection.from, &elements)
                    && testbench_connection_endpoint_exists(&connection.to, &elements)
            })
            .collect();

        GuiTestbenchDocument {
            name: self.name,
            dut_macro: self.dut_macro,
            dut_position: self
                .dut_position
                .map(|position| position.to_pos())
                .unwrap_or_else(default_dut_position),
            small_signal_mode: self.small_signal_mode.into_gui_mode(),
            compact_outputs: self
                .compact_outputs
                .into_iter()
                .map(GuiProjectCompactOutputBinding::into_binding)
                .collect(),
            elements,
            connections,
            selected_endpoint: None,
            pending_connection: None,
            extra_body: self.extra_body,
            next_element_id: self.next_element_id,
        }
    }
}

impl GuiProjectCompactOutputBinding {
    fn from_binding(binding: &GuiCompactOutputBinding) -> Self {
        Self {
            source_column: binding.source_column.clone(),
            compact_parameter: binding.compact_parameter.clone(),
        }
    }

    fn into_binding(self) -> GuiCompactOutputBinding {
        GuiCompactOutputBinding {
            source_column: self.source_column,
            compact_parameter: self.compact_parameter,
        }
    }
}

impl GuiProjectSpec {
    fn from_spec_document(spec: &GuiSpecDocument) -> Self {
        Self {
            name: spec.name.clone(),
            source_kind: GuiProjectSpecSourceKind::from_gui_source_kind(spec.source_kind),
            testbench: spec.testbench.clone(),
            input: spec.input.clone(),
            output: spec.output.clone(),
            node: spec.node.clone(),
            min: spec.min.clone(),
            max: spec.max.clone(),
            parameter_map: spec
                .parameter_map
                .iter()
                .map(GuiProjectSpecParameter::from_spec_parameter)
                .collect(),
        }
    }

    fn into_spec_document(self) -> GuiSpecDocument {
        GuiSpecDocument {
            name: self.name,
            source_kind: self.source_kind.into_gui_source_kind(),
            testbench: self.testbench,
            input: self.input,
            output: self.output,
            node: self.node,
            min: self.min,
            max: self.max,
            parameter_map: self
                .parameter_map
                .into_iter()
                .map(GuiProjectSpecParameter::into_spec_parameter)
                .collect(),
        }
    }
}

impl GuiProjectDerivedColumn {
    fn from_derived_column_document(column: &GuiDerivedColumnDocument) -> Self {
        Self {
            name: column.name.clone(),
            expression: column.expression.clone(),
        }
    }

    fn into_derived_column_document(self) -> GuiDerivedColumnDocument {
        GuiDerivedColumnDocument {
            name: self.name,
            expression: self.expression,
        }
    }
}

impl GuiProjectSpecSourceKind {
    fn from_gui_source_kind(kind: GuiSpecSourceKind) -> Self {
        match kind {
            GuiSpecSourceKind::TransferFunction => Self::TransferFunction,
            GuiSpecSourceKind::NodeVoltage => Self::NodeVoltage,
        }
    }

    fn into_gui_source_kind(self) -> GuiSpecSourceKind {
        match self {
            Self::TransferFunction => GuiSpecSourceKind::TransferFunction,
            Self::NodeVoltage => GuiSpecSourceKind::NodeVoltage,
        }
    }
}

impl GuiProjectSpecParameter {
    fn from_spec_parameter(parameter: &GuiSpecParameter) -> Self {
        Self {
            name: parameter.name.clone(),
            value: parameter.value.clone(),
        }
    }

    fn into_spec_parameter(self) -> GuiSpecParameter {
        GuiSpecParameter {
            name: self.name,
            value: self.value,
        }
    }
}

impl GuiProjectCandidateDocument {
    fn from_candidate_document(candidates: &GuiCandidateDocument) -> Self {
        Self {
            axes: candidates
                .axes
                .iter()
                .map(GuiProjectCandidateAxis::from_candidate_axis)
                .collect(),
            net_voltage_constraints: candidates
                .net_voltage_constraints
                .iter()
                .map(GuiProjectNetVoltageConstraint::from_net_voltage_constraint)
                .collect(),
            global_build_parameters: candidates
                .global_build_parameters
                .iter()
                .map(GuiProjectBuildParameter::from_build_parameter)
                .collect(),
            primitive_build_overrides: candidates
                .primitive_build_overrides
                .iter()
                .map(GuiProjectPrimitiveBuildOverride::from_primitive_build_override)
                .collect(),
            python_path: candidates.python_path.clone(),
            nmos_lut_path: candidates.nmos_lut_path.clone(),
            pmos_lut_path: candidates.pmos_lut_path.clone(),
            timing_output: candidates.timing_output,
        }
    }

    fn into_candidate_document(self) -> GuiCandidateDocument {
        GuiCandidateDocument {
            axes: self
                .axes
                .into_iter()
                .map(GuiProjectCandidateAxis::into_candidate_axis)
                .collect(),
            net_voltage_constraints: self
                .net_voltage_constraints
                .into_iter()
                .map(GuiProjectNetVoltageConstraint::into_net_voltage_constraint)
                .collect(),
            global_build_parameters: self
                .global_build_parameters
                .into_iter()
                .map(GuiProjectBuildParameter::into_build_parameter)
                .collect(),
            primitive_build_overrides: self
                .primitive_build_overrides
                .into_iter()
                .map(GuiProjectPrimitiveBuildOverride::into_primitive_build_override)
                .collect(),
            python_path: defaulted_python_gmid_path(self.python_path),
            nmos_lut_path: defaulted_project_text(self.nmos_lut_path, default_nmos_lut_path()),
            pmos_lut_path: defaulted_project_text(self.pmos_lut_path, default_pmos_lut_path()),
            timing_output: self.timing_output,
        }
    }
}

impl GuiProjectCandidateAxis {
    fn from_candidate_axis(axis: &GuiCandidateAxis) -> Self {
        Self {
            name: axis.name.clone(),
            values: axis.values.clone(),
        }
    }

    fn into_candidate_axis(self) -> GuiCandidateAxis {
        GuiCandidateAxis {
            name: self.name,
            values: self.values,
        }
    }
}

impl GuiProjectNetVoltageConstraint {
    fn from_net_voltage_constraint(constraint: &GuiNetVoltageConstraint) -> Self {
        Self {
            net: constraint.net.clone(),
            values: constraint.values.clone(),
        }
    }

    fn into_net_voltage_constraint(self) -> GuiNetVoltageConstraint {
        GuiNetVoltageConstraint {
            net: self.net,
            values: self.values,
        }
    }
}

impl GuiProjectBuildParameter {
    fn from_build_parameter(parameter: &GuiBuildParameter) -> Self {
        Self {
            name: parameter.name.clone(),
            values: parameter.values.clone(),
        }
    }

    fn into_build_parameter(self) -> GuiBuildParameter {
        GuiBuildParameter {
            name: self.name,
            values: self.values,
        }
    }
}

impl GuiProjectPrimitiveBuildOverride {
    fn from_primitive_build_override(override_: &GuiPrimitiveBuildOverride) -> Self {
        Self {
            instance_id: override_.instance_id,
            parameters: override_
                .parameters
                .iter()
                .map(GuiProjectBuildParameter::from_build_parameter)
                .collect(),
        }
    }

    fn into_primitive_build_override(self) -> GuiPrimitiveBuildOverride {
        GuiPrimitiveBuildOverride {
            instance_id: self.instance_id,
            parameters: self
                .parameters
                .into_iter()
                .map(GuiProjectBuildParameter::into_build_parameter)
                .collect(),
        }
    }
}

impl GuiProjectSmallSignalMode {
    fn from_gui_mode(mode: GuiSmallSignalMode) -> Self {
        match mode {
            GuiSmallSignalMode::CompactWhenAvailable => Self::CompactWhenAvailable,
            GuiSmallSignalMode::Expand => Self::Expand,
        }
    }

    fn into_gui_mode(self) -> GuiSmallSignalMode {
        match self {
            Self::CompactWhenAvailable => GuiSmallSignalMode::CompactWhenAvailable,
            Self::Expand => GuiSmallSignalMode::Expand,
        }
    }
}

impl GuiProjectTestbenchConnection {
    fn from_testbench_connection(connection: &GuiTestbenchConnection) -> Self {
        Self {
            from: GuiProjectTestbenchEndpoint::from_testbench_endpoint(&connection.from),
            to: GuiProjectTestbenchEndpoint::from_testbench_endpoint(&connection.to),
        }
    }

    fn into_testbench_connection(self) -> GuiTestbenchConnection {
        GuiTestbenchConnection {
            from: self.from.into_testbench_endpoint(),
            to: self.to.into_testbench_endpoint(),
        }
    }
}

impl GuiProjectTestbenchEndpoint {
    fn from_testbench_endpoint(endpoint: &TestbenchEndpoint) -> Self {
        match endpoint {
            TestbenchEndpoint::ElementPin { element_id, pin } => Self::ElementPin {
                element_id: *element_id,
                pin: GuiProjectTestbenchPin::from_testbench_pin(*pin),
            },
            TestbenchEndpoint::DutPort { port_name } => Self::DutPort {
                port_name: port_name.clone(),
            },
        }
    }

    fn into_testbench_endpoint(self) -> TestbenchEndpoint {
        match self {
            Self::ElementPin { element_id, pin } => TestbenchEndpoint::ElementPin {
                element_id,
                pin: pin.into_testbench_pin(),
            },
            Self::DutPort { port_name } => TestbenchEndpoint::DutPort { port_name },
        }
    }
}

impl GuiProjectTestbenchPin {
    fn from_testbench_pin(pin: TestbenchPin) -> Self {
        match pin {
            TestbenchPin::A => Self::A,
            TestbenchPin::B => Self::B,
        }
    }

    fn into_testbench_pin(self) -> TestbenchPin {
        match self {
            Self::A => TestbenchPin::A,
            Self::B => TestbenchPin::B,
        }
    }
}

impl GuiProjectTestbenchElement {
    fn from_testbench_element(element: &GuiTestbenchElement) -> Self {
        Self {
            id: element.id,
            kind: GuiProjectTestbenchElementKind::from_testbench_kind(element.kind),
            name: element.name.clone(),
            nplus: element.nplus.clone(),
            nminus: element.nminus.clone(),
            value: element.value.clone(),
            position: GuiProjectPosition::from_pos(element.position),
            orientation: GuiProjectOrientation::from_gui_orientation(element.orientation),
        }
    }

    fn into_testbench_element(self) -> GuiTestbenchElement {
        GuiTestbenchElement {
            id: self.id,
            kind: self.kind.into_testbench_kind(),
            name: self.name,
            nplus: self.nplus,
            nminus: self.nminus,
            value: self.value,
            position: self.position.to_pos(),
            orientation: self.orientation.into_gui_orientation(),
        }
    }
}

impl GuiProjectTestbenchElementKind {
    fn from_testbench_kind(kind: GuiTestbenchElementKind) -> Self {
        match kind {
            GuiTestbenchElementKind::VoltageSource => Self::VoltageSource,
            GuiTestbenchElementKind::CurrentSource => Self::CurrentSource,
            GuiTestbenchElementKind::Resistor => Self::Resistor,
            GuiTestbenchElementKind::Capacitor => Self::Capacitor,
        }
    }

    fn into_testbench_kind(self) -> GuiTestbenchElementKind {
        match self {
            Self::VoltageSource => GuiTestbenchElementKind::VoltageSource,
            Self::CurrentSource => GuiTestbenchElementKind::CurrentSource,
            Self::Resistor => GuiTestbenchElementKind::Resistor,
            Self::Capacitor => GuiTestbenchElementKind::Capacitor,
        }
    }
}

impl GuiProjectConnection {
    fn from_canvas_connection(connection: &CanvasConnection) -> Self {
        Self {
            from: GuiProjectEndpoint::from_canvas_endpoint(&connection.from),
            to: GuiProjectEndpoint::from_canvas_endpoint(&connection.to),
        }
    }

    fn into_canvas_connection(self) -> CanvasConnection {
        CanvasConnection {
            from: self.from.into_canvas_endpoint(),
            to: self.to.into_canvas_endpoint(),
        }
    }
}

impl GuiProjectEndpoint {
    fn from_canvas_endpoint(endpoint: &CanvasEndpoint) -> Self {
        match endpoint {
            CanvasEndpoint::PrimitivePin {
                instance_id,
                pin_name,
            } => Self::PrimitivePin {
                instance_id: *instance_id,
                pin: pin_name.clone(),
            },
            CanvasEndpoint::LabelPin { label_id } => Self::LabelPin {
                label_id: *label_id,
            },
            CanvasEndpoint::MacroPort { port_id } => Self::MacroPort { port_id: *port_id },
        }
    }

    fn into_canvas_endpoint(self) -> CanvasEndpoint {
        match self {
            Self::PrimitivePin { instance_id, pin } => CanvasEndpoint::PrimitivePin {
                instance_id,
                pin_name: pin,
            },
            Self::LabelPin { label_id } => CanvasEndpoint::LabelPin { label_id },
            Self::MacroPort { port_id } => CanvasEndpoint::MacroPort { port_id },
        }
    }
}

fn show_testbench_editor(
    ui: &mut egui::Ui,
    testbench_index: usize,
    testbench: &mut GuiTestbenchDocument,
) {
    ui.heading("Testbench");
    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.text_edit_singleline(&mut testbench.name);
    });

    ui.separator();
    ui.horizontal(|ui| {
        ui.heading("Elements");
        if ui.button("Add element").clicked() {
            testbench.add_element(GuiTestbenchElementKind::VoltageSource);
        }
    });

    let mut remove_element = None;
    let resolved_nodes = resolve_testbench_nodes(testbench, testbench_index + 1);
    for (element_index, element) in testbench.elements.iter_mut().enumerate() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt(format!(
                    "testbench_{testbench_index}_element_{element_index}_kind"
                ))
                .selected_text(element.kind.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut element.kind,
                        GuiTestbenchElementKind::VoltageSource,
                        GuiTestbenchElementKind::VoltageSource.label(),
                    );
                    ui.selectable_value(
                        &mut element.kind,
                        GuiTestbenchElementKind::CurrentSource,
                        GuiTestbenchElementKind::CurrentSource.label(),
                    );
                    ui.selectable_value(
                        &mut element.kind,
                        GuiTestbenchElementKind::Resistor,
                        GuiTestbenchElementKind::Resistor.label(),
                    );
                    ui.selectable_value(
                        &mut element.kind,
                        GuiTestbenchElementKind::Capacitor,
                        GuiTestbenchElementKind::Capacitor.label(),
                    );
                });

                if ui.button("Delete").clicked() {
                    remove_element = Some(element_index);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut element.name);
                ui.label(format!("Rot: {} deg", element.orientation.degrees()));
                ui.label(node_a_label(element.kind));
                ui.text_edit_singleline(&mut element.nplus);
                ui.label(node_b_label(element.kind));
                ui.text_edit_singleline(&mut element.nminus);
                ui.label("Value:");
                ui.text_edit_singleline(&mut element.value);
            });

            match &resolved_nodes {
                Ok(nodes) => {
                    ui.label(format!(
                        "Resolved: {} {}, {} {}",
                        node_a_label(element.kind).trim_end_matches(':'),
                        display_testbench_node(element, TestbenchPin::A, nodes),
                        node_b_label(element.kind).trim_end_matches(':'),
                        display_testbench_node(element, TestbenchPin::B, nodes),
                    ));
                }
                Err(error) => {
                    ui.colored_label(egui::Color32::from_rgb(230, 90, 90), error);
                }
            }
        });
    }

    if let Some(element_index) = remove_element {
        testbench.remove_element(element_index);
    }

    ui.separator();
    ui.horizontal(|ui| {
        ui.heading("Compact outputs");
        if ui.button("+ output").clicked() {
            testbench
                .compact_outputs
                .push(GuiCompactOutputBinding::default());
        }
    });
    let mut remove_binding = None;
    for (binding_index, binding) in testbench.compact_outputs.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.label("Source column:");
            ui.text_edit_singleline(&mut binding.source_column);
            ui.label("Compact parameter:");
            ui.text_edit_singleline(&mut binding.compact_parameter);
            if ui.button("Delete").clicked() {
                remove_binding = Some(binding_index);
            }
        });
    }
    if let Some(binding_index) = remove_binding {
        testbench.compact_outputs.remove(binding_index);
    }

    ui.separator();
    ui.label("Extra body:");
    ui.add(
        egui::TextEdit::multiline(&mut testbench.extra_body)
            .desired_rows(4)
            .code_editor(),
    );
}

fn node_a_label(kind: GuiTestbenchElementKind) -> &'static str {
    match kind {
        GuiTestbenchElementKind::VoltageSource | GuiTestbenchElementKind::CurrentSource => "N+:",
        GuiTestbenchElementKind::Resistor | GuiTestbenchElementKind::Capacitor => "N1:",
    }
}

fn node_b_label(kind: GuiTestbenchElementKind) -> &'static str {
    match kind {
        GuiTestbenchElementKind::VoltageSource | GuiTestbenchElementKind::CurrentSource => "N-:",
        GuiTestbenchElementKind::Resistor | GuiTestbenchElementKind::Capacitor => "N2:",
    }
}

fn node_label_without_colon(kind: GuiTestbenchElementKind, pin: TestbenchPin) -> &'static str {
    match pin {
        TestbenchPin::A => node_a_label(kind).trim_end_matches(':'),
        TestbenchPin::B => node_b_label(kind).trim_end_matches(':'),
    }
}

fn display_testbench_node(
    element: &GuiTestbenchElement,
    pin: TestbenchPin,
    resolved_nodes: &HashMap<TestbenchEndpoint, String>,
) -> String {
    let endpoint = TestbenchEndpoint::ElementPin {
        element_id: element.id,
        pin,
    };

    if let Some(node) = resolved_nodes.get(&endpoint) {
        return node.clone();
    }

    let manual_node = testbench_element_pin_text(element, pin).trim();
    if manual_node.is_empty() {
        "(unset)".to_string()
    } else {
        manual_node.to_string()
    }
}

fn gui_testbenches_to_specs(
    testbenches: &[GuiTestbenchDocument],
    circuits: &[GuiCircuitDocument],
) -> Result<Vec<TestbenchSpec>, String> {
    let mut specs = Vec::with_capacity(testbenches.len());

    for (index, testbench) in testbenches.iter().enumerate() {
        let name = required_text(&testbench.name, &format!("testbench {}", index + 1), "name")?;
        let dut_macro = required_text(
            &testbench.dut_macro,
            &format!("testbench {}", index + 1),
            "DUT macro",
        )?;
        if !circuits.iter().any(|circuit| circuit.name == dut_macro) {
            return Err(format!(
                "testbench {} references unknown DUT macro '{}'",
                index + 1,
                dut_macro
            ));
        }

        let mut spec = TestbenchSpec::new(name).with_macro_dut(dut_macro);
        let resolved_nodes = resolve_testbench_nodes(testbench, index + 1)?;

        for (element_index, element) in testbench.elements.iter().enumerate() {
            spec = spec.with_element(gui_testbench_element_to_spec(
                element,
                &resolved_nodes,
                index + 1,
                element_index + 1,
            )?);
        }

        if !testbench.extra_body.trim().is_empty() {
            spec = spec.with_extra_body(testbench.extra_body.trim().to_string());
        }

        for (binding_index, binding) in testbench.compact_outputs.iter().enumerate() {
            let source_column = required_text(
                &binding.source_column,
                &format!(
                    "testbench {} compact output {}",
                    index + 1,
                    binding_index + 1
                ),
                "source column",
            )?;
            let compact_parameter = required_text(
                &binding.compact_parameter,
                &format!(
                    "testbench {} compact output {}",
                    index + 1,
                    binding_index + 1
                ),
                "compact parameter",
            )?;
            spec = spec.with_compact_output(source_column, compact_parameter);
        }

        specs.push(spec);
    }

    Ok(specs)
}

fn ensure_testbench_dut_macros_for(
    testbenches: &mut [GuiTestbenchDocument],
    macro_names: &[String],
    default_dut: &str,
) {
    for testbench in testbenches {
        if testbench.dut_macro.trim().is_empty()
            || !macro_names.iter().any(|name| name == &testbench.dut_macro)
        {
            testbench.dut_macro = default_dut.to_string();
        }
    }
}

fn gui_specs_to_exploration_specs(
    specs: &[GuiSpecDocument],
    testbenches: &[TestbenchSpec],
) -> Result<Vec<ExplorationSpec>, String> {
    specs
        .iter()
        .enumerate()
        .map(|(index, spec)| gui_spec_to_exploration_spec(spec, testbenches, index + 1))
        .collect()
}

fn gui_derived_columns_to_specs(
    columns: &[GuiDerivedColumnDocument],
) -> Result<Vec<DerivedColumnSpec>, String> {
    columns
        .iter()
        .enumerate()
        .map(|(index, column)| {
            let context = format!("derived column {}", index + 1);
            Ok(DerivedColumnSpec::new(
                required_text(&column.name, &context, "name")?,
                required_text(&column.expression, &context, "expression")?,
            ))
        })
        .collect()
}

fn gui_candidate_input(
    candidates: &GuiCandidateDocument,
    document: &GuiCircuitDocument,
    catalog: &PrimitiveCatalog,
    macro_blocks: &[GuiDutMacroView],
) -> Result<ExplorationCandidateInput, String> {
    let axes = candidates
        .axes
        .iter()
        .enumerate()
        .map(|(index, axis)| gui_candidate_axis(axis, index + 1))
        .collect::<Result<Vec<_>, _>>()?;
    let sets = gui_primitive_candidate_sets(candidates, document, catalog, macro_blocks)?;

    Ok(ExplorationCandidateInput {
        axes,
        sets,
        filters: Vec::new(),
    })
}

fn gui_primitive_candidate_sets(
    candidates: &GuiCandidateDocument,
    document: &GuiCircuitDocument,
    catalog: &PrimitiveCatalog,
    macro_blocks: &[GuiDutMacroView],
) -> Result<Vec<CandidateSet>, String> {
    let build_instances = document
        .canvas_instances
        .iter()
        .filter_map(|instance| {
            let primitive = instance
                .block
                .primitive_name()
                .and_then(|name| catalog.get(name))?;
            primitive.build.as_ref()?;
            Some((instance, primitive))
        })
        .collect::<Vec<_>>();

    if build_instances.is_empty() {
        return Ok(Vec::new());
    }

    let net_index = CanvasNetIndex::from_document(document, catalog, macro_blocks);
    let voltage_constraints = gui_net_voltage_constraint_map(candidates)?;
    let global_parameters = gui_build_parameter_map(
        &candidates.global_build_parameters,
        "global build parameter",
    )?;
    let backend = gui_python_gmid_backend(candidates)?;
    let engine = PrimitiveBuildEngine::new(backend);
    let mut sets = Vec::with_capacity(build_instances.len());

    for (instance, primitive) in build_instances {
        let input = gui_primitive_build_input(
            candidates,
            instance,
            primitive,
            &net_index,
            &voltage_constraints,
            &global_parameters,
        )?;
        let instance_name = exported_instance_name(instance);
        let set = engine
            .build_candidate_set_for_primitive(primitive, &instance_name, &input)
            .map_err(|error| {
                format!(
                    "failed to build candidates for instance '{}' ({})\n\n{error:?}",
                    instance_name, primitive.name
                )
            })?;
        sets.push(set);
    }

    Ok(sets)
}

fn gui_primitive_build_input(
    candidates: &GuiCandidateDocument,
    instance: &CanvasInstance,
    primitive: &PrimitiveManifest,
    net_index: &CanvasNetIndex,
    voltage_constraints: &HashMap<String, PrimitiveBuildValue>,
    global_parameters: &HashMap<String, PrimitiveBuildValue>,
) -> Result<PrimitiveBuildInput, String> {
    let Some(build) = &primitive.build else {
        return Err(format!("primitive '{}' has no build spec", primitive.name));
    };
    let override_parameters = candidates
        .primitive_build_overrides
        .iter()
        .find(|override_| override_.instance_id == instance.id)
        .map(|override_| {
            gui_build_parameter_map(
                &override_.parameters,
                &format!(
                    "override for instance '{}'",
                    exported_instance_name(instance)
                ),
            )
        })
        .transpose()?
        .unwrap_or_default();

    let mut values = HashMap::new();
    for input in &build.inputs {
        if input.source.as_deref() == Some("port_voltage") {
            let endpoint = CanvasEndpoint::PrimitivePin {
                instance_id: instance.id,
                pin_name: input.name.clone(),
            };
            let net = net_index.net_for_endpoint(&endpoint).ok_or_else(|| {
                format!(
                    "instance '{}' input '{}' is a port_voltage but the pin is not known in the active canvas",
                    exported_instance_name(instance),
                    input.name
                )
            })?;
            let value = voltage_constraints.get(net).ok_or_else(|| {
                format!(
                    "missing voltage constraint for net '{}' used by instance '{}' input '{}'",
                    net,
                    exported_instance_name(instance),
                    input.name
                )
            })?;
            values.insert(input.name.clone(), value.clone());
            continue;
        }

        if let Some(value) = override_parameters.get(&input.name) {
            values.insert(input.name.clone(), value.clone());
        } else if let Some(value) = global_parameters.get(&input.name) {
            values.insert(input.name.clone(), value.clone());
        } else if input.required {
            return Err(format!(
                "missing build parameter '{}' for instance '{}'; define it globally or as an instance override",
                input.name,
                exported_instance_name(instance)
            ));
        }
    }

    let mut build_input = PrimitiveBuildInput::new(values);
    if let Some(lut_config) = primitive.lut_config.clone() {
        build_input = build_input.with_lut_config(lut_config);
    }
    Ok(build_input)
}

fn gui_python_gmid_backend(
    candidates: &GuiCandidateDocument,
) -> Result<PythonGmidLutBackend, String> {
    let python = required_text(
        &candidates.python_path,
        "Python GMID backend",
        "python path",
    )?;
    let nmos = required_text(
        &candidates.nmos_lut_path,
        "Python GMID backend",
        "NMOS LUT path",
    )?;
    let pmos = required_text(
        &candidates.pmos_lut_path,
        "Python GMID backend",
        "PMOS LUT path",
    )?;
    let python = resolve_workspace_relative_path(&python);
    let nmos = resolve_workspace_relative_path(&nmos);
    let pmos = resolve_workspace_relative_path(&pmos);

    Ok(PythonGmidLutBackend::with_default_helper(
        python,
        HashMap::from([("nmos".to_string(), nmos), ("pmos".to_string(), pmos)]),
    )
    .with_timing_output(candidates.timing_output))
}

fn gui_net_voltage_constraint_map(
    candidates: &GuiCandidateDocument,
) -> Result<HashMap<String, PrimitiveBuildValue>, String> {
    let mut values = HashMap::new();
    for (index, constraint) in candidates.net_voltage_constraints.iter().enumerate() {
        let context = format!("net voltage constraint {}", index + 1);
        let net = required_text(&constraint.net, &context, "net")?;
        if values.contains_key(&net) {
            return Err(format!("duplicate voltage constraint for net '{net}'"));
        }
        values.insert(
            net,
            primitive_build_value_from_text(
                &constraint.values,
                PrimitiveBuildInputKind::Vector,
                &context,
            )?,
        );
    }
    Ok(values)
}

fn gui_build_parameter_map(
    parameters: &[GuiBuildParameter],
    owner: &str,
) -> Result<HashMap<String, PrimitiveBuildValue>, String> {
    let mut values = HashMap::new();
    for (index, parameter) in parameters.iter().enumerate() {
        let context = format!("{owner} {}", index + 1);
        let name = required_text(&parameter.name, &context, "name")?;
        if values.contains_key(&name) {
            return Err(format!("duplicate build parameter '{name}' in {owner}"));
        }
        values.insert(
            name,
            primitive_build_value_from_text(
                &parameter.values,
                PrimitiveBuildInputKind::Scalar,
                &context,
            )?,
        );
    }
    Ok(values)
}

fn primitive_build_value_from_text(
    input: &str,
    default_kind: PrimitiveBuildInputKind,
    context: &str,
) -> Result<PrimitiveBuildValue, String> {
    let values = parse_candidate_values(input, context)?;
    if default_kind == PrimitiveBuildInputKind::Scalar && values.len() == 1 {
        Ok(PrimitiveBuildValue::Scalar(values[0]))
    } else {
        Ok(PrimitiveBuildValue::Vector(values))
    }
}

#[derive(Clone)]
struct CanvasNetIndex {
    nets: Vec<CanvasNet>,
    endpoint_to_net: HashMap<CanvasEndpoint, String>,
}

#[derive(Clone)]
struct CanvasNet {
    name: String,
}

impl CanvasNetIndex {
    fn from_document(
        document: &GuiCircuitDocument,
        catalog: &PrimitiveCatalog,
        macro_blocks: &[GuiDutMacroView],
    ) -> Self {
        let mut nets = Vec::new();
        let mut endpoint_to_net = HashMap::new();
        let mut seen_endpoints = HashMap::new();

        for (net_index, endpoints) in connected_endpoint_groups(&document.connections)
            .into_iter()
            .enumerate()
        {
            let name = canvas_net_name(&endpoints, &document.label_pins, &document.macro_ports)
                .unwrap_or_else(|| format!("N{}", net_index + 1));
            for endpoint in &endpoints {
                endpoint_to_net.insert(endpoint.clone(), name.clone());
                seen_endpoints.insert(endpoint.clone(), true);
            }
            nets.push(CanvasNet { name });
        }

        for endpoint in document_canvas_endpoints(document, catalog, macro_blocks) {
            if seen_endpoints.contains_key(&endpoint) {
                continue;
            }
            let name = isolated_endpoint_net_name(&endpoint, document);
            endpoint_to_net.insert(endpoint.clone(), name.clone());
            seen_endpoints.insert(endpoint.clone(), true);
            nets.push(CanvasNet { name });
        }

        nets.sort_by(|left, right| left.name.cmp(&right.name));
        Self {
            nets,
            endpoint_to_net,
        }
    }

    fn net_for_endpoint(&self, endpoint: &CanvasEndpoint) -> Option<&str> {
        self.endpoint_to_net.get(endpoint).map(String::as_str)
    }

    fn nets(&self) -> &[CanvasNet] {
        &self.nets
    }
}

fn document_canvas_endpoints(
    document: &GuiCircuitDocument,
    catalog: &PrimitiveCatalog,
    macro_blocks: &[GuiDutMacroView],
) -> Vec<CanvasEndpoint> {
    let mut endpoints = Vec::new();
    for instance in &document.canvas_instances {
        if let Some(primitive_name) = instance.block.primitive_name() {
            if let Some(primitive) = catalog.get(primitive_name) {
                for pin in &primitive.pins {
                    endpoints.push(CanvasEndpoint::PrimitivePin {
                        instance_id: instance.id,
                        pin_name: pin.name.clone(),
                    });
                }
            }
        } else if let Some(macro_name) = instance.block.macro_name() {
            if let Some(macro_block) = macro_blocks
                .iter()
                .find(|macro_block| macro_block.name == macro_name)
            {
                for port in &macro_block.ports {
                    endpoints.push(CanvasEndpoint::PrimitivePin {
                        instance_id: instance.id,
                        pin_name: port.name.clone(),
                    });
                }
            }
        }
    }
    endpoints.extend(
        document
            .label_pins
            .iter()
            .map(|label_pin| CanvasEndpoint::LabelPin {
                label_id: label_pin.id,
            }),
    );
    endpoints.extend(
        document
            .macro_ports
            .iter()
            .map(|macro_port| CanvasEndpoint::MacroPort {
                port_id: macro_port.id,
            }),
    );
    endpoints
}

fn isolated_endpoint_net_name(endpoint: &CanvasEndpoint, document: &GuiCircuitDocument) -> String {
    match endpoint {
        CanvasEndpoint::PrimitivePin {
            instance_id,
            pin_name,
        } => {
            let instance_name = document
                .canvas_instances
                .iter()
                .find(|instance| instance.id == *instance_id)
                .map(exported_instance_name)
                .unwrap_or_else(|| circuit_instance_id(*instance_id));
            format!("{instance_name}.{pin_name}")
        }
        CanvasEndpoint::LabelPin { label_id } => document
            .label_pins
            .iter()
            .find(|label_pin| label_pin.id == *label_id)
            .map(|label_pin| label_pin.name.trim().to_string())
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| format!("label_{label_id}")),
        CanvasEndpoint::MacroPort { port_id } => document
            .macro_ports
            .iter()
            .find(|macro_port| macro_port.id == *port_id)
            .map(|macro_port| macro_port.name.trim().to_string())
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| format!("port_{port_id}")),
    }
}

fn gui_candidate_axis(axis: &GuiCandidateAxis, axis_index: usize) -> Result<CandidateAxis, String> {
    let context = format!("candidate axis {axis_index}");
    let name = required_text(&axis.name, &context, "name")?;
    let values = parse_candidate_values(&axis.values, &context)?;

    Ok(CandidateAxis::new(name, values))
}

fn parse_candidate_values(input: &str, context: &str) -> Result<Vec<f64>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(format!("{context} has no values"));
    }

    if let Some(values) = parse_linspace_values(trimmed, context)? {
        return Ok(values);
    }

    trimmed
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter(|token| !token.trim().is_empty())
        .map(|token| {
            token.trim().parse::<f64>().map_err(|error| {
                format!(
                    "{context} has invalid numeric value '{}': {error}",
                    token.trim()
                )
            })
        })
        .collect()
}

fn parse_linspace_values(input: &str, context: &str) -> Result<Option<Vec<f64>>, String> {
    let Some(arguments) = input
        .strip_prefix("linspace(")
        .and_then(|value| value.strip_suffix(')'))
    else {
        return Ok(None);
    };

    let parts = arguments
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    if parts.len() != 3 {
        return Err(format!(
            "{context} linspace expects exactly 3 arguments: linspace(start, stop, count)"
        ));
    }

    let start = parts[0].parse::<f64>().map_err(|error| {
        format!(
            "{context} has invalid linspace start '{}': {error}",
            parts[0]
        )
    })?;
    let stop = parts[1].parse::<f64>().map_err(|error| {
        format!(
            "{context} has invalid linspace stop '{}': {error}",
            parts[1]
        )
    })?;
    let count = parts[2].parse::<usize>().map_err(|error| {
        format!(
            "{context} has invalid linspace count '{}': {error}",
            parts[2]
        )
    })?;

    if count == 0 {
        return Err(format!(
            "{context} linspace count must be greater than zero"
        ));
    }

    if count == 1 {
        return Ok(Some(vec![start]));
    }

    let step = (stop - start) / (count - 1) as f64;
    Ok(Some(
        (0..count)
            .map(|index| start + step * index as f64)
            .collect(),
    ))
}

fn gui_specs_small_signal_mode(
    specs: &[GuiSpecDocument],
    testbenches: &[GuiTestbenchDocument],
) -> Result<GuiSmallSignalMode, String> {
    let mut selected_mode: Option<GuiSmallSignalMode> = None;

    for (index, spec) in specs.iter().enumerate() {
        let testbench_name =
            required_text(&spec.testbench, &format!("spec {}", index + 1), "testbench")?;
        let testbench = testbenches
            .iter()
            .find(|testbench| testbench.name == testbench_name)
            .ok_or_else(|| {
                format!(
                    "spec {} references unknown testbench '{testbench_name}'",
                    index + 1
                )
            })?;

        match selected_mode {
            Some(mode) if mode != testbench.small_signal_mode => {
                return Err(format!(
                    "specs reference testbenches with mixed small-signal modes ('{}' and '{}'); prepare specs currently expects one mode per run",
                    mode.label(),
                    testbench.small_signal_mode.label()
                ));
            }
            Some(_) => {}
            None => selected_mode = Some(testbench.small_signal_mode),
        }
    }

    Ok(selected_mode.unwrap_or(GuiSmallSignalMode::CompactWhenAvailable))
}

fn compact_output_bindings_for_workspace(
    workspace: &GuiMacroWorkspace,
) -> Result<Vec<CompactOutputBinding>, String> {
    let mut bindings = Vec::new();

    for (testbench_index, testbench) in workspace.testbenches.iter().enumerate() {
        for (binding_index, binding) in testbench.compact_outputs.iter().enumerate() {
            let source_column = required_text(
                &binding.source_column,
                &format!(
                    "testbench {} compact output {}",
                    testbench_index + 1,
                    binding_index + 1
                ),
                "source column",
            )?;
            let compact_parameter = required_text(
                &binding.compact_parameter,
                &format!(
                    "testbench {} compact output {}",
                    testbench_index + 1,
                    binding_index + 1
                ),
                "compact parameter",
            )?;
            bindings.push(CompactOutputBinding::new(source_column, compact_parameter));
        }
    }

    if bindings.is_empty() {
        return Err("submacro workspace has no compact outputs configured".to_string());
    }

    Ok(bindings)
}

fn format_hierarchy_candidates_output(candidate_input: &ExplorationCandidateInput) -> String {
    format!(
        "Candidate axes: {}\nCandidate sets: {}\nFilters: {}",
        candidate_input.axes.len(),
        candidate_input.sets.len(),
        candidate_input.filters.len()
    )
}

fn gui_spec_to_exploration_spec(
    spec: &GuiSpecDocument,
    testbenches: &[TestbenchSpec],
    spec_index: usize,
) -> Result<ExplorationSpec, String> {
    let context = format!("spec {spec_index}");
    let name = required_text(&spec.name, &context, "name")?;
    let testbench_name = required_text(&spec.testbench, &context, "testbench")?;
    let testbench = testbenches
        .iter()
        .find(|testbench| testbench.name == testbench_name)
        .cloned()
        .ok_or_else(|| {
            format!("spec {spec_index} references unknown testbench '{testbench_name}'")
        })?;
    let source = match spec.source_kind {
        GuiSpecSourceKind::TransferFunction => {
            let input = required_text(&spec.input, &context, "input")?;
            let output = required_text(&spec.output, &context, "output")?;

            SpecSource::TransferFunction {
                testbench,
                input,
                output,
            }
        }
        GuiSpecSourceKind::NodeVoltage => {
            let node = required_text(&spec.node, &context, "node")?;

            SpecSource::NodeVoltage { testbench, node }
        }
    };
    let mut exploration_spec = ExplorationSpec::new(
        name,
        RangeCondition::new(
            optional_f64(&spec.min, &context, "min")?,
            optional_f64(&spec.max, &context, "max")?,
        ),
        source,
        SpecOutput::Eval,
    );

    for (parameter_index, parameter) in spec.parameter_map.iter().enumerate() {
        let parameter_context = format!("{context}, parameter {}", parameter_index + 1);
        let name = required_text(&parameter.name, &parameter_context, "name")?;
        let value = required_text(&parameter.value, &parameter_context, "value")?;
        exploration_spec
            .parameter_map
            .push(SpecParameter::new(name, value));
    }

    Ok(exploration_spec)
}

fn gui_testbench_element_to_spec(
    element: &GuiTestbenchElement,
    resolved_nodes: &HashMap<TestbenchEndpoint, String>,
    testbench_index: usize,
    element_index: usize,
) -> Result<TestbenchElement, String> {
    let context = format!("testbench {testbench_index}, element {element_index}");
    let name = required_text(&element.name, &context, "name")?;
    let nplus = resolved_testbench_node(
        element,
        TestbenchPin::A,
        resolved_nodes,
        &context,
        node_a_label(element.kind),
    )?;
    let nminus = resolved_testbench_node(
        element,
        TestbenchPin::B,
        resolved_nodes,
        &context,
        node_b_label(element.kind),
    )?;
    let value = required_text(&element.value, &context, "value")?;

    Ok(match element.kind {
        GuiTestbenchElementKind::VoltageSource => TestbenchElement::VoltageSource {
            name,
            nplus,
            nminus,
            value,
        },
        GuiTestbenchElementKind::CurrentSource => TestbenchElement::CurrentSource {
            name,
            nplus,
            nminus,
            value,
        },
        GuiTestbenchElementKind::Resistor => TestbenchElement::Resistor {
            name,
            n1: nplus,
            n2: nminus,
            value,
        },
        GuiTestbenchElementKind::Capacitor => TestbenchElement::Capacitor {
            name,
            n1: nplus,
            n2: nminus,
            value,
        },
    })
}

fn resolve_testbench_nodes(
    testbench: &GuiTestbenchDocument,
    testbench_index: usize,
) -> Result<HashMap<TestbenchEndpoint, String>, String> {
    let mut adjacency: HashMap<TestbenchEndpoint, Vec<TestbenchEndpoint>> = HashMap::new();

    for element in &testbench.elements {
        for pin in [TestbenchPin::A, TestbenchPin::B] {
            adjacency
                .entry(TestbenchEndpoint::ElementPin {
                    element_id: element.id,
                    pin,
                })
                .or_default();
        }
    }

    for connection in &testbench.connections {
        if !testbench_connection_endpoint_exists(&connection.from, &testbench.elements)
            || !testbench_connection_endpoint_exists(&connection.to, &testbench.elements)
        {
            continue;
        }

        adjacency
            .entry(connection.from.clone())
            .or_default()
            .push(connection.to.clone());
        adjacency
            .entry(connection.to.clone())
            .or_default()
            .push(connection.from.clone());
    }

    let mut resolved = HashMap::new();
    let mut visited = HashMap::new();
    let mut next_generated_net = 1;

    for endpoint in adjacency.keys() {
        if visited.contains_key(endpoint) {
            continue;
        }

        let mut stack = vec![endpoint.clone()];
        let mut component = Vec::new();

        while let Some(current) = stack.pop() {
            if visited.insert(current.clone(), true).is_some() {
                continue;
            }

            component.push(current.clone());
            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains_key(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }

        let mut explicit_nodes = Vec::new();
        for endpoint in &component {
            if let Some(node) = testbench_endpoint_node_text(testbench, endpoint) {
                if !explicit_nodes.iter().any(|existing| existing == node) {
                    explicit_nodes.push(node.to_string());
                }
            }
        }

        let node = match explicit_nodes.as_slice() {
            [node] => Some(node.clone()),
            [] if component.len() > 1 => {
                let node = format!("tb_net_{next_generated_net}");
                next_generated_net += 1;
                Some(node)
            }
            [] => None,
            _ => {
                let endpoints = component
                    .iter()
                    .map(|endpoint| format_testbench_endpoint(testbench, endpoint))
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(format!(
                    "testbench {testbench_index} has conflicting node names on connected pins: {endpoints}"
                ));
            }
        };

        if let Some(node) = node {
            for endpoint in component {
                resolved.insert(endpoint, node.clone());
            }
        }
    }

    Ok(resolved)
}

fn resolved_testbench_node(
    element: &GuiTestbenchElement,
    pin: TestbenchPin,
    resolved_nodes: &HashMap<TestbenchEndpoint, String>,
    context: &str,
    field: &str,
) -> Result<String, String> {
    let endpoint = TestbenchEndpoint::ElementPin {
        element_id: element.id,
        pin,
    };

    if let Some(node) = resolved_nodes.get(&endpoint) {
        return Ok(node.clone());
    }

    required_text(testbench_element_pin_text(element, pin), context, field)
}

fn testbench_endpoint_node_text<'a>(
    testbench: &'a GuiTestbenchDocument,
    endpoint: &'a TestbenchEndpoint,
) -> Option<&'a str> {
    match endpoint {
        TestbenchEndpoint::ElementPin { element_id, pin } => {
            let element = testbench
                .elements
                .iter()
                .find(|element| element.id == *element_id)?;
            let value = testbench_element_pin_text(element, *pin).trim();

            if value.is_empty() { None } else { Some(value) }
        }
        TestbenchEndpoint::DutPort { port_name } => {
            let value = port_name.trim();

            if value.is_empty() { None } else { Some(value) }
        }
    }
}

fn testbench_element_pin_text(element: &GuiTestbenchElement, pin: TestbenchPin) -> &str {
    match pin {
        TestbenchPin::A => &element.nplus,
        TestbenchPin::B => &element.nminus,
    }
}

fn format_testbench_endpoint(
    testbench: &GuiTestbenchDocument,
    endpoint: &TestbenchEndpoint,
) -> String {
    match endpoint {
        TestbenchEndpoint::ElementPin { element_id, pin } => {
            let element_name = testbench
                .elements
                .iter()
                .find(|element| element.id == *element_id)
                .map(|element| element.name.as_str())
                .unwrap_or("unknown");
            let pin_name = match pin {
                TestbenchPin::A => "A",
                TestbenchPin::B => "B",
            };

            format!("{element_name}.{pin_name}")
        }
        TestbenchEndpoint::DutPort { port_name } => format!("DUT.{port_name}"),
    }
}

fn required_text(value: &str, owner: &str, field: &str) -> Result<String, String> {
    let value = value.trim();

    if value.is_empty() {
        return Err(format!("{owner} has empty {field}"));
    }

    Ok(value.to_string())
}

fn optional_f64(value: &str, owner: &str, field: &str) -> Result<Option<f64>, String> {
    let value = value.trim();

    if value.is_empty() {
        return Ok(None);
    }

    value
        .parse::<f64>()
        .map(Some)
        .map_err(|error| format!("{owner} has invalid {field} '{value}': {error}"))
}

fn show_primitive_details(ui: &mut egui::Ui, primitive: &PrimitiveManifest) {
    ui.label(&primitive.name);
    ui.label(format!("Subckt: {}", primitive.subckt_name));
    ui.label(format!("Version: {}", primitive.version));

    if let Some(description) = &primitive.description {
        ui.label(description);
    }

    ui.collapsing("Pins", |ui| {
        for pin in &primitive.pins {
            ui.horizontal(|ui| {
                ui.label(&pin.name);
                ui.label(pin_role_label(&pin.role));
            });
        }
    });
}

fn draw_primitive_preview(ui: &mut egui::Ui, primitive: &PrimitiveManifest) {
    let preview_size = egui::vec2(200.0, 96.0);
    let (rect, _) = ui.allocate_exact_size(preview_size, egui::Sense::hover());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 4.0, egui::Color32::from_gray(28));

    let symbol_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(150.0, 58.0));
    painter.rect_filled(symbol_rect, 4.0, egui::Color32::from_rgb(45, 49, 56));
    painter.rect_stroke(
        symbol_rect,
        4.0,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(130, 150, 170)),
        egui::StrokeKind::Inside,
    );
    painter.text(
        symbol_rect.center_top() + egui::vec2(0.0, 16.0),
        egui::Align2::CENTER_TOP,
        &primitive.name,
        egui::FontId::proportional(13.0),
        egui::Color32::WHITE,
    );

    draw_instance_pins(
        &painter,
        symbol_rect,
        0,
        primitive,
        GuiOrientation::R0,
        None,
    );
}

fn show_macro_block_details(ui: &mut egui::Ui, macro_block: &GuiDutMacroView) {
    ui.label(&macro_block.name);
    ui.label(format!("Ports: {}", macro_block.ports.len()));

    ui.collapsing("Ports", |ui| {
        for port in &macro_block.ports {
            ui.horizontal(|ui| {
                ui.label(&port.name);
                ui.label(port.role.label());
            });
        }
    });
}

fn draw_macro_block_preview(ui: &mut egui::Ui, macro_block: &GuiDutMacroView) {
    let preview_size = egui::vec2(200.0, 96.0);
    let (rect, _) = ui.allocate_exact_size(preview_size, egui::Sense::hover());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 4.0, egui::Color32::from_gray(28));

    let symbol_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(150.0, 58.0));
    painter.rect_filled(symbol_rect, 4.0, egui::Color32::from_rgb(45, 49, 56));
    painter.rect_stroke(
        symbol_rect,
        4.0,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(130, 150, 170)),
        egui::StrokeKind::Inside,
    );
    painter.text(
        symbol_rect.center_top() + egui::vec2(0.0, 16.0),
        egui::Align2::CENTER_TOP,
        &macro_block.name,
        egui::FontId::proportional(13.0),
        egui::Color32::WHITE,
    );

    let pin_views = macro_pin_views(symbol_rect, macro_block, GuiOrientation::R0);
    draw_instance_pin_views(&painter, 0, &pin_views, None);
}

fn draw_testbench_canvas(
    ui: &mut egui::Ui,
    testbench: &mut GuiTestbenchDocument,
    dut_macro: Option<&GuiDutMacroView>,
) {
    let canvas_size = testbench_canvas_size(ui.available_width(), testbench);
    let (canvas_rect, _) = ui.allocate_exact_size(canvas_size, egui::Sense::hover());
    let painter = ui.painter_at(canvas_rect);

    painter.rect_filled(canvas_rect, 0.0, egui::Color32::from_gray(24));
    let dut_rect = dut_macro_rect(canvas_rect, testbench.dut_position);
    draw_dut_macro_symbol(
        &painter,
        dut_rect,
        dut_macro,
        testbench.selected_endpoint.as_ref(),
    );

    let mut clicked_endpoint = None;
    if let Some(dut_macro) = dut_macro {
        let dut_response = ui.allocate_rect(dut_rect, egui::Sense::click_and_drag());
        if dut_response.dragged() {
            testbench.dut_position += dut_response.drag_delta();
        }
        for port in &dut_macro.ports {
            let position = dut_port_position(dut_rect, port.symbol_side, port.symbol_offset);
            let hit_rect = egui::Rect::from_center_size(position, egui::vec2(14.0, 14.0));
            let response = ui.allocate_rect(hit_rect, egui::Sense::click());

            if response.clicked() {
                clicked_endpoint = Some(TestbenchEndpoint::DutPort {
                    port_name: port.name.clone(),
                });
            }
        }
    }

    if testbench.elements.is_empty() {
        painter.text(
            canvas_rect.center_bottom() - egui::vec2(0.0, 22.0),
            egui::Align2::CENTER_CENTER,
            "Insert a testbench element",
            egui::FontId::proportional(14.0),
            egui::Color32::from_gray(210),
        );
        if let Some(endpoint) = clicked_endpoint {
            testbench.selected_endpoint = Some(endpoint.clone());
            update_pending_testbench_connection(
                &mut testbench.pending_connection,
                &mut testbench.connections,
                endpoint,
            );
        }
        return;
    }

    draw_testbench_connections(
        &painter,
        canvas_rect,
        &testbench.elements,
        dut_macro,
        testbench.dut_position,
        &testbench.connections,
    );

    let resolved_nodes = resolve_testbench_nodes(testbench, 0).ok();
    for element in &mut testbench.elements {
        let center = canvas_rect.min + element.position.to_vec2();
        let rect = egui::Rect::from_center_size(center, egui::vec2(88.0, 54.0));
        let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());

        if response.clicked() || response.dragged() {
            testbench.selected_endpoint = Some(TestbenchEndpoint::ElementPin {
                element_id: element.id,
                pin: TestbenchPin::A,
            });
        }

        if response.dragged() {
            element.position += response.drag_delta();
        }

        for pin in [TestbenchPin::A, TestbenchPin::B] {
            let endpoint = TestbenchEndpoint::ElementPin {
                element_id: element.id,
                pin,
            };
            let pin_position = testbench_pin_position(rect, element.orientation, pin);
            let pin_rect = egui::Rect::from_center_size(pin_position, egui::vec2(14.0, 14.0));
            let pin_response = ui.allocate_rect(pin_rect, egui::Sense::click());

            if pin_response.clicked() {
                clicked_endpoint = Some(endpoint);
            }
        }

        draw_testbench_element_symbol(
            &painter,
            rect,
            element,
            testbench.selected_endpoint.as_ref(),
            testbench_element_has_canvas_issue(element, resolved_nodes.as_ref()),
        );
    }

    if let Some(endpoint) = clicked_endpoint {
        testbench.selected_endpoint = Some(endpoint.clone());
        update_pending_testbench_connection(
            &mut testbench.pending_connection,
            &mut testbench.connections,
            endpoint,
        );
    }
}

fn draw_dut_macro_symbol(
    painter: &egui::Painter,
    rect: egui::Rect,
    dut_macro: Option<&GuiDutMacroView>,
    selected_endpoint: Option<&TestbenchEndpoint>,
) {
    let Some(dut_macro) = dut_macro else {
        return;
    };

    let stroke = egui::Stroke::new(1.5, egui::Color32::from_rgb(205, 175, 90));
    let fill = egui::Color32::from_rgb(54, 50, 38);
    let port_color = egui::Color32::from_rgb(235, 195, 105);

    painter.rect_filled(rect, 3.0, fill);
    painter.rect_stroke(rect, 3.0, stroke, egui::StrokeKind::Inside);
    painter.text(
        rect.center_top() + egui::vec2(0.0, 24.0),
        egui::Align2::CENTER_CENTER,
        "DUT",
        egui::FontId::proportional(16.0),
        egui::Color32::WHITE,
    );
    painter.text(
        rect.center() + egui::vec2(0.0, 12.0),
        egui::Align2::CENTER_CENTER,
        &dut_macro.name,
        egui::FontId::proportional(12.0),
        egui::Color32::from_gray(220),
    );

    for port in &dut_macro.ports {
        let position = dut_port_position(rect, port.symbol_side, port.symbol_offset);
        let selected = selected_endpoint
            == Some(&TestbenchEndpoint::DutPort {
                port_name: port.name.clone(),
            });
        painter.circle_filled(
            position,
            if selected { 5.0 } else { 3.5 },
            if selected {
                egui::Color32::from_rgb(245, 200, 80)
            } else {
                port_color
            },
        );
        draw_dut_port_label(painter, position, port.symbol_side, &port.name);
    }
}

fn dut_macro_rect(canvas_rect: egui::Rect, dut_position: egui::Pos2) -> egui::Rect {
    let size = egui::vec2(150.0, 106.0);
    egui::Rect::from_min_size(canvas_rect.min + dut_position.to_vec2(), size)
}

fn testbench_canvas_size(available_width: f32, testbench: &GuiTestbenchDocument) -> egui::Vec2 {
    let mut max_x = available_width.max(640.0);
    let mut max_y: f32 = 260.0;

    let dut_size = egui::vec2(150.0, 106.0);
    max_x = max_x.max(testbench.dut_position.x + dut_size.x + 96.0);
    max_y = max_y.max(testbench.dut_position.y + dut_size.y + 96.0);

    for element in &testbench.elements {
        max_x = max_x.max(element.position.x + 120.0);
        max_y = max_y.max(element.position.y + 96.0);
    }

    egui::vec2(max_x, max_y)
}

fn dut_port_position(rect: egui::Rect, side: SymbolPinSide, offset: f32) -> egui::Pos2 {
    let offset = offset.clamp(0.0, 1.0);

    match side {
        SymbolPinSide::Left => egui::pos2(rect.left(), rect.top() + rect.height() * offset),
        SymbolPinSide::Right => egui::pos2(rect.right(), rect.top() + rect.height() * offset),
        SymbolPinSide::Top => egui::pos2(rect.left() + rect.width() * offset, rect.top()),
        SymbolPinSide::Bottom => egui::pos2(rect.left() + rect.width() * offset, rect.bottom()),
    }
}

fn draw_dut_port_label(
    painter: &egui::Painter,
    position: egui::Pos2,
    side: SymbolPinSide,
    name: &str,
) {
    let text_color = egui::Color32::from_gray(225);
    let font = egui::FontId::proportional(10.0);
    let (offset, align) = match side {
        SymbolPinSide::Left => (egui::vec2(7.0, 0.0), egui::Align2::LEFT_CENTER),
        SymbolPinSide::Right => (egui::vec2(-7.0, 0.0), egui::Align2::RIGHT_CENTER),
        SymbolPinSide::Top => (egui::vec2(0.0, 7.0), egui::Align2::CENTER_TOP),
        SymbolPinSide::Bottom => (egui::vec2(0.0, -7.0), egui::Align2::CENTER_BOTTOM),
    };

    painter.text(position + offset, align, name, font, text_color);
}

fn draw_testbench_element_symbol(
    painter: &egui::Painter,
    rect: egui::Rect,
    element: &GuiTestbenchElement,
    selected_endpoint: Option<&TestbenchEndpoint>,
    has_issue: bool,
) {
    let stroke = egui::Stroke::new(1.5, egui::Color32::from_rgb(130, 150, 170));
    let body_color = egui::Color32::from_rgb(45, 49, 56);
    let pin_color = egui::Color32::from_rgb(120, 210, 150);
    let selected_pin_color = egui::Color32::from_rgb(245, 200, 80);
    let element_selected = selected_endpoint.is_some_and(|endpoint| match endpoint {
        TestbenchEndpoint::ElementPin { element_id, .. } => *element_id == element.id,
        TestbenchEndpoint::DutPort { .. } => false,
    });
    let stroke = if has_issue {
        egui::Stroke::new(2.0, egui::Color32::from_rgb(220, 85, 75))
    } else if element_selected {
        egui::Stroke::new(2.0, egui::Color32::from_rgb(220, 180, 80))
    } else {
        stroke
    };
    let pin_a_selected = selected_endpoint
        == Some(&TestbenchEndpoint::ElementPin {
            element_id: element.id,
            pin: TestbenchPin::A,
        });
    let pin_b_selected = selected_endpoint
        == Some(&TestbenchEndpoint::ElementPin {
            element_id: element.id,
            pin: TestbenchPin::B,
        });

    let pin_a_position = testbench_pin_position(rect, element.orientation, TestbenchPin::A);
    let pin_b_position = testbench_pin_position(rect, element.orientation, TestbenchPin::B);
    painter.line_segment([pin_a_position, rect.center()], stroke);
    painter.line_segment([rect.center(), pin_b_position], stroke);
    painter.circle_filled(
        pin_a_position,
        4.0,
        if pin_a_selected {
            selected_pin_color
        } else {
            pin_color
        },
    );
    painter.circle_filled(
        pin_b_position,
        4.0,
        if pin_b_selected {
            selected_pin_color
        } else {
            pin_color
        },
    );

    match element.kind {
        GuiTestbenchElementKind::VoltageSource | GuiTestbenchElementKind::CurrentSource => {
            painter.circle_filled(rect.center(), 20.0, body_color);
            painter.circle_stroke(rect.center(), 20.0, stroke);
            draw_testbench_pin_polarity(painter, rect, element.orientation, TestbenchPin::A, "+");
            draw_testbench_pin_polarity(painter, rect, element.orientation, TestbenchPin::B, "-");
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                match element.kind {
                    GuiTestbenchElementKind::VoltageSource => "V",
                    GuiTestbenchElementKind::CurrentSource => "I",
                    GuiTestbenchElementKind::Resistor | GuiTestbenchElementKind::Capacitor => "",
                },
                egui::FontId::proportional(18.0),
                egui::Color32::WHITE,
            );
        }
        GuiTestbenchElementKind::Resistor => {
            let body = egui::Rect::from_center_size(rect.center(), egui::vec2(42.0, 18.0));
            painter.rect_filled(body, 2.0, body_color);
            painter.rect_stroke(body, 2.0, stroke, egui::StrokeKind::Inside);
        }
        GuiTestbenchElementKind::Capacitor => {
            let x = rect.center().x;
            let y = rect.center().y;
            painter.line_segment(
                [egui::pos2(x - 6.0, y - 18.0), egui::pos2(x - 6.0, y + 18.0)],
                stroke,
            );
            painter.line_segment(
                [egui::pos2(x + 6.0, y - 18.0), egui::pos2(x + 6.0, y + 18.0)],
                stroke,
            );
        }
    }

    painter.text(
        rect.center_bottom() + egui::vec2(0.0, 4.0),
        egui::Align2::CENTER_TOP,
        &element.name,
        egui::FontId::proportional(11.0),
        egui::Color32::from_gray(220),
    );
}

fn draw_testbench_pin_polarity(
    painter: &egui::Painter,
    rect: egui::Rect,
    orientation: GuiOrientation,
    pin: TestbenchPin,
    label: &str,
) {
    let pin_position = testbench_pin_position(rect, orientation, pin);
    let direction = normalized_or_zero(pin_position - rect.center());
    let text_position = pin_position - direction * 14.0;

    painter.text(
        text_position,
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(14.0),
        egui::Color32::from_gray(235),
    );
}

fn normalized_or_zero(vector: egui::Vec2) -> egui::Vec2 {
    if vector.length_sq() <= f32::EPSILON {
        egui::Vec2::ZERO
    } else {
        vector.normalized()
    }
}

fn update_pending_testbench_connection(
    pending_connection: &mut Option<TestbenchEndpoint>,
    connections: &mut Vec<GuiTestbenchConnection>,
    selected_endpoint: TestbenchEndpoint,
) {
    match pending_connection.take() {
        Some(from) if from != selected_endpoint => {
            connections.push(GuiTestbenchConnection {
                from,
                to: selected_endpoint,
            });
        }
        _ => {
            *pending_connection = Some(selected_endpoint);
        }
    }
}

fn draw_testbench_connections(
    painter: &egui::Painter,
    canvas_rect: egui::Rect,
    elements: &[GuiTestbenchElement],
    dut_macro: Option<&GuiDutMacroView>,
    dut_position: egui::Pos2,
    connections: &[GuiTestbenchConnection],
) {
    let stroke = egui::Stroke::new(1.5, egui::Color32::from_rgb(105, 190, 230));

    for connection in connections {
        let Some(from) = testbench_endpoint_position(
            canvas_rect,
            elements,
            dut_macro,
            dut_position,
            &connection.from,
        ) else {
            continue;
        };
        let Some(to) = testbench_endpoint_position(
            canvas_rect,
            elements,
            dut_macro,
            dut_position,
            &connection.to,
        ) else {
            continue;
        };

        draw_testbench_manhattan_connection(painter, from, to, stroke);
    }
}

fn testbench_endpoint_position(
    canvas_rect: egui::Rect,
    elements: &[GuiTestbenchElement],
    dut_macro: Option<&GuiDutMacroView>,
    dut_position: egui::Pos2,
    endpoint: &TestbenchEndpoint,
) -> Option<egui::Pos2> {
    match endpoint {
        TestbenchEndpoint::ElementPin { element_id, pin } => {
            let element = elements.iter().find(|element| element.id == *element_id)?;
            let center = canvas_rect.min + element.position.to_vec2();
            let rect = egui::Rect::from_center_size(center, egui::vec2(88.0, 54.0));

            Some(testbench_pin_position(rect, element.orientation, *pin))
        }
        TestbenchEndpoint::DutPort { port_name } => {
            let dut_macro = dut_macro?;
            let port = dut_macro
                .ports
                .iter()
                .find(|port| port.name == *port_name)?;
            let rect = dut_macro_rect(canvas_rect, dut_position);

            Some(dut_port_position(
                rect,
                port.symbol_side,
                port.symbol_offset,
            ))
        }
    }
}

fn testbench_pin_position(
    rect: egui::Rect,
    orientation: GuiOrientation,
    pin: TestbenchPin,
) -> egui::Pos2 {
    let position = match pin {
        TestbenchPin::A => egui::pos2(rect.left(), rect.center().y),
        TestbenchPin::B => egui::pos2(rect.right(), rect.center().y),
    };

    rotate_point_in_rect(position, rect, orientation)
}

fn draw_testbench_manhattan_connection(
    painter: &egui::Painter,
    from: egui::Pos2,
    to: egui::Pos2,
    stroke: egui::Stroke,
) {
    let escape = 18.0;
    let from_escape = from + egui::vec2(escape, 0.0);
    let to_escape = to - egui::vec2(escape, 0.0);
    let mid_x = (from_escape.x + to_escape.x) * 0.5;
    let points = [
        from,
        from_escape,
        egui::pos2(mid_x, from_escape.y),
        egui::pos2(mid_x, to_escape.y),
        to_escape,
        to,
    ];

    for segment in points.windows(2) {
        painter.line_segment([segment[0], segment[1]], stroke);
    }
}

fn show_macro_document_details(ui: &mut egui::Ui, circuit: &mut GuiCircuitDocument) {
    ui.heading("Macro");
    ui.label(format!("Name: {}", circuit.name));
    ui.horizontal(|ui| {
        ui.label("Subckt:");
        ui.text_edit_singleline(&mut circuit.subckt_name);
    });
    ui.label(format!("Instances: {}", circuit.canvas_instances.len()));
    ui.label(format!("Lab pins: {}", circuit.label_pins.len()));
    ui.label(format!("Macro ports: {}", circuit.macro_ports.len()));
    ui.label(format!("Connections: {}", circuit.connections.len()));

    ui.separator();
    ui.heading("Ports");
    if circuit.macro_ports.is_empty() {
        ui.label("Use Add port to place a connectable macro port on the canvas.");
    } else {
        for port in &circuit.macro_ports {
            ui.label(format!(
                "{} [{}] {} {:.2}",
                port.name,
                port.role.label(),
                symbol_pin_side_label(port.symbol_side),
                port.symbol_offset
            ));
        }
    }
}

fn show_validation_messages(ui: &mut egui::Ui, messages: &[String]) {
    if messages.is_empty() {
        return;
    }

    ui.separator();
    ui.heading("Validation");
    for message in messages {
        ui.colored_label(egui::Color32::from_rgb(230, 120, 90), message);
    }
}

fn validate_macro_document(circuit: &GuiCircuitDocument) -> Vec<String> {
    let mut messages = Vec::new();

    if circuit.name.trim().is_empty() {
        messages.push("Macro name is empty.".to_string());
    }
    if circuit.subckt_name.trim().is_empty() {
        messages.push("Macro subckt name is empty.".to_string());
    }
    if circuit.macro_ports.is_empty() {
        messages.push("Macro has no ports.".to_string());
    }

    let mut seen_ports = HashMap::new();
    for port in &circuit.macro_ports {
        let name = port.name.trim();
        if name.is_empty() {
            messages.push(format!("Macro port {} has an empty name.", port.id));
            continue;
        }

        if seen_ports.insert(name.to_string(), port.id).is_some() {
            messages.push(format!("Macro port name '{name}' is duplicated."));
        }

        let endpoint = CanvasEndpoint::MacroPort { port_id: port.id };
        let connected = circuit
            .connections
            .iter()
            .any(|connection| connection.from == endpoint || connection.to == endpoint);
        if !connected {
            messages.push(format!(
                "Macro port '{}' is not connected to the internal circuit.",
                name
            ));
        }
    }

    messages
}

fn macro_port_canvas_issue_ids(
    macro_ports: &[CanvasMacroPort],
    connections: &[CanvasConnection],
) -> Vec<usize> {
    let mut name_counts = HashMap::new();
    for port in macro_ports {
        let name = port.name.trim();
        if !name.is_empty() {
            *name_counts.entry(name.to_string()).or_insert(0usize) += 1;
        }
    }

    macro_ports
        .iter()
        .filter(|port| {
            let name = port.name.trim();
            let endpoint = CanvasEndpoint::MacroPort { port_id: port.id };
            let connected = connections
                .iter()
                .any(|connection| connection.from == endpoint || connection.to == endpoint);

            name.is_empty() || name_counts.get(name).copied().unwrap_or_default() > 1 || !connected
        })
        .map(|port| port.id)
        .collect()
}

fn validate_testbench_document(
    testbench: &GuiTestbenchDocument,
    macro_names: &[String],
    testbench_index: usize,
) -> Vec<String> {
    let mut messages = Vec::new();

    if testbench.name.trim().is_empty() {
        messages.push("Testbench name is empty.".to_string());
    }
    if testbench.dut_macro.trim().is_empty() {
        messages.push("Testbench has no DUT macro selected.".to_string());
    } else if !macro_names.iter().any(|name| name == &testbench.dut_macro) {
        messages.push(format!(
            "Testbench references unknown DUT macro '{}'.",
            testbench.dut_macro
        ));
    }

    match resolve_testbench_nodes(testbench, testbench_index) {
        Ok(resolved_nodes) => {
            for (element_index, element) in testbench.elements.iter().enumerate() {
                let context = format!("element {} '{}'", element_index + 1, element.name);
                if element.name.trim().is_empty() {
                    messages.push(format!("{context} has an empty name."));
                }
                if element.value.trim().is_empty() {
                    messages.push(format!("{context} has an empty value."));
                }

                for pin in [TestbenchPin::A, TestbenchPin::B] {
                    let endpoint = TestbenchEndpoint::ElementPin {
                        element_id: element.id,
                        pin,
                    };
                    let manual_node = testbench_element_pin_text(element, pin).trim();
                    if manual_node.is_empty() && !resolved_nodes.contains_key(&endpoint) {
                        messages.push(format!(
                            "{} has unresolved {}.",
                            context,
                            node_label_without_colon(element.kind, pin)
                        ));
                    }
                }
            }
        }
        Err(error) => messages.push(error),
    }

    messages
}

fn testbench_element_has_canvas_issue(
    element: &GuiTestbenchElement,
    resolved_nodes: Option<&HashMap<TestbenchEndpoint, String>>,
) -> bool {
    if element.name.trim().is_empty() || element.value.trim().is_empty() {
        return true;
    }

    let Some(resolved_nodes) = resolved_nodes else {
        return true;
    };

    [TestbenchPin::A, TestbenchPin::B].into_iter().any(|pin| {
        let endpoint = TestbenchEndpoint::ElementPin {
            element_id: element.id,
            pin,
        };
        testbench_element_pin_text(element, pin).trim().is_empty()
            && !resolved_nodes.contains_key(&endpoint)
    })
}

fn show_testbench_dut_selector(
    ui: &mut egui::Ui,
    testbench: &mut GuiTestbenchDocument,
    macro_names: &[String],
) {
    ui.horizontal(|ui| {
        ui.label("DUT macro:");
        egui::ComboBox::from_id_salt("testbench_dut_macro")
            .selected_text(display_optional_name(&testbench.dut_macro))
            .show_ui(ui, |ui| {
                for macro_name in macro_names {
                    ui.selectable_value(&mut testbench.dut_macro, macro_name.clone(), macro_name);
                }
            });
    });

    if macro_names.is_empty() {
        ui.label("Create a macro before using this testbench.");
    }

    ui.horizontal(|ui| {
        ui.label("Small-signal mode:");
        egui::ComboBox::from_id_salt("testbench_small_signal_mode")
            .selected_text(testbench.small_signal_mode.label())
            .show_ui(ui, |ui| {
                for mode in GuiSmallSignalMode::all() {
                    ui.selectable_value(&mut testbench.small_signal_mode, mode, mode.label());
                }
            });
    });
}

fn show_instance_details(
    ui: &mut egui::Ui,
    instance: &mut CanvasInstance,
    selected_endpoint: Option<&CanvasEndpoint>,
) {
    ui.heading(&instance.instance_name);
    ui.label(format!("Canvas ID: {}", instance.id));
    ui.label(format!(
        "{}: {}",
        instance.block.kind_label(),
        instance.block.name()
    ));
    ui.horizontal(|ui| {
        ui.label("Instance name:");
        ui.text_edit_singleline(&mut instance.instance_name);
    });
    ui.label(format!(
        "Position: {:.0}, {:.0}",
        instance.position.x, instance.position.y
    ));
    ui.label(format!(
        "Orientation: {} deg",
        instance.orientation.degrees()
    ));

    if let Some(CanvasEndpoint::PrimitivePin {
        instance_id,
        pin_name,
    }) = selected_endpoint.filter(|endpoint| match endpoint {
        CanvasEndpoint::PrimitivePin { instance_id, .. } => *instance_id == instance.id,
        CanvasEndpoint::LabelPin { .. } | CanvasEndpoint::MacroPort { .. } => false,
    }) {
        let _ = instance_id;
        ui.separator();
        ui.label(format!("Selected pin: {pin_name}"));
    }
}

fn show_label_pin_details(ui: &mut egui::Ui, label_pin: &mut CanvasLabelPin) {
    ui.heading("Lab pin");
    ui.label(format!("ID: {}", label_pin.id));
    ui.horizontal(|ui| {
        ui.label("Net:");
        ui.text_edit_singleline(&mut label_pin.name);
    });
    ui.label(format!(
        "Position: {:.0}, {:.0}",
        label_pin.position.x, label_pin.position.y
    ));
}

fn show_macro_port_details(ui: &mut egui::Ui, macro_port: &mut CanvasMacroPort) {
    ui.heading("Macro port");
    ui.label(format!("ID: {}", macro_port.id));
    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.text_edit_singleline(&mut macro_port.name);
    });
    ui.horizontal(|ui| {
        ui.label("Role:");
        egui::ComboBox::from_id_salt(format!("selected_macro_port_role_{}", macro_port.id))
            .selected_text(macro_port.role.label())
            .show_ui(ui, |ui| {
                for role in GuiMacroPortRole::all() {
                    ui.selectable_value(&mut macro_port.role, role, role.label());
                }
            });
    });
    ui.horizontal(|ui| {
        ui.label("Symbol side:");
        egui::ComboBox::from_id_salt(format!("selected_macro_port_side_{}", macro_port.id))
            .selected_text(symbol_pin_side_label(macro_port.symbol_side))
            .show_ui(ui, |ui| {
                for side in symbol_pin_sides() {
                    ui.selectable_value(
                        &mut macro_port.symbol_side,
                        side,
                        symbol_pin_side_label(side),
                    );
                }
            });
    });
    ui.add(
        egui::DragValue::new(&mut macro_port.symbol_offset)
            .range(0.0..=1.0)
            .speed(0.01)
            .prefix("Symbol offset "),
    );
    macro_port.symbol_offset = macro_port.symbol_offset.clamp(0.0, 1.0);
    ui.label(format!(
        "Canvas position: {:.0}, {:.0}",
        macro_port.position.x, macro_port.position.y
    ));
}

fn update_pending_connection(
    pending_connection: &mut Option<CanvasEndpoint>,
    connections: &mut Vec<CanvasConnection>,
    selected_endpoint: CanvasEndpoint,
) {
    match pending_connection.take() {
        Some(from) if from != selected_endpoint => {
            connections.push(CanvasConnection {
                from,
                to: selected_endpoint,
            });
        }
        _ => {
            *pending_connection = Some(selected_endpoint);
        }
    }
}

fn project_connection_endpoint_exists(
    endpoint: &CanvasEndpoint,
    instances: &[CanvasInstance],
    label_pins: &[CanvasLabelPin],
    macro_ports: &[CanvasMacroPort],
) -> bool {
    match endpoint {
        CanvasEndpoint::PrimitivePin { instance_id, .. } => {
            instances.iter().any(|instance| instance.id == *instance_id)
        }
        CanvasEndpoint::LabelPin { label_id } => {
            label_pins.iter().any(|label_pin| label_pin.id == *label_id)
        }
        CanvasEndpoint::MacroPort { port_id } => macro_ports
            .iter()
            .any(|macro_port| macro_port.id == *port_id),
    }
}

fn testbench_connection_endpoint_exists(
    endpoint: &TestbenchEndpoint,
    elements: &[GuiTestbenchElement],
) -> bool {
    match endpoint {
        TestbenchEndpoint::ElementPin { element_id, .. } => {
            elements.iter().any(|element| element.id == *element_id)
        }
        TestbenchEndpoint::DutPort { .. } => true,
    }
}

fn draw_canvas_connections(
    painter: &egui::Painter,
    canvas: &CanvasView,
    instances: &[CanvasInstance],
    label_pins: &[CanvasLabelPin],
    macro_ports: &[CanvasMacroPort],
    catalog: Option<&PrimitiveCatalog>,
    macro_blocks: &[GuiDutMacroView],
    connections: &[CanvasConnection],
) {
    for connection in connections {
        let Some(from) = endpoint_view(
            canvas,
            instances,
            label_pins,
            macro_ports,
            catalog,
            macro_blocks,
            &connection.from,
        ) else {
            continue;
        };
        let Some(to) = endpoint_view(
            canvas,
            instances,
            label_pins,
            macro_ports,
            catalog,
            macro_blocks,
            &connection.to,
        ) else {
            continue;
        };

        draw_manhattan_connection(painter, &from, &to);
    }
}

fn endpoint_view(
    canvas: &CanvasView,
    instances: &[CanvasInstance],
    label_pins: &[CanvasLabelPin],
    macro_ports: &[CanvasMacroPort],
    catalog: Option<&PrimitiveCatalog>,
    macro_blocks: &[GuiDutMacroView],
    endpoint: &CanvasEndpoint,
) -> Option<EndpointView> {
    match endpoint {
        CanvasEndpoint::PrimitivePin {
            instance_id,
            pin_name,
        } => {
            let catalog = catalog?;
            let instance = instances
                .iter()
                .find(|instance| instance.id == *instance_id)?;
            let rect = canvas.instance_rect(instance);

            block_pin_views(rect, instance, Some(catalog), macro_blocks)?
                .into_iter()
                .find(|pin| pin.name == *pin_name)
                .map(EndpointView::from_pin_view)
        }
        CanvasEndpoint::LabelPin { label_id } => {
            let label_pin = label_pins
                .iter()
                .find(|label_pin| label_pin.id == *label_id)?;

            Some(EndpointView {
                position: canvas.to_screen(label_pin.position),
                side: PinSide::Left,
            })
        }
        CanvasEndpoint::MacroPort { port_id } => {
            let macro_port = macro_ports
                .iter()
                .find(|macro_port| macro_port.id == *port_id)?;

            Some(EndpointView {
                position: canvas.to_screen(macro_port.position),
                side: symbol_pin_side_to_pin_side(macro_port.symbol_side),
            })
        }
    }
}

fn draw_manhattan_connection(painter: &egui::Painter, from: &EndpointView, to: &EndpointView) {
    let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(120, 210, 150));
    let from_escape = pin_escape_position(from);
    let to_escape = pin_escape_position(to);
    let mid_x = (from_escape.x + to_escape.x) * 0.5;
    let points = [
        from.position,
        from_escape,
        egui::pos2(mid_x, from_escape.y),
        egui::pos2(mid_x, to_escape.y),
        to_escape,
        to.position,
    ];

    for segment in points.windows(2) {
        painter.line_segment([segment[0], segment[1]], stroke);
    }
}

fn pin_escape_position(pin: &EndpointView) -> egui::Pos2 {
    let escape = 24.0;

    match pin.side {
        PinSide::Left => pin.position - egui::vec2(escape, 0.0),
        PinSide::Right => pin.position + egui::vec2(escape, 0.0),
        PinSide::Top => pin.position - egui::vec2(0.0, escape),
        PinSide::Bottom => pin.position + egui::vec2(0.0, escape),
    }
}

fn format_endpoint(endpoint: &CanvasEndpoint) -> String {
    match endpoint {
        CanvasEndpoint::PrimitivePin {
            instance_id,
            pin_name,
        } => format!("{instance_id}:{pin_name}"),
        CanvasEndpoint::LabelPin { label_id } => format!("lab:{label_id}"),
        CanvasEndpoint::MacroPort { port_id } => format!("port:{port_id}"),
    }
}

fn circuit_instance_id(instance_id: usize) -> String {
    format!("x{instance_id}")
}

fn default_project_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| std::env::temp_dir().join("sstadex-gui-mna"))
}

fn default_project_path() -> PathBuf {
    default_project_dir().join("gui_project.json")
}

fn default_python_gmid_path() -> String {
    "python".to_string()
}

fn legacy_python_gmid_path() -> &'static str {
    ".venv-sstadex/bin/python"
}

fn default_nmos_lut_path() -> String {
    "LUTs/ihp-sg13g2/lv_5w_nmos.npz".to_string()
}

fn default_pmos_lut_path() -> String {
    "LUTs/ihp-sg13g2/lv_5w_pmos.npz".to_string()
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(default_project_dir)
}

fn resolve_workspace_relative_path(path: &str) -> PathBuf {
    let candidate = PathBuf::from(path);
    if candidate.is_absolute() || candidate.exists() {
        return candidate;
    }

    let workspace_candidate = workspace_root().join(path);
    if workspace_candidate.exists() {
        workspace_candidate
    } else {
        candidate
    }
}

fn defaulted_project_text(value: String, default_value: String) -> String {
    if value.trim().is_empty() {
        default_value
    } else {
        value
    }
}

fn defaulted_python_gmid_path(value: String) -> String {
    if value.trim().is_empty() || value.trim() == legacy_python_gmid_path() {
        default_python_gmid_path()
    } else {
        value
    }
}

fn default_dut_position() -> egui::Pos2 {
    egui::pos2(360.0, 36.0)
}

fn imported_instance_position(index: usize) -> egui::Pos2 {
    let column = index % 3;
    let row = index / 3;
    egui::pos2(96.0 + 190.0 * column as f32, 96.0 + 130.0 * row as f32)
}

fn imported_label_pin_position(index: usize) -> egui::Pos2 {
    egui::pos2(80.0, 260.0 + 32.0 * index as f32)
}

fn imported_macro_port_position(side: SymbolPinSide, offset: f32) -> egui::Pos2 {
    let offset = offset.clamp(0.0, 1.0);
    let left = 24.0;
    let right = 660.0;
    let top = 32.0;
    let bottom = 460.0;

    match side {
        SymbolPinSide::Left => egui::pos2(left, top + (bottom - top) * offset),
        SymbolPinSide::Right => egui::pos2(right, top + (bottom - top) * offset),
        SymbolPinSide::Top => egui::pos2(left + (right - left) * offset, top),
        SymbolPinSide::Bottom => egui::pos2(left + (right - left) * offset, bottom),
    }
}

fn next_available_circuit_name(circuits: &[GuiCircuitDocument]) -> String {
    let mut index = 1;

    loop {
        let candidate = format!("macro_{index}");
        let is_available = circuits.iter().all(|circuit| circuit.name != candidate);

        if is_available {
            return candidate;
        }

        index += 1;
    }
}

fn macro_port_role(role: GuiMacroPortRole) -> MacroPortRole {
    match role {
        GuiMacroPortRole::Input => MacroPortRole::Input,
        GuiMacroPortRole::Output => MacroPortRole::Output,
        GuiMacroPortRole::Inout => MacroPortRole::Inout,
        GuiMacroPortRole::Bias => MacroPortRole::Bias,
        GuiMacroPortRole::Supply => MacroPortRole::Supply,
        GuiMacroPortRole::Ground => MacroPortRole::Ground,
    }
}

fn gui_macro_port_role(role: MacroPortRole) -> GuiMacroPortRole {
    match role {
        MacroPortRole::Input => GuiMacroPortRole::Input,
        MacroPortRole::Output => GuiMacroPortRole::Output,
        MacroPortRole::Inout => GuiMacroPortRole::Inout,
        MacroPortRole::Bias => GuiMacroPortRole::Bias,
        MacroPortRole::Supply => GuiMacroPortRole::Supply,
        MacroPortRole::Ground => GuiMacroPortRole::Ground,
    }
}

fn pin_role_from_macro_port_role(role: GuiMacroPortRole) -> PinRole {
    match role {
        GuiMacroPortRole::Input => PinRole::Input,
        GuiMacroPortRole::Output => PinRole::Output,
        GuiMacroPortRole::Inout => PinRole::Internal,
        GuiMacroPortRole::Bias => PinRole::Bias,
        GuiMacroPortRole::Supply | GuiMacroPortRole::Ground => PinRole::Supply,
    }
}

fn default_symbol_side_for_index(index: usize) -> SymbolPinSide {
    match index % 4 {
        0 => SymbolPinSide::Left,
        1 => SymbolPinSide::Right,
        2 => SymbolPinSide::Top,
        _ => SymbolPinSide::Bottom,
    }
}

fn default_symbol_offset_for_index(index: usize) -> f32 {
    let slot = (index / 4) + 1;
    (slot as f32 / (slot + 1) as f32).clamp(0.0, 1.0)
}

fn macro_small_signal_mode(mode: GuiSmallSignalMode) -> MacroSmallSignalMode {
    match mode {
        GuiSmallSignalMode::CompactWhenAvailable => MacroSmallSignalMode::CompactWhenAvailable,
        GuiSmallSignalMode::Expand => MacroSmallSignalMode::Expand,
    }
}

fn document_macro_ports(document: &GuiCircuitDocument) -> Vec<GuiMacroPortView<'_>> {
    if document.macro_ports.is_empty() {
        document
            .ports
            .iter()
            .map(GuiMacroPortView::Metadata)
            .collect()
    } else {
        document
            .macro_ports
            .iter()
            .map(GuiMacroPortView::Canvas)
            .collect()
    }
}

fn gui_macro_port_from_view(port: GuiMacroPortView<'_>) -> GuiMacroPort {
    GuiMacroPort {
        name: macro_port_name(port).to_string(),
        role: macro_port_role_value(port),
        symbol_side: macro_port_symbol_side(port),
        symbol_offset: macro_port_symbol_offset(port),
    }
}

fn macro_port_name(port: GuiMacroPortView<'_>) -> &str {
    match port {
        GuiMacroPortView::Canvas(port) => &port.name,
        GuiMacroPortView::Metadata(port) => &port.name,
    }
}

fn macro_port_role_value(port: GuiMacroPortView<'_>) -> GuiMacroPortRole {
    match port {
        GuiMacroPortView::Canvas(port) => port.role,
        GuiMacroPortView::Metadata(port) => port.role,
    }
}

fn macro_port_symbol_side(port: GuiMacroPortView<'_>) -> SymbolPinSide {
    match port {
        GuiMacroPortView::Canvas(port) => port.symbol_side,
        GuiMacroPortView::Metadata(port) => port.symbol_side,
    }
}

fn macro_port_symbol_offset(port: GuiMacroPortView<'_>) -> f32 {
    match port {
        GuiMacroPortView::Canvas(port) => port.symbol_offset,
        GuiMacroPortView::Metadata(port) => port.symbol_offset,
    }
}

fn default_symbol_side_for_role(role: GuiMacroPortRole) -> SymbolPinSide {
    match role {
        GuiMacroPortRole::Input | GuiMacroPortRole::Bias => SymbolPinSide::Left,
        GuiMacroPortRole::Output | GuiMacroPortRole::Inout => SymbolPinSide::Right,
        GuiMacroPortRole::Supply => SymbolPinSide::Top,
        GuiMacroPortRole::Ground => SymbolPinSide::Bottom,
    }
}

fn symbol_pin_sides() -> [SymbolPinSide; 4] {
    [
        SymbolPinSide::Left,
        SymbolPinSide::Right,
        SymbolPinSide::Top,
        SymbolPinSide::Bottom,
    ]
}

fn symbol_pin_side_label(side: SymbolPinSide) -> &'static str {
    match side {
        SymbolPinSide::Left => "left",
        SymbolPinSide::Right => "right",
        SymbolPinSide::Top => "top",
        SymbolPinSide::Bottom => "bottom",
    }
}

fn symbol_pin_side_to_pin_side(side: SymbolPinSide) -> PinSide {
    match side {
        SymbolPinSide::Left => PinSide::Left,
        SymbolPinSide::Right => PinSide::Right,
        SymbolPinSide::Top => PinSide::Top,
        SymbolPinSide::Bottom => PinSide::Bottom,
    }
}

fn display_optional_name(name: &str) -> &str {
    if name.trim().is_empty() {
        "(none)"
    } else {
        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libsstadex::exploration::ExplorationColumn;
    use libsstadex::primitive::build::{
        BuildExpression, PrimitiveBuildInputSpec, PrimitiveBuildSpec, SweepMode,
    };
    use libsstadex::primitive::manifest::{Pin, PrimitiveFiles, PrimitiveShape, PrimitiveUi};

    #[test]
    fn macro_workspace_switching_preserves_per_macro_candidates() {
        let mut app = SstadexApp::default();

        app.candidates.axes = vec![GuiCandidateAxis {
            name: "gain_target".to_string(),
            values: "10, 20".to_string(),
        }];
        app.add_circuit_document();
        app.candidates.axes = vec![GuiCandidateAxis {
            name: "bias_current".to_string(),
            values: "1e-6, 2e-6".to_string(),
        }];

        app.switch_circuit_document(0);

        assert_eq!(app.active_circuit, 0);
        assert_eq!(app.candidates.axes.len(), 1);
        assert_eq!(app.candidates.axes[0].name, "gain_target");

        app.switch_circuit_document(1);

        assert_eq!(app.active_circuit, 1);
        assert_eq!(app.candidates.axes.len(), 1);
        assert_eq!(app.candidates.axes[0].name, "bias_current");
    }

    #[test]
    fn inserting_library_macro_imports_project_macro_workspace() {
        let mut app = SstadexApp::default();
        app.macro_catalog = Some(
            load_macro_catalog(&workspace_root().join("analoglib/macros"))
                .expect("macro catalog should load"),
        );

        app.add_canvas_instance(GuiBlockRef::Macro {
            name: "current_source".to_string(),
        });

        assert!(
            app.circuits
                .iter()
                .any(|circuit| circuit.name == "current_source")
        );
        assert_eq!(app.macro_workspaces.len(), app.circuits.len());
        assert!(matches!(
            app.canvas_instances[0].block,
            GuiBlockRef::Macro { ref name } if name == "current_source"
        ));

        let imported = app
            .circuits
            .iter()
            .find(|circuit| circuit.name == "current_source")
            .unwrap();
        assert!(imported.small_signal.is_some());
    }

    #[test]
    fn importing_library_macro_recursively_imports_child_macros() {
        let mut app = SstadexApp::default();
        app.macro_catalog = Some(
            load_macro_catalog(&workspace_root().join("analoglib/macros"))
                .expect("macro catalog should load"),
        );

        app.add_canvas_instance(GuiBlockRef::Macro {
            name: "ota_1stage".to_string(),
        });

        assert!(
            app.circuits
                .iter()
                .any(|circuit| circuit.name == "ota_1stage")
        );
        assert!(
            app.circuits
                .iter()
                .any(|circuit| circuit.name == "current_source")
        );
        assert_eq!(app.macro_workspaces.len(), app.circuits.len());
    }

    #[test]
    fn primitive_build_input_uses_shared_net_voltage_constraint() {
        let primitive = primitive_manifest_with_build();
        let instance = CanvasInstance {
            id: 1,
            instance_name: "xcs".to_string(),
            block: GuiBlockRef::Primitive {
                name: primitive.name.clone(),
            },
            position: egui::pos2(0.0, 0.0),
            orientation: GuiOrientation::R0,
        };
        let document = GuiCircuitDocument {
            name: "macro".to_string(),
            subckt_name: "macro".to_string(),
            small_signal: None,
            ports: Vec::new(),
            canvas_instances: vec![instance.clone()],
            label_pins: vec![CanvasLabelPin {
                id: 1,
                name: "VIN".to_string(),
                position: egui::pos2(0.0, 0.0),
            }],
            macro_ports: Vec::new(),
            connections: vec![CanvasConnection {
                from: CanvasEndpoint::PrimitivePin {
                    instance_id: 1,
                    pin_name: "VIN".to_string(),
                },
                to: CanvasEndpoint::LabelPin { label_id: 1 },
            }],
            next_instance_id: 2,
            next_label_pin_id: 2,
            next_macro_port_id: 1,
        };
        let catalog = catalog_with_primitive(primitive.clone());
        let net_index = CanvasNetIndex::from_document(&document, &catalog, &[]);
        let voltage_constraints = HashMap::from([(
            "VIN".to_string(),
            PrimitiveBuildValue::Vector(vec![0.4, 0.6]),
        )]);
        let global_parameters =
            HashMap::from([("current".to_string(), PrimitiveBuildValue::Scalar(100e-6))]);

        let input = gui_primitive_build_input(
            &GuiCandidateDocument::default(),
            &instance,
            &primitive,
            &net_index,
            &voltage_constraints,
            &global_parameters,
        )
        .unwrap();

        assert_eq!(
            input.values.get("VIN"),
            Some(&PrimitiveBuildValue::Vector(vec![0.4, 0.6]))
        );
        assert_eq!(
            input.values.get("current"),
            Some(&PrimitiveBuildValue::Scalar(100e-6))
        );
    }

    #[test]
    fn primitive_build_input_prefers_instance_override_over_global_parameter() {
        let primitive = primitive_manifest_with_build();
        let instance = CanvasInstance {
            id: 7,
            instance_name: "x7".to_string(),
            block: GuiBlockRef::Primitive {
                name: primitive.name.clone(),
            },
            position: egui::pos2(0.0, 0.0),
            orientation: GuiOrientation::R0,
        };
        let document = GuiCircuitDocument {
            name: "macro".to_string(),
            subckt_name: "macro".to_string(),
            small_signal: None,
            ports: Vec::new(),
            canvas_instances: vec![instance.clone()],
            label_pins: Vec::new(),
            macro_ports: Vec::new(),
            connections: Vec::new(),
            next_instance_id: 8,
            next_label_pin_id: 1,
            next_macro_port_id: 1,
        };
        let catalog = catalog_with_primitive(primitive.clone());
        let net_index = CanvasNetIndex::from_document(&document, &catalog, &[]);
        let voltage_constraints =
            HashMap::from([("x7.VIN".to_string(), PrimitiveBuildValue::Vector(vec![0.5]))]);
        let global_parameters =
            HashMap::from([("current".to_string(), PrimitiveBuildValue::Scalar(100e-6))]);
        let candidates = GuiCandidateDocument {
            primitive_build_overrides: vec![GuiPrimitiveBuildOverride {
                instance_id: 7,
                parameters: vec![GuiBuildParameter {
                    name: "current".to_string(),
                    values: "250e-6".to_string(),
                }],
            }],
            ..GuiCandidateDocument::default()
        };

        let input = gui_primitive_build_input(
            &candidates,
            &instance,
            &primitive,
            &net_index,
            &voltage_constraints,
            &global_parameters,
        )
        .unwrap();

        assert_eq!(
            input.values.get("current"),
            Some(&PrimitiveBuildValue::Scalar(250e-6))
        );
    }

    #[test]
    fn project_macro_instance_roundtrips_block_ref() {
        let document = macro_instance_document();

        let project = GuiProjectCircuit::from_circuit_document(&document);
        let loaded = project.into_circuit_document();

        assert_eq!(loaded.canvas_instances.len(), 1);
        assert_eq!(
            loaded.canvas_instances[0].block.macro_name(),
            Some("current_source")
        );
    }

    #[test]
    fn project_testbench_roundtrips_compact_outputs() {
        let testbench = GuiTestbenchDocument {
            name: "tb_current_source".to_string(),
            dut_macro: "current_source".to_string(),
            dut_position: default_dut_position(),
            small_signal_mode: GuiSmallSignalMode::CompactWhenAvailable,
            compact_outputs: vec![GuiCompactOutputBinding {
                source_column: "bias_current".to_string(),
                compact_parameter: "isource".to_string(),
            }],
            elements: Vec::new(),
            connections: Vec::new(),
            selected_endpoint: None,
            pending_connection: None,
            extra_body: String::new(),
            next_element_id: 1,
        };

        let project = GuiProjectTestbench::from_testbench_document(&testbench);
        let loaded = project.into_testbench_document();

        assert_eq!(loaded.compact_outputs.len(), 1);
        assert_eq!(loaded.compact_outputs[0].source_column, "bias_current");
        assert_eq!(loaded.compact_outputs[0].compact_parameter, "isource");
    }

    #[test]
    fn project_spec_roundtrips_node_voltage_source() {
        let spec = GuiSpecDocument {
            name: "vout".to_string(),
            source_kind: GuiSpecSourceKind::NodeVoltage,
            testbench: "tb".to_string(),
            input: String::new(),
            output: String::new(),
            node: "VOUT".to_string(),
            min: "0".to_string(),
            max: String::new(),
            parameter_map: Vec::new(),
        };

        let project = GuiProjectSpec::from_spec_document(&spec);
        let loaded = project.into_spec_document();

        assert_eq!(loaded.source_kind, GuiSpecSourceKind::NodeVoltage);
        assert_eq!(loaded.node, "VOUT");
    }

    #[test]
    fn project_derived_column_roundtrips() {
        let column = GuiDerivedColumnDocument {
            name: "gain_db".to_string(),
            expression: "20 * log10(abs(gain))".to_string(),
        };

        let project = GuiProjectDerivedColumn::from_derived_column_document(&column);
        let loaded = project.into_derived_column_document();

        assert_eq!(loaded.name, "gain_db");
        assert_eq!(loaded.expression, "20 * log10(abs(gain))");
    }

    #[test]
    fn gui_derived_columns_convert_to_library_specs() {
        let columns = vec![GuiDerivedColumnDocument {
            name: "gain_db".to_string(),
            expression: "20 * log10(abs(gain))".to_string(),
        }];

        let converted = gui_derived_columns_to_specs(&columns).unwrap();

        assert_eq!(converted.len(), 1);
        assert_eq!(converted[0].name, "gain_db");
        assert_eq!(converted[0].expression, "20 * log10(abs(gain))");
    }

    #[test]
    fn gui_spec_converts_to_node_voltage_source() {
        let spec = GuiSpecDocument {
            name: "vout".to_string(),
            source_kind: GuiSpecSourceKind::NodeVoltage,
            testbench: "tb".to_string(),
            input: String::new(),
            output: String::new(),
            node: "VOUT".to_string(),
            min: "0".to_string(),
            max: String::new(),
            parameter_map: Vec::new(),
        };

        let converted =
            gui_spec_to_exploration_spec(&spec, &[TestbenchSpec::new("tb")], 1).unwrap();

        assert!(matches!(
            converted.source,
            SpecSource::NodeVoltage { node, .. } if node == "VOUT"
        ));
    }

    #[test]
    fn hierarchy_child_results_become_compact_candidate_set() {
        let mut app = SstadexApp::default();
        app.circuits = vec![
            macro_instance_document(),
            GuiCircuitDocument::empty("current_source"),
        ];
        let mut child_workspace = GuiMacroWorkspace::default();
        child_workspace.testbenches.push(GuiTestbenchDocument {
            name: "tb_current_source".to_string(),
            dut_macro: "current_source".to_string(),
            dut_position: default_dut_position(),
            small_signal_mode: GuiSmallSignalMode::CompactWhenAvailable,
            compact_outputs: vec![GuiCompactOutputBinding {
                source_column: "bias_current".to_string(),
                compact_parameter: "isource".to_string(),
            }],
            elements: Vec::new(),
            connections: Vec::new(),
            selected_endpoint: None,
            pending_connection: None,
            extra_body: String::new(),
            next_element_id: 1,
        });
        app.macro_workspaces = vec![GuiMacroWorkspace::default(), child_workspace];
        let mut results_by_macro = HashMap::new();
        results_by_macro.insert(
            "current_source".to_string(),
            ExplorationTable {
                columns: vec![
                    ExplorationColumn::new("bias_current", vec![1.0, 2.0]),
                    ExplorationColumn::new("xcs.width_m1", vec![3.0, 4.0]),
                    ExplorationColumn::new("xcs.length__m1", vec![0.15, 0.2]),
                    ExplorationColumn::new("gain", vec![10.0, 20.0]),
                ],
                row_count: 2,
            },
        );

        let sets = app
            .compact_candidate_sets_for_children(0, &results_by_macro)
            .unwrap();

        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0].name, "xcs_macro");
        assert_eq!(sets[0].points[0].get("isource__xcs_macro"), Some(1.0));
        assert_eq!(sets[0].points[1].get("isource__xcs_macro"), Some(2.0));
        assert_eq!(sets[0].points[0].get("xcs_macro.xcs.width_m1"), Some(3.0));
        assert_eq!(sets[0].points[1].get("xcs_macro.xcs.length__m1"), Some(0.2));
        assert_eq!(sets[0].points[0].get("xcs_macro.gain"), None);
    }

    #[test]
    fn macro_instance_exports_as_macro_block() {
        let app = SstadexApp::default();
        let document = macro_instance_document();

        let circuit = app.build_circuit_from_document(&document);

        assert_eq!(circuit.instances.len(), 1);
        assert_eq!(circuit.instances[0].macro_name(), Some("current_source"));
        assert_eq!(circuit.instances[0].primitive_name(), None);
    }

    #[test]
    fn canvas_net_index_includes_macro_instance_ports() {
        let primitive_catalog = PrimitiveCatalog::new();
        let document = macro_instance_document();
        let macro_blocks = vec![GuiDutMacroView {
            name: "current_source".to_string(),
            ports: vec![GuiMacroPort {
                name: "VOUT".to_string(),
                role: GuiMacroPortRole::Output,
                symbol_side: SymbolPinSide::Right,
                symbol_offset: 0.5,
            }],
        }];

        let net_index = CanvasNetIndex::from_document(&document, &primitive_catalog, &macro_blocks);

        assert_eq!(
            net_index.net_for_endpoint(&CanvasEndpoint::PrimitivePin {
                instance_id: 1,
                pin_name: "VOUT".to_string()
            }),
            Some("xcs_macro.VOUT")
        );
    }

    #[test]
    fn project_candidate_document_roundtrips_declarative_build_config() {
        let candidates = GuiCandidateDocument {
            net_voltage_constraints: vec![GuiNetVoltageConstraint {
                net: "VIN".to_string(),
                values: "linspace(0.4, 0.8, 3)".to_string(),
            }],
            global_build_parameters: vec![GuiBuildParameter {
                name: "current".to_string(),
                values: "100e-6".to_string(),
            }],
            primitive_build_overrides: vec![GuiPrimitiveBuildOverride {
                instance_id: 3,
                parameters: vec![GuiBuildParameter {
                    name: "current".to_string(),
                    values: "200e-6".to_string(),
                }],
            }],
            python_path: "python".to_string(),
            nmos_lut_path: "nmos.npz".to_string(),
            pmos_lut_path: "pmos.npz".to_string(),
            timing_output: false,
            ..GuiCandidateDocument::default()
        };

        let restored = GuiProjectCandidateDocument::from_candidate_document(&candidates)
            .into_candidate_document();

        assert_eq!(restored.net_voltage_constraints[0].net, "VIN");
        assert_eq!(restored.global_build_parameters[0].name, "current");
        assert_eq!(restored.primitive_build_overrides[0].instance_id, 3);
        assert_eq!(restored.python_path, "python");
        assert_eq!(restored.nmos_lut_path, "nmos.npz");
        assert_eq!(restored.pmos_lut_path, "pmos.npz");
        assert!(!restored.timing_output);
    }

    #[test]
    fn resolves_relative_gmid_paths_against_workspace_root() {
        let resolved = resolve_workspace_relative_path("libsstadex/Cargo.toml");

        assert_eq!(resolved, workspace_root().join("libsstadex/Cargo.toml"));
    }

    #[test]
    fn exploration_table_csv_includes_index_and_escapes_headers() {
        let table = ExplorationTable {
            row_count: 2,
            columns: vec![
                ExplorationColumn::new("gain", vec![1.0, 2.5]),
                ExplorationColumn::new("weird,name", vec![3.0, 4.0]),
            ],
        };

        assert_eq!(
            exploration_table_to_csv(&table),
            "index,gain,\"weird,name\"\n0,1.000000000000e0,3.000000000000e0\n1,2.500000000000e0,4.000000000000e0\n"
        );
    }

    #[test]
    fn exploration_table_summary_reports_visible_row_limit() {
        let table = ExplorationTable {
            row_count: 201,
            columns: vec![ExplorationColumn::new("gain", vec![1.0; 201])],
        };

        assert_eq!(results_table_shown_rows(&table), 200);
        assert_eq!(
            exploration_table_summary_lines(&table),
            vec![
                "rows: 201".to_string(),
                "columns: 1".to_string(),
                "showing first 200 rows".to_string(),
            ]
        );
    }

    #[test]
    fn automatic_area_column_sums_width_columns() {
        let mut table = ExplorationTable {
            row_count: 2,
            columns: vec![
                ExplorationColumn::new("width__x1__m1", vec![1.0, 2.0]),
                ExplorationColumn::new("x2.width_m1", vec![3.0, 4.0]),
                ExplorationColumn::new("gm__x1__m1", vec![5.0, 6.0]),
            ],
        };

        add_automatic_area_column(&mut table).unwrap();

        assert_eq!(table.column("area").unwrap().values, vec![4.0, 6.0]);
    }

    #[test]
    fn automatic_area_column_preserves_existing_area() {
        let mut table = ExplorationTable {
            row_count: 1,
            columns: vec![
                ExplorationColumn::new("width__x1__m1", vec![1.0]),
                ExplorationColumn::new("area", vec![9.0]),
            ],
        };

        add_automatic_area_column(&mut table).unwrap();

        assert_eq!(table.column("area").unwrap().values, vec![9.0]);
        assert_eq!(
            table
                .columns
                .iter()
                .filter(|column| column.name == "area")
                .count(),
            1
        );
    }

    fn catalog_with_primitive(primitive: PrimitiveManifest) -> PrimitiveCatalog {
        let mut catalog = PrimitiveCatalog::new();
        catalog.register(primitive);
        catalog
    }

    fn macro_instance_document() -> GuiCircuitDocument {
        GuiCircuitDocument {
            name: "parent".to_string(),
            subckt_name: "parent".to_string(),
            small_signal: None,
            ports: Vec::new(),
            canvas_instances: vec![CanvasInstance {
                id: 1,
                instance_name: "xcs_macro".to_string(),
                block: GuiBlockRef::Macro {
                    name: "current_source".to_string(),
                },
                position: egui::pos2(0.0, 0.0),
                orientation: GuiOrientation::R0,
            }],
            label_pins: Vec::new(),
            macro_ports: Vec::new(),
            connections: Vec::new(),
            next_instance_id: 2,
            next_label_pin_id: 1,
            next_macro_port_id: 1,
        }
    }

    fn primitive_manifest_with_build() -> PrimitiveManifest {
        PrimitiveManifest {
            name: "primitive".to_string(),
            version: "1.0".to_string(),
            description: None,
            subckt_name: "primitive".to_string(),
            pins: vec![Pin {
                name: "VIN".to_string(),
                role: PinRole::Input,
            }],
            files: PrimitiveFiles {
                netlist: "netlist.spice".to_string(),
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
            build: Some(PrimitiveBuildSpec {
                inputs: vec![
                    PrimitiveBuildInputSpec {
                        name: "current".to_string(),
                        kind: PrimitiveBuildInputKind::Scalar,
                        required: true,
                        source: None,
                    },
                    PrimitiveBuildInputSpec {
                        name: "VIN".to_string(),
                        kind: PrimitiveBuildInputKind::Vector,
                        required: true,
                        source: Some("port_voltage".to_string()),
                    },
                ],
                sweep_mode: SweepMode::Aligned,
                derived: Vec::new(),
                lut: Vec::new(),
                columns: vec![BuildExpression {
                    name: "gm".to_string(),
                    expr: "current".to_string(),
                }],
            }),
        }
    }
}

fn project_path_from_input(input: &str) -> Result<PathBuf, String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("project path is empty".to_string());
    }

    Ok(PathBuf::from(trimmed))
}

fn project_dialog_dir(input: &str) -> PathBuf {
    project_path_from_input(input)
        .ok()
        .and_then(|path| {
            path.parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .map(PathBuf::from)
        })
        .unwrap_or_else(default_project_dir)
}

fn project_dialog_file_name(input: &str) -> String {
    project_path_from_input(input)
        .ok()
        .and_then(|path| {
            path.file_name()
                .filter(|name| !name.is_empty())
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "gui_project.json".to_string())
}

fn next_available_instance_name(instances: &[CanvasInstance]) -> String {
    let mut index = 1;

    loop {
        let candidate = circuit_instance_id(index);
        let is_available = instances
            .iter()
            .all(|instance| exported_instance_name(instance) != candidate);

        if is_available {
            return candidate;
        }

        index += 1;
    }
}

fn exported_instance_name(instance: &CanvasInstance) -> String {
    let name = instance.instance_name.trim();

    if name.is_empty() {
        circuit_instance_id(instance.id)
    } else {
        name.to_string()
    }
}

fn canvas_net_name(
    endpoints: &[CanvasEndpoint],
    label_pins: &[CanvasLabelPin],
    macro_ports: &[CanvasMacroPort],
) -> Option<String> {
    endpoints
        .iter()
        .filter_map(|endpoint| match endpoint {
            CanvasEndpoint::LabelPin { label_id } => label_pins
                .iter()
                .find(|label_pin| label_pin.id == *label_id)
                .map(|label_pin| label_pin.name.trim().to_string()),
            CanvasEndpoint::MacroPort { port_id } => macro_ports
                .iter()
                .find(|macro_port| macro_port.id == *port_id)
                .map(|macro_port| macro_port.name.trim().to_string()),
            CanvasEndpoint::PrimitivePin { .. } => None,
        })
        .find(|name| !name.is_empty())
}

fn connected_endpoint_groups(connections: &[CanvasConnection]) -> Vec<Vec<CanvasEndpoint>> {
    let mut adjacency: HashMap<CanvasEndpoint, Vec<CanvasEndpoint>> = HashMap::new();

    for connection in connections {
        adjacency
            .entry(connection.from.clone())
            .or_default()
            .push(connection.to.clone());
        adjacency
            .entry(connection.to.clone())
            .or_default()
            .push(connection.from.clone());
    }

    let mut groups = Vec::new();
    let mut visited: HashMap<CanvasEndpoint, bool> = HashMap::new();

    for pin in adjacency.keys() {
        if visited.contains_key(pin) {
            continue;
        }

        let mut group = Vec::new();
        let mut stack = vec![pin.clone()];

        while let Some(current) = stack.pop() {
            if visited.insert(current.clone(), true).is_some() {
                continue;
            }

            group.push(current.clone());

            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors {
                    stack.push(neighbor.clone());
                }
            }
        }

        group.sort_by(|left, right| endpoint_sort_key(left).cmp(&endpoint_sort_key(right)));
        groups.push(group);
    }

    groups
}

fn endpoint_sort_key(endpoint: &CanvasEndpoint) -> String {
    match endpoint {
        CanvasEndpoint::PrimitivePin {
            instance_id,
            pin_name,
        } => format!("p:{instance_id}:{pin_name}"),
        CanvasEndpoint::LabelPin { label_id } => format!("l:{label_id}"),
        CanvasEndpoint::MacroPort { port_id } => format!("m:{port_id}"),
    }
}

fn format_mna_output(output: &CircuitMnaOutput, circuit_path: &std::path::Path) -> String {
    let mut lines = Vec::new();

    lines.push("MNA completed".to_string());
    lines.push(format!(
        "Generated circuit JSON: {}",
        circuit_path.display()
    ));
    lines.push(format!("SPICE: {}", output.spice_path));
    lines.push(format!("CIR: {}", output.cir_path));
    lines.push(String::new());
    lines.push("Variables:".to_string());

    for variable in &output.variables {
        lines.push(format!(
            "  {} -> {} ({})",
            variable.variable, variable.node_name, variable.node_number
        ));
    }

    lines.push(String::new());
    lines.push("Equations:".to_string());

    for equation in &output.equations {
        lines.push(format!("  {}", equation.text));
    }

    lines.join("\n")
}

fn format_testbench_mna_output(
    output: &CircuitMnaOutput,
    small_signal_netlist: &str,
    macro_dir: &std::path::Path,
    testbench_path: &std::path::Path,
    mode: GuiSmallSignalMode,
) -> String {
    let mut lines = Vec::new();

    lines.push("Testbench MNA completed".to_string());
    lines.push(format!("Small-signal mode: {}", mode.label()));
    lines.push(format!("Macro dir: {}", macro_dir.display()));
    lines.push(format!("Testbenches JSON: {}", testbench_path.display()));
    lines.push(format!("SPICE: {}", output.spice_path));
    lines.push(format!("CIR: {}", output.cir_path));
    lines.push(String::new());
    lines.push("Generated SPICE netlist:".to_string());
    lines.push(small_signal_netlist.trim_end().to_string());
    lines.push(String::new());
    lines.push("Variables:".to_string());

    for variable in &output.variables {
        lines.push(format!(
            "  {} -> {} ({})",
            variable.variable, variable.node_name, variable.node_number
        ));
    }

    lines.push(String::new());
    lines.push("Equations:".to_string());

    for equation in &output.equations {
        lines.push(format!("  {}", equation.text));
    }

    lines.join("\n")
}

fn format_equations_output(output: &CircuitMnaOutput) -> String {
    let mut lines = Vec::new();

    lines.push("Variables:".to_string());
    for variable in &output.variables {
        lines.push(format!(
            "  {} -> {} ({})",
            variable.variable, variable.node_name, variable.node_number
        ));
    }

    lines.push(String::new());
    lines.push("Equations:".to_string());
    for equation in &output.equations {
        lines.push(format!("  {}", equation.text));
    }

    lines.join("\n")
}

fn format_artifacts_output(
    output: &CircuitMnaOutput,
    macro_dir: &std::path::Path,
    testbench_path: &std::path::Path,
    mode: GuiSmallSignalMode,
) -> String {
    [
        format!("Small-signal mode: {}", mode.label()),
        format!("Macro dir: {}", macro_dir.display()),
        format!("Testbenches JSON: {}", testbench_path.display()),
        format!("SPICE: {}", output.spice_path),
        format!("CIR: {}", output.cir_path),
    ]
    .join("\n")
}

fn format_prepared_specs_output(prepared_specs: &[PreparedSpec]) -> String {
    if prepared_specs.is_empty() {
        return "No prepared specs".to_string();
    }

    let mut output = String::new();

    for (index, spec) in prepared_specs.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }

        let _ = writeln!(output, "Spec {}", index + 1);
        let _ = writeln!(output, "name: {}", spec.name);
        let _ = writeln!(
            output,
            "condition: {}",
            format_range_condition(spec.condition)
        );
        let _ = writeln!(output, "output: {:?}", spec.output);

        if spec.parameter_map.is_empty() {
            let _ = writeln!(output, "parameters: none");
        } else {
            let _ = writeln!(output, "parameters:");
            for parameter in &spec.parameter_map {
                let _ = writeln!(output, "  {} = {}", parameter.name, parameter.value);
            }
        }

        match &spec.source {
            PreparedSpecSource::CandidateExpression { expression } => {
                let _ = writeln!(output, "source: candidate expression");
                let _ = writeln!(output, "expression:");
                let _ = writeln!(output, "{expression}");
            }
            PreparedSpecSource::TransferFunction { expression } => {
                let _ = writeln!(output, "source: transfer function");
                let _ = writeln!(output, "expression:");
                let _ = writeln!(output, "{expression}");
            }
            PreparedSpecSource::Composed => {
                let _ = writeln!(output, "source: composed");
            }
        }
    }

    output
}

fn format_candidates_output(candidates: &[CandidatePoint]) -> String {
    if candidates.is_empty() {
        return "No candidates".to_string();
    }

    let columns = CandidatePoint::to_columns(candidates);
    if columns.is_empty() {
        return format!(
            "Generated {} candidate point(s) with no columns",
            candidates.len()
        );
    }

    let max_rows = 200;
    let shown_rows = candidates.len().min(max_rows);
    let mut output = String::new();

    let _ = writeln!(output, "candidate points: {}", candidates.len());
    if candidates.len() > shown_rows {
        let _ = writeln!(output, "showing first {shown_rows} rows");
    }
    output.push('\n');

    let _ = write!(output, "index");
    for column in &columns {
        let _ = write!(output, "\t{}", column.name);
    }
    output.push('\n');

    for row in 0..shown_rows {
        let _ = write!(output, "{row}");
        for column in &columns {
            if let Some(value) = column.values.get(row) {
                let _ = write!(output, "\t{value:.6e}");
            } else {
                output.push('\t');
            }
        }
        output.push('\n');
    }

    output
}

fn format_exploration_table_output(table: &ExplorationTable) -> String {
    if table.columns.is_empty() {
        return format!("rows: {}\ncolumns: 0", table.row_count);
    }

    let shown_rows = results_table_shown_rows(table);
    let mut output = String::new();

    let _ = writeln!(output, "rows: {}", table.row_count);
    let _ = writeln!(output, "columns: {}", table.columns.len());
    if table.row_count > shown_rows {
        let _ = writeln!(output, "showing first {shown_rows} rows");
    }
    output.push('\n');

    let _ = write!(output, "index");
    for column in &table.columns {
        let _ = write!(output, "\t{}", column.name);
    }
    output.push('\n');

    for row in 0..shown_rows {
        let _ = write!(output, "{row}");
        for column in &table.columns {
            if let Some(value) = column.values.get(row) {
                let _ = write!(output, "\t{value:.6e}");
            } else {
                output.push('\t');
            }
        }
        output.push('\n');
    }

    output
}

fn exploration_table_to_csv(table: &ExplorationTable) -> String {
    let mut output = String::new();

    output.push_str("index");
    for column in &table.columns {
        output.push(',');
        output.push_str(&csv_escape(&column.name));
    }
    output.push('\n');

    for row in 0..table.row_count {
        let _ = write!(output, "{row}");
        for column in &table.columns {
            output.push(',');
            if let Some(value) = column.values.get(row) {
                let _ = write!(output, "{value:.12e}");
            }
        }
        output.push('\n');
    }

    output
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn format_range_condition(condition: RangeCondition) -> String {
    match (condition.min, condition.max) {
        (Some(min), Some(max)) => format!("{min} < abs(value) < {max}"),
        (Some(min), None) => format!("abs(value) > {min}"),
        (None, Some(max)) => format!("abs(value) < {max}"),
        (None, None) => "unbounded".to_string(),
    }
}

fn format_circuit_summary(circuit: &Circuit) -> String {
    let mut lines = Vec::new();

    lines.push(format!("instances: {}", circuit.instances.len()));
    for instance in &circuit.instances {
        let block = instance
            .primitive_name()
            .map(|name| format!("primitive:{name}"))
            .or_else(|| instance.macro_name().map(|name| format!("macro:{name}")))
            .unwrap_or_else(|| "unknown".to_string());
        lines.push(format!("  {}: {}", instance.id, block));
    }

    lines.push(format!("connections: {}", circuit.connections.len()));
    for connection in &circuit.connections {
        lines.push(format!(
            "  {}.{} = {}",
            connection.from.instance, connection.from.pin, connection.net
        ));
    }

    lines.join("\n")
}

fn draw_canvas_instance(
    painter: &egui::Painter,
    rect: egui::Rect,
    instance: &CanvasInstance,
    _selected_endpoint: Option<&CanvasEndpoint>,
    selected: bool,
) {
    let stroke_color = if selected {
        egui::Color32::from_rgb(220, 180, 80)
    } else {
        egui::Color32::from_rgb(130, 150, 170)
    };
    let stroke_width = if selected { 2.0 } else { 1.0 };

    painter.rect_filled(rect, 4.0, egui::Color32::from_rgb(45, 49, 56));
    painter.rect_stroke(
        rect,
        4.0,
        egui::Stroke::new(stroke_width, stroke_color),
        egui::StrokeKind::Inside,
    );

    painter.text(
        rect.center_top() + egui::vec2(0.0, 16.0),
        egui::Align2::CENTER_TOP,
        exported_instance_name(instance),
        egui::FontId::proportional(15.0),
        egui::Color32::WHITE,
    );

    painter.text(
        rect.center_bottom() - egui::vec2(0.0, 22.0),
        egui::Align2::CENTER_BOTTOM,
        instance.block.name(),
        egui::FontId::proportional(12.0),
        egui::Color32::from_gray(180),
    );
}

fn draw_label_pin(
    painter: &egui::Painter,
    position: egui::Pos2,
    label_pin: &CanvasLabelPin,
    selected: bool,
) {
    let color = if selected {
        egui::Color32::from_rgb(240, 210, 90)
    } else {
        egui::Color32::from_rgb(120, 210, 150)
    };

    painter.circle_filled(position, if selected { 5.5 } else { 4.5 }, color);
    painter.text(
        position + egui::vec2(8.0, 0.0),
        egui::Align2::LEFT_CENTER,
        &label_pin.name,
        egui::FontId::proportional(12.0),
        egui::Color32::from_gray(230),
    );
}

fn draw_macro_port_pin(
    painter: &egui::Painter,
    position: egui::Pos2,
    macro_port: &CanvasMacroPort,
    selected: bool,
    has_issue: bool,
) {
    let color = if selected {
        egui::Color32::from_rgb(240, 210, 90)
    } else {
        egui::Color32::from_rgb(235, 195, 105)
    };
    let rect = egui::Rect::from_center_size(position, egui::vec2(10.0, 10.0));

    painter.rect_filled(rect, 2.0, color);
    if has_issue {
        painter.rect_stroke(
            rect.expand(3.0),
            2.0,
            egui::Stroke::new(1.5, egui::Color32::from_rgb(220, 85, 75)),
            egui::StrokeKind::Inside,
        );
    }
    painter.text(
        position + egui::vec2(9.0, -6.0),
        egui::Align2::LEFT_CENTER,
        &macro_port.name,
        egui::FontId::proportional(12.0),
        egui::Color32::from_gray(235),
    );
    painter.text(
        position + egui::vec2(9.0, 7.0),
        egui::Align2::LEFT_CENTER,
        "port",
        egui::FontId::proportional(9.0),
        egui::Color32::from_gray(170),
    );
}

fn draw_instance_pins(
    painter: &egui::Painter,
    rect: egui::Rect,
    instance_id: usize,
    primitive: &PrimitiveManifest,
    orientation: GuiOrientation,
    selected_endpoint: Option<&CanvasEndpoint>,
) {
    let pin_views = pin_views(rect, primitive, orientation);
    draw_instance_pin_views(painter, instance_id, &pin_views, selected_endpoint);
}

fn draw_instance_pin_views(
    painter: &egui::Painter,
    instance_id: usize,
    pin_views: &[PinView],
    selected_endpoint: Option<&CanvasEndpoint>,
) {
    for pin_view in pin_views {
        let selected = selected_endpoint.is_some_and(|endpoint| {
            *endpoint
                == CanvasEndpoint::PrimitivePin {
                    instance_id,
                    pin_name: pin_view.name.clone(),
                }
        });
        let pin_color = if selected {
            egui::Color32::from_rgb(240, 210, 90)
        } else if pin_view.role == PinRole::Internal {
            egui::Color32::from_gray(120)
        } else {
            egui::Color32::from_rgb(120, 190, 220)
        };

        painter.circle_filled(
            pin_view.position,
            if selected { 5.5 } else { 4.0 },
            pin_color,
        );
        painter.text(
            pin_view.label_position,
            pin_view.align,
            &pin_view.name,
            egui::FontId::proportional(10.0),
            egui::Color32::from_gray(210),
        );
    }
}

fn block_pin_views(
    rect: egui::Rect,
    instance: &CanvasInstance,
    catalog: Option<&PrimitiveCatalog>,
    macro_blocks: &[GuiDutMacroView],
) -> Option<Vec<PinView>> {
    match &instance.block {
        GuiBlockRef::Primitive { name } => {
            let primitive = catalog?.get(name)?;
            Some(pin_views(rect, primitive, instance.orientation))
        }
        GuiBlockRef::Macro { name } => {
            let macro_block = macro_blocks
                .iter()
                .find(|macro_block| &macro_block.name == name)?;
            Some(macro_pin_views(rect, macro_block, instance.orientation))
        }
    }
}

struct EndpointView {
    position: egui::Pos2,
    side: PinSide,
}

impl EndpointView {
    fn from_pin_view(pin_view: PinView) -> Self {
        Self {
            position: pin_view.position,
            side: pin_view.side,
        }
    }
}

struct PinView {
    name: String,
    role: PinRole,
    side: PinSide,
    position: egui::Pos2,
    label_position: egui::Pos2,
    align: egui::Align2,
}

fn pin_views(
    rect: egui::Rect,
    primitive: &PrimitiveManifest,
    orientation: GuiOrientation,
) -> Vec<PinView> {
    if let Some(symbol) = &primitive.ui.symbol {
        let mut views = Vec::new();

        for symbol_pin in &symbol.pins {
            let Some(pin) = primitive
                .pins
                .iter()
                .find(|pin| pin.name == symbol_pin.name)
            else {
                continue;
            };
            let side = match symbol_pin.side {
                SymbolPinSide::Left => PinSide::Left,
                SymbolPinSide::Right => PinSide::Right,
                SymbolPinSide::Top => PinSide::Top,
                SymbolPinSide::Bottom => PinSide::Bottom,
            };

            views.push(rotate_pin_view(
                pin_view_at(
                    rect,
                    side,
                    symbol_pin.offset.clamp(0.0, 1.0),
                    &pin.name,
                    &pin.role,
                ),
                rect,
                orientation,
            ));
        }

        return views;
    }

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut supplies = Vec::new();
    let mut bottom_pins = Vec::new();

    for pin in &primitive.pins {
        match pin.role {
            PinRole::Input => inputs.push(pin),
            PinRole::Output => outputs.push(pin),
            PinRole::Supply => supplies.push(pin),
            PinRole::Bias | PinRole::Internal => bottom_pins.push(pin),
        }
    }

    let mut views = Vec::new();
    append_pin_group(&mut views, rect, PinSide::Left, &inputs);
    append_pin_group(&mut views, rect, PinSide::Right, &outputs);
    append_pin_group(&mut views, rect, PinSide::Top, &supplies);
    append_pin_group(&mut views, rect, PinSide::Bottom, &bottom_pins);
    views
        .into_iter()
        .map(|view| rotate_pin_view(view, rect, orientation))
        .collect()
}

fn macro_pin_views(
    rect: egui::Rect,
    macro_block: &GuiDutMacroView,
    orientation: GuiOrientation,
) -> Vec<PinView> {
    macro_block
        .ports
        .iter()
        .map(|port| {
            let side = symbol_pin_side_to_pin_side(port.symbol_side);
            rotate_pin_view(
                pin_view_at(
                    rect,
                    side,
                    port.symbol_offset.clamp(0.0, 1.0),
                    &port.name,
                    &pin_role_from_macro_port_role(port.role),
                ),
                rect,
                orientation,
            )
        })
        .collect()
}

#[derive(Clone, Copy)]
enum PinSide {
    Left,
    Right,
    Top,
    Bottom,
}

fn append_pin_group(
    views: &mut Vec<PinView>,
    rect: egui::Rect,
    side: PinSide,
    pins: &[&libsstadex::primitive::manifest::Pin],
) {
    if pins.is_empty() {
        return;
    }

    for (index, pin) in pins.iter().enumerate() {
        let t = (index + 1) as f32 / (pins.len() + 1) as f32;
        views.push(pin_view_at(rect, side, t, &pin.name, &pin.role));
    }
}

fn pin_view_at(
    rect: egui::Rect,
    side: PinSide,
    offset: f32,
    name: &str,
    role: &PinRole,
) -> PinView {
    let (position, label_position, align) = match side {
        PinSide::Left => {
            let y = egui::lerp(rect.top() + 14.0..=rect.bottom() - 14.0, offset);
            (
                egui::pos2(rect.left(), y),
                egui::pos2(rect.left() + 8.0, y),
                egui::Align2::LEFT_CENTER,
            )
        }
        PinSide::Right => {
            let y = egui::lerp(rect.top() + 14.0..=rect.bottom() - 14.0, offset);
            (
                egui::pos2(rect.right(), y),
                egui::pos2(rect.right() - 8.0, y),
                egui::Align2::RIGHT_CENTER,
            )
        }
        PinSide::Top => {
            let x = egui::lerp(rect.left() + 18.0..=rect.right() - 18.0, offset);
            (
                egui::pos2(x, rect.top()),
                egui::pos2(x, rect.top() + 8.0),
                egui::Align2::CENTER_TOP,
            )
        }
        PinSide::Bottom => {
            let x = egui::lerp(rect.left() + 18.0..=rect.right() - 18.0, offset);
            (
                egui::pos2(x, rect.bottom()),
                egui::pos2(x, rect.bottom() - 8.0),
                egui::Align2::CENTER_BOTTOM,
            )
        }
    };

    PinView {
        name: name.to_string(),
        role: role.clone(),
        side,
        position,
        label_position,
        align,
    }
}

fn rotate_pin_view(view: PinView, rect: egui::Rect, orientation: GuiOrientation) -> PinView {
    if orientation == GuiOrientation::R0 {
        return view;
    }

    let side = rotate_pin_side(view.side, orientation);
    let position = rotate_point_in_rect(view.position, rect, orientation);

    PinView {
        position,
        label_position: pin_label_position(position, side),
        align: pin_label_align(side),
        side,
        ..view
    }
}

fn rotate_pin_side(side: PinSide, orientation: GuiOrientation) -> PinSide {
    let mut side = side;
    let turns = match orientation {
        GuiOrientation::R0 => 0,
        GuiOrientation::R90 => 1,
        GuiOrientation::R180 => 2,
        GuiOrientation::R270 => 3,
    };

    for _ in 0..turns {
        side = match side {
            PinSide::Left => PinSide::Top,
            PinSide::Top => PinSide::Right,
            PinSide::Right => PinSide::Bottom,
            PinSide::Bottom => PinSide::Left,
        };
    }

    side
}

fn rotate_point_in_rect(
    position: egui::Pos2,
    rect: egui::Rect,
    orientation: GuiOrientation,
) -> egui::Pos2 {
    let center = rect.center();
    let delta = position - center;

    match orientation {
        GuiOrientation::R0 => position,
        GuiOrientation::R90 => center + egui::vec2(-delta.y, delta.x),
        GuiOrientation::R180 => center + egui::vec2(-delta.x, -delta.y),
        GuiOrientation::R270 => center + egui::vec2(delta.y, -delta.x),
    }
}

fn pin_label_position(position: egui::Pos2, side: PinSide) -> egui::Pos2 {
    match side {
        PinSide::Left => position + egui::vec2(8.0, 0.0),
        PinSide::Right => position - egui::vec2(8.0, 0.0),
        PinSide::Top => position + egui::vec2(0.0, 8.0),
        PinSide::Bottom => position - egui::vec2(0.0, 8.0),
    }
}

fn pin_label_align(side: PinSide) -> egui::Align2 {
    match side {
        PinSide::Left => egui::Align2::LEFT_CENTER,
        PinSide::Right => egui::Align2::RIGHT_CENTER,
        PinSide::Top => egui::Align2::CENTER_TOP,
        PinSide::Bottom => egui::Align2::CENTER_BOTTOM,
    }
}

fn pin_role_label(role: &PinRole) -> &'static str {
    match role {
        PinRole::Input => "input",
        PinRole::Output => "output",
        PinRole::Bias => "bias",
        PinRole::Supply => "supply",
        PinRole::Internal => "internal",
    }
}
