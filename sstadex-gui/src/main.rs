use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui;
use libsstadex::analysis::{
    CircuitMnaOutput, analyze_circuit_mna, analyze_macro_testbench_mna_with_mode,
};
use libsstadex::catalog::{PrimitiveCatalog, load_primitive_catalog};
use libsstadex::circuit::{Circuit, Connection, Instance, PinRef, save_circuit};
use libsstadex::exploration::{TestbenchElement, TestbenchSpec, save_testbenches};
use libsstadex::macro_model::{
    MacroCatalog, MacroMetadata, MacroModel, MacroPort, MacroPortRole, MacroSmallSignalMode,
    MacroSymbol, MacroSymbolPin, save_macro_model,
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
    load_error: Option<String>,
    show_insert_primitive_window: bool,
    insert_primitive_selection: Option<String>,
    canvas_instances: Vec<CanvasInstance>,
    label_pins: Vec<CanvasLabelPin>,
    macro_ports: Vec<CanvasMacroPort>,
    selected_instance_id: Option<usize>,
    selected_endpoint: Option<CanvasEndpoint>,
    pending_connection: Option<CanvasEndpoint>,
    connections: Vec<CanvasConnection>,
    circuits: Vec<GuiCircuitDocument>,
    active_circuit: usize,
    active_document: ActiveDocument,
    renaming_circuit: Option<usize>,
    bottom_view: BottomView,
    testbenches: Vec<GuiTestbenchDocument>,
    selected_testbench: Option<usize>,
    renaming_testbench: Option<usize>,
    project_path: String,
    output_log: String,
    next_instance_id: usize,
    next_label_pin_id: usize,
    next_macro_port_id: usize,
}

#[derive(Clone)]
struct CanvasInstance {
    id: usize,
    instance_name: String,
    primitive_name: String,
    position: egui::Pos2,
    orientation: GuiOrientation,
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
    Testbenches,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActiveDocument {
    Circuit,
    Testbench,
}

#[derive(Clone)]
struct GuiTestbenchDocument {
    name: String,
    dut_macro: String,
    dut_position: egui::Pos2,
    small_signal_mode: GuiSmallSignalMode,
    elements: Vec<GuiTestbenchElement>,
    connections: Vec<GuiTestbenchConnection>,
    selected_endpoint: Option<TestbenchEndpoint>,
    pending_connection: Option<TestbenchEndpoint>,
    extra_body: String,
    next_element_id: usize,
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
    selected_testbench: Option<usize>,
    testbenches: Vec<GuiProjectTestbench>,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectCircuit {
    name: String,
    #[serde(default)]
    subckt_name: String,
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
    elements: Vec<GuiProjectTestbenchElement>,
    #[serde(default)]
    connections: Vec<GuiProjectTestbenchConnection>,
    extra_body: String,
    next_element_id: usize,
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
    primitive: String,
    position: GuiProjectPosition,
    #[serde(default)]
    orientation: GuiProjectOrientation,
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
        let (catalog, load_error) = match load_primitive_catalog(&primitives_dir) {
            Ok(catalog) => (Some(catalog), None),
            Err(error) => (None, Some(format!("{error:?}"))),
        };

        Self {
            catalog,
            load_error,
            show_insert_primitive_window: false,
            insert_primitive_selection: None,
            canvas_instances: Vec::new(),
            label_pins: Vec::new(),
            macro_ports: Vec::new(),
            selected_instance_id: None,
            selected_endpoint: None,
            pending_connection: None,
            connections: Vec::new(),
            circuits: vec![GuiCircuitDocument::empty("macro_1")],
            active_circuit: 0,
            active_document: ActiveDocument::Circuit,
            renaming_circuit: None,
            bottom_view: BottomView::Logs,
            testbenches: Vec::new(),
            selected_testbench: None,
            renaming_testbench: None,
            project_path: default_project_path().display().to_string(),
            output_log: "Logs, netlists, and MNA results will appear here".to_string(),
            next_instance_id: 1,
            next_label_pin_id: 1,
            next_macro_port_id: 1,
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
                    .add_enabled(is_circuit_active, egui::Button::new("Insert primitive"))
                    .clicked()
                {
                    self.show_insert_primitive_window = true;
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

        self.show_insert_primitive_window(ctx);

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
                            ui.label(if testbench.extra_body.trim().is_empty() {
                                "Extra body: empty"
                            } else {
                                "Extra body: present"
                            });
                        } else {
                            ui.label("No testbench selected");
                        }
                    }
                }
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(240.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.bottom_view, BottomView::Logs, "Logs");
                    ui.selectable_value(
                        &mut self.bottom_view,
                        BottomView::Testbenches,
                        "Testbenches",
                    );
                });
                ui.separator();

                match self.bottom_view {
                    BottomView::Logs => self.show_logs_ui(ui),
                    BottomView::Testbenches => self.show_testbenches_ui(ui),
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| match self.active_document {
            ActiveDocument::Circuit => self.show_circuit_document_ui(ui),
            ActiveDocument::Testbench => self.show_testbench_document_ui(ui),
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
            let primitive = self
                .catalog
                .as_ref()
                .and_then(|catalog| catalog.get(&instance.primitive_name));

            if let Some(primitive) = primitive {
                for pin_view in pin_views(rect, primitive, instance.orientation) {
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
                primitive,
                self.selected_endpoint.as_ref(),
                selected,
            );
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
            draw_macro_port_pin(&painter, position, macro_port, selected);
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

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Testbenches");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("+").clicked() {
                    self.add_testbench();
                }
            });
        });

        for index in 0..self.testbenches.len() {
            self.show_testbench_browser_item(ui, index);
        }
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

    fn add_circuit_document(&mut self) {
        self.save_active_circuit_document();

        let name = next_available_circuit_name(&self.circuits);
        self.circuits.push(GuiCircuitDocument::empty(name));
        self.active_circuit = self.circuits.len() - 1;
        self.active_document = ActiveDocument::Circuit;
        self.load_active_circuit_document();
    }

    fn switch_circuit_document(&mut self, index: usize) {
        if index == self.active_circuit || index >= self.circuits.len() {
            return;
        }

        self.save_active_circuit_document();
        self.active_circuit = index;
        self.load_active_circuit_document();
    }

    fn delete_circuit_document(&mut self, index: usize) {
        if self.circuits.len() <= 1 || index >= self.circuits.len() {
            return;
        }

        self.save_active_circuit_document();
        self.circuits.remove(index);

        if self.active_circuit == index {
            self.active_circuit = index.saturating_sub(1).min(self.circuits.len() - 1);
            self.load_active_circuit_document();
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

    fn show_insert_primitive_window(&mut self, ctx: &egui::Context) {
        if !self.show_insert_primitive_window {
            return;
        }

        let mut is_open = self.show_insert_primitive_window;
        let mut primitive_to_insert = None;
        let mut close_requested = false;

        egui::Window::new("Insert primitive")
            .open(&mut is_open)
            .fixed_size(egui::vec2(440.0, 300.0))
            .resizable(false)
            .show(ctx, |ui| {
                if let Some(error) = &self.load_error {
                    ui.label(format!("Failed to load catalog: {error}"));
                    return;
                }

                let Some(catalog) = &self.catalog else {
                    ui.label("No primitives loaded yet");
                    return;
                };

                if self.insert_primitive_selection.is_none() {
                    self.insert_primitive_selection = catalog
                        .list()
                        .first()
                        .map(|primitive| primitive.name.clone());
                }

                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(180.0);
                        ui.heading("Primitives");
                        ui.separator();

                        egui::ScrollArea::vertical()
                            .id_salt("insert_primitive_catalog_scroll")
                            .max_height(190.0)
                            .show(ui, |ui| {
                                for primitive in catalog.list() {
                                    ui.selectable_value(
                                        &mut self.insert_primitive_selection,
                                        Some(primitive.name.clone()),
                                        &primitive.name,
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
                            .id_salt("insert_primitive_preview_scroll")
                            .max_height(210.0)
                            .show(ui, |ui| {
                                if let Some(primitive) = self
                                    .insert_primitive_selection
                                    .as_ref()
                                    .and_then(|name| catalog.get(name))
                                {
                                    show_primitive_details(ui, primitive);
                                    ui.separator();
                                    draw_primitive_preview(ui, primitive);
                                } else {
                                    ui.label("Select a primitive");
                                }
                            });
                    });
                });

                ui.separator();
                ui.horizontal(|ui| {
                    let can_insert = self.insert_primitive_selection.is_some();
                    if ui
                        .add_enabled(can_insert, egui::Button::new("Insert"))
                        .clicked()
                    {
                        primitive_to_insert = self.insert_primitive_selection.clone();
                    }
                    if ui.button("Cancel").clicked() {
                        close_requested = true;
                    }
                });
            });

        if let Some(primitive_name) = primitive_to_insert {
            self.add_canvas_instance(&primitive_name);
            is_open = false;
        } else if close_requested {
            is_open = false;
        }

        self.show_insert_primitive_window = is_open;
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

    fn add_canvas_instance(&mut self, primitive_name: &str) {
        let offset = 28.0 * self.canvas_instances.len() as f32;
        let id = self.next_instance_id;
        let instance_name = next_available_instance_name(&self.canvas_instances);

        self.canvas_instances.push(CanvasInstance {
            id,
            instance_name,
            primitive_name: primitive_name.to_string(),
            position: egui::pos2(40.0 + offset, 40.0 + offset),
            orientation: GuiOrientation::R0,
        });

        self.selected_instance_id = Some(id);
        self.selected_endpoint = None;
        self.pending_connection = None;
        self.next_instance_id += 1;
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
        }
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

        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let macro_dir = output_dir.join("gui_macros");
        let testbench_path = output_dir.join("gui_testbenches.json");

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

        format!(
            "Saved flow inputs\n\nMacros: {}\nTestbenches: {}\nMacro dir: {}\nTestbenches: {}",
            macro_models.len(),
            testbenches.len(),
            macro_dir.display(),
            testbench_path.display()
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
        let mode = macro_small_signal_mode(gui_testbench.small_signal_mode);

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

        let mut macro_catalog = MacroCatalog::new();
        for macro_model in macro_models {
            macro_catalog.register(macro_model);
        }

        match analyze_macro_testbench_mna_with_mode(
            testbench,
            catalog,
            &macro_catalog,
            &mna_dir,
            false,
            mode,
        ) {
            Ok(analysis) => format_testbench_mna_output(
                &CircuitMnaOutput::from_analysis(&analysis),
                &analysis.small_signal_netlist,
                &macro_dir,
                &testbench_path,
                gui_testbench.small_signal_mode,
            ),
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

        if !(4..=5).contains(&project.version) {
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
            version: 5,
            active_circuit: self.active_circuit,
            circuits: self
                .circuits
                .iter()
                .map(GuiProjectCircuit::from_circuit_document)
                .collect(),
            selected_testbench: self.selected_testbench,
            testbenches: self
                .testbenches
                .iter()
                .map(GuiProjectTestbench::from_testbench_document)
                .collect(),
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
        self.active_circuit = project.active_circuit.min(self.circuits.len() - 1);
        self.testbenches = project
            .testbenches
            .into_iter()
            .map(GuiProjectTestbench::into_testbench_document)
            .collect();
        self.selected_testbench = project
            .selected_testbench
            .filter(|index| *index < self.testbenches.len());
        self.ensure_testbench_dut_macros();
        self.load_active_circuit_document();

        self.selected_instance_id = None;
        self.selected_endpoint = None;
        self.pending_connection = None;
    }

    fn ensure_testbench_dut_macros(&mut self) {
        let default_dut = self.default_dut_macro_name();
        let macro_names = self.macro_names();
        for testbench in &mut self.testbenches {
            if testbench.dut_macro.trim().is_empty()
                || !macro_names.iter().any(|name| name == &testbench.dut_macro)
            {
                testbench.dut_macro = default_dut.clone();
            }
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
            circuit.add_instance(Instance::new(
                exported_instance_name(instance),
                instance.primitive_name.clone(),
            ));
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

impl GuiProjectCircuit {
    fn from_circuit_document(circuit: &GuiCircuitDocument) -> Self {
        Self {
            name: circuit.name.clone(),
            subckt_name: circuit.subckt_name.clone(),
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
                    primitive: instance.primitive_name.clone(),
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
            .map(|instance| CanvasInstance {
                id: instance.id,
                instance_name: instance.name,
                primitive_name: instance.primitive,
                position: instance.position.to_pos(),
                orientation: instance.orientation.into_gui_orientation(),
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

impl GuiProjectTestbench {
    fn from_testbench_document(testbench: &GuiTestbenchDocument) -> Self {
        Self {
            name: testbench.name.clone(),
            dut_macro: testbench.dut_macro.clone(),
            small_signal_mode: GuiProjectSmallSignalMode::from_gui_mode(
                testbench.small_signal_mode,
            ),
            dut_position: Some(GuiProjectPosition::from_pos(testbench.dut_position)),
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
            elements,
            connections,
            selected_endpoint: None,
            pending_connection: None,
            extra_body: self.extra_body,
            next_element_id: self.next_element_id,
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

        specs.push(spec);
    }

    Ok(specs)
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
) {
    let stroke = egui::Stroke::new(1.5, egui::Color32::from_rgb(130, 150, 170));
    let body_color = egui::Color32::from_rgb(45, 49, 56);
    let pin_color = egui::Color32::from_rgb(120, 210, 150);
    let selected_pin_color = egui::Color32::from_rgb(245, 200, 80);
    let element_selected = selected_endpoint.is_some_and(|endpoint| match endpoint {
        TestbenchEndpoint::ElementPin { element_id, .. } => *element_id == element.id,
        TestbenchEndpoint::DutPort { .. } => false,
    });
    let stroke = if element_selected {
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
    ui.label(format!("Primitive: {}", instance.primitive_name));
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
    connections: &[CanvasConnection],
) {
    for connection in connections {
        let Some(from) = endpoint_view(
            canvas,
            instances,
            label_pins,
            macro_ports,
            catalog,
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
            let primitive = catalog.get(&instance.primitive_name)?;
            let rect = canvas.instance_rect(instance);

            pin_views(rect, primitive, instance.orientation)
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

fn default_dut_position() -> egui::Pos2 {
    egui::pos2(360.0, 36.0)
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
    primitive: Option<&PrimitiveManifest>,
    selected_endpoint: Option<&CanvasEndpoint>,
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
        &instance.primitive_name,
        egui::FontId::proportional(12.0),
        egui::Color32::from_gray(180),
    );

    if let Some(primitive) = primitive {
        draw_instance_pins(
            painter,
            rect,
            instance.id,
            primitive,
            instance.orientation,
            selected_endpoint,
        );
    }
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
) {
    let color = if selected {
        egui::Color32::from_rgb(240, 210, 90)
    } else {
        egui::Color32::from_rgb(235, 195, 105)
    };
    let rect = egui::Rect::from_center_size(position, egui::vec2(10.0, 10.0));

    painter.rect_filled(rect, 2.0, color);
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
    for pin_view in pin_views(rect, primitive, orientation) {
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
