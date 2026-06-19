use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui;
use libsstadex::analysis::{analyze_circuit_mna, CircuitMnaOutput};
use libsstadex::catalog::{load_primitive_catalog, PrimitiveCatalog};
use libsstadex::circuit::{save_circuit, Circuit, Connection, Instance, PinRef};
use libsstadex::exploration::{save_testbenches, TestbenchElement, TestbenchSpec};
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
    selected_instance_id: Option<usize>,
    selected_endpoint: Option<CanvasEndpoint>,
    pending_connection: Option<CanvasEndpoint>,
    connections: Vec<CanvasConnection>,
    circuits: Vec<GuiCircuitDocument>,
    active_circuit: usize,
    bottom_view: BottomView,
    testbenches: Vec<GuiTestbench>,
    selected_testbench: Option<usize>,
    project_path: String,
    output_log: String,
    next_instance_id: usize,
    next_label_pin_id: usize,
}

#[derive(Clone)]
struct CanvasInstance {
    id: usize,
    instance_name: String,
    primitive_name: String,
    position: egui::Pos2,
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
}

#[derive(Clone)]
struct CanvasLabelPin {
    id: usize,
    name: String,
    position: egui::Pos2,
}

#[derive(Clone)]
struct CanvasConnection {
    from: CanvasEndpoint,
    to: CanvasEndpoint,
}

struct GuiCircuitDocument {
    name: String,
    canvas_instances: Vec<CanvasInstance>,
    label_pins: Vec<CanvasLabelPin>,
    connections: Vec<CanvasConnection>,
    next_instance_id: usize,
    next_label_pin_id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BottomView {
    Logs,
    Testbenches,
}

struct GuiTestbench {
    name: String,
    elements: Vec<GuiTestbenchElement>,
    extra_body: String,
}

struct GuiTestbenchElement {
    kind: GuiTestbenchElementKind,
    name: String,
    nplus: String,
    nminus: String,
    value: String,
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
}

#[derive(Serialize, Deserialize)]
struct GuiProjectCircuit {
    name: String,
    instances: Vec<GuiProjectInstance>,
    label_pins: Vec<GuiProjectLabelPin>,
    connections: Vec<GuiProjectConnection>,
    next_instance_id: usize,
    next_label_pin_id: usize,
}

#[derive(Serialize, Deserialize)]
struct GuiProjectInstance {
    id: usize,
    name: String,
    primitive: String,
    position: GuiProjectPosition,
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
            selected_instance_id: None,
            selected_endpoint: None,
            pending_connection: None,
            connections: Vec::new(),
            circuits: vec![GuiCircuitDocument::empty("gui_canvas")],
            active_circuit: 0,
            bottom_view: BottomView::Logs,
            testbenches: Vec::new(),
            selected_testbench: None,
            project_path: default_project_path().display().to_string(),
            output_log: "Logs, netlists, and MNA results will appear here".to_string(),
            next_instance_id: 1,
            next_label_pin_id: 1,
        }
    }
}

impl eframe::App for SstadexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|input| input.key_pressed(egui::Key::Delete)) {
            self.delete_selected_instance();
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
                if ui.button("Open circuit").clicked() {
                    self.output_log = self.open_gui_project();
                    self.bottom_view = BottomView::Logs;
                }
                if ui.button("Save circuit").clicked() {
                    self.output_log = self.save_gui_project();
                    self.bottom_view = BottomView::Logs;
                }
                if ui.button("Add lab pin").clicked() {
                    self.add_label_pin();
                }
                if ui.button("Insert primitive").clicked() {
                    self.show_insert_primitive_window = true;
                }
                if ui.button("Run MNA").clicked() {
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

                if let Some(label_pin) = self.selected_label_pin_mut() {
                    show_label_pin_details(ui, label_pin);
                } else {
                    let selected_endpoint = self.selected_endpoint.clone();

                    if let Some(instance) = self.selected_instance_mut() {
                        show_instance_details(ui, instance, selected_endpoint.as_ref());
                    } else {
                        ui.label("Nothing selected");
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

        egui::CentralPanel::default().show(ctx, |ui| {
            let active_circuit_name = self
                .circuits
                .get(self.active_circuit)
                .map(|circuit| circuit.name.as_str())
                .unwrap_or("gui_canvas");
            ui.heading(format!("Canvas - {active_circuit_name}"));
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
                    for pin_view in pin_views(rect, primitive) {
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
        });
    }
}

impl SstadexApp {
    fn show_project_browser_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Project");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Circuits");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("+").clicked() {
                    self.add_circuit_document();
                }
            });
        });

        for index in 0..self.circuits.len() {
            let selected = self.active_circuit == index;
            let name = self.circuits[index].name.clone();

            if ui.selectable_label(selected, name).clicked() {
                self.switch_circuit_document(index);
            }
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

        for (index, testbench) in self.testbenches.iter().enumerate() {
            let label = if testbench.name.trim().is_empty() {
                "(unnamed)"
            } else {
                testbench.name.as_str()
            };

            if ui
                .selectable_label(self.selected_testbench == Some(index), label)
                .clicked()
            {
                self.selected_testbench = Some(index);
                self.bottom_view = BottomView::Testbenches;
            }
        }
    }

    fn add_circuit_document(&mut self) {
        self.save_active_circuit_document();

        let name = next_available_circuit_name(&self.circuits);
        self.circuits.push(GuiCircuitDocument::empty(name));
        self.active_circuit = self.circuits.len() - 1;
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

    fn save_active_circuit_document(&mut self) {
        let Some(circuit) = self.circuits.get_mut(self.active_circuit) else {
            return;
        };

        circuit.canvas_instances = self.canvas_instances.clone();
        circuit.label_pins = self.label_pins.clone();
        circuit.connections = self.connections.clone();
        circuit.next_instance_id = self.next_instance_id;
        circuit.next_label_pin_id = self.next_label_pin_id;
    }

    fn load_active_circuit_document(&mut self) {
        let Some(circuit) = self.circuits.get_mut(self.active_circuit) else {
            return;
        };

        self.canvas_instances = circuit.canvas_instances.clone();
        self.label_pins = circuit.label_pins.clone();
        self.connections = circuit.connections.clone();
        self.next_instance_id = circuit.next_instance_id;
        self.next_label_pin_id = circuit.next_label_pin_id;
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

    fn add_canvas_instance(&mut self, primitive_name: &str) {
        let offset = 28.0 * self.canvas_instances.len() as f32;
        let id = self.next_instance_id;
        let instance_name = next_available_instance_name(&self.canvas_instances);

        self.canvas_instances.push(CanvasInstance {
            id,
            instance_name,
            primitive_name: primitive_name.to_string(),
            position: egui::pos2(40.0 + offset, 40.0 + offset),
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

    fn add_testbench(&mut self) {
        let index = self.testbenches.len() + 1;

        self.testbenches.push(GuiTestbench {
            name: format!("tb_{index}"),
            elements: Vec::new(),
            extra_body: String::new(),
        });
        self.selected_testbench = Some(self.testbenches.len() - 1);
        self.bottom_view = BottomView::Testbenches;
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

    fn run_mna_from_canvas(&self) -> String {
        let Some(catalog) = &self.catalog else {
            return "Cannot run MNA: primitive catalog is not loaded".to_string();
        };

        if self.canvas_instances.is_empty() {
            return "Cannot run MNA: the canvas has no instances".to_string();
        }

        let circuit = self.build_circuit_from_canvas();
        let output_dir = std::env::temp_dir().join("sstadex-gui-mna");
        let circuit_path = output_dir.join("gui_canvas.json");

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
            Ok(analysis) => format_mna_output(&CircuitMnaOutput::from_analysis(&analysis), &circuit_path),
            Err(error) => format!(
                "MNA failed for circuit '{}'\n\nCircuit JSON: {}\n\n{error:?}\n\nGenerated circuit summary:\n{}",
                circuit.name,
                circuit_path.display(),
                format_circuit_summary(&circuit)
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
            Err(error) => return format!("Cannot save circuit: {error}"),
        };
        let output_dir = project_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(default_project_dir);
        let circuit_path = output_dir.join("gui_canvas.json");

        if let Err(error) = std::fs::create_dir_all(&output_dir) {
            return format!(
                "Cannot save circuit: failed to create output directory '{}'\n\n{error}",
                output_dir.display()
            );
        }

        let project = self.gui_project();
        let project_content = match serde_json::to_string_pretty(&project) {
            Ok(content) => content,
            Err(error) => {
                return format!("Cannot save circuit: failed to serialize GUI project\n\n{error}");
            }
        };

        if let Err(error) = std::fs::write(&project_path, project_content) {
            return format!(
                "Cannot save circuit: failed to write GUI project '{}'\n\n{error}",
                project_path.display()
            );
        }

        let circuit = self.build_circuit_from_canvas();
        if let Err(error) = save_circuit(&circuit_path, &circuit) {
            return format!(
                "Saved GUI project but failed to export circuit JSON '{}'\n\n{error:?}",
                circuit_path.display()
            );
        }

        format!(
            "Saved circuit project\n\nProject: {}\nCircuit JSON: {}\nInstances: {}\nConnections: {}",
            project_path.display(),
            circuit_path.display(),
            self.canvas_instances.len(),
            self.connections.len()
        )
    }

    fn open_gui_project(&mut self) -> String {
        let project_path = match project_path_from_input(&self.project_path) {
            Ok(path) => path,
            Err(error) => return format!("Cannot open circuit: {error}"),
        };
        let content = match std::fs::read_to_string(&project_path) {
            Ok(content) => content,
            Err(error) => {
                return format!(
                    "Cannot open circuit: failed to read GUI project '{}'\n\n{error}",
                    project_path.display()
                );
            }
        };

        let project: GuiProject = match serde_json::from_str(&content) {
            Ok(project) => project,
            Err(error) => {
                return format!(
                    "Cannot open circuit: invalid GUI project '{}'\n\n{error}",
                    project_path.display()
                );
            }
        };

        if project.version != 2 {
            return format!(
                "Cannot open circuit: unsupported GUI project version {}",
                project.version
            );
        }

        self.apply_gui_project(project);

        format!(
            "Opened circuit project\n\nProject: {}\nInstances: {}\nLab pins: {}\nConnections: {}",
            project_path.display(),
            self.canvas_instances.len(),
            self.label_pins.len(),
            self.connections.len()
        )
    }

    fn gui_project(&self) -> GuiProject {
        GuiProject {
            version: 2,
            active_circuit: self.active_circuit,
            circuits: self
                .circuits
                .iter()
                .map(GuiProjectCircuit::from_circuit_document)
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
            circuits.push(GuiCircuitDocument::empty("gui_canvas"));
        }

        self.circuits = circuits;
        self.active_circuit = project.active_circuit.min(self.circuits.len() - 1);
        self.load_active_circuit_document();

        self.selected_instance_id = None;
        self.selected_endpoint = None;
        self.pending_connection = None;
    }

    fn build_circuit_from_canvas(&self) -> Circuit {
        let circuit_name = self
            .circuits
            .get(self.active_circuit)
            .map(|circuit| circuit.name.as_str())
            .unwrap_or("gui_canvas");
        let mut circuit = Circuit::new(circuit_name);

        for instance in &self.canvas_instances {
            circuit.add_instance(Instance::new(
                exported_instance_name(instance),
                instance.primitive_name.clone(),
            ));
        }

        for (net_index, endpoints) in connected_endpoint_groups(&self.connections)
            .into_iter()
            .enumerate()
        {
            let net = self
                .label_net_name(&endpoints)
                .unwrap_or_else(|| format!("N{}", net_index + 1));

            for endpoint in endpoints {
                if let CanvasEndpoint::PrimitivePin {
                    instance_id,
                    pin_name,
                } = endpoint
                {
                    let instance_name = self
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

    fn label_net_name(&self, endpoints: &[CanvasEndpoint]) -> Option<String> {
        endpoints
            .iter()
            .filter_map(|endpoint| match endpoint {
                CanvasEndpoint::LabelPin { label_id } => self
                    .label_pins
                    .iter()
                    .find(|label_pin| label_pin.id == *label_id)
                    .map(|label_pin| label_pin.name.trim().to_string()),
                CanvasEndpoint::PrimitivePin { .. } => None,
            })
            .find(|name| !name.is_empty())
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

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Testbenches");
                for (index, testbench) in self.testbenches.iter().enumerate() {
                    let label = if testbench.name.trim().is_empty() {
                        "(unnamed)"
                    } else {
                        testbench.name.as_str()
                    };
                    ui.selectable_value(&mut self.selected_testbench, Some(index), label);
                }
            });

            ui.separator();

            let Some(selected_index) = self.selected_testbench else {
                ui.label("Select a testbench");
                return;
            };
            let Some(testbench) = self.testbenches.get_mut(selected_index) else {
                self.selected_testbench = None;
                return;
            };

            egui::ScrollArea::vertical().show(ui, |ui| {
                show_testbench_editor(ui, selected_index, testbench);
            });
        });
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

        let testbenches = match gui_testbenches_to_specs(&self.testbenches) {
            Ok(testbenches) => testbenches,
            Err(error) => return format!("Cannot save testbenches: {error}"),
        };

        match save_testbenches(&output_path, &testbenches) {
            Ok(()) => format!(
                "Saved {} testbench(es)\n\nPath: {}",
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
}

impl CanvasEndpoint {
    fn references_instance(&self, instance_id: usize) -> bool {
        match self {
            CanvasEndpoint::PrimitivePin {
                instance_id: endpoint_instance_id,
                ..
            } => *endpoint_instance_id == instance_id,
            CanvasEndpoint::LabelPin { .. } => false,
        }
    }
}

impl GuiCircuitDocument {
    fn empty(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            canvas_instances: Vec::new(),
            label_pins: Vec::new(),
            connections: Vec::new(),
            next_instance_id: 1,
            next_label_pin_id: 1,
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
    fn new(kind: GuiTestbenchElementKind, index: usize) -> Self {
        Self {
            kind,
            name: format!("{}{}", kind.default_name_prefix(), index),
            nplus: String::new(),
            nminus: "0".to_string(),
            value: String::new(),
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

impl GuiProjectCircuit {
    fn from_circuit_document(circuit: &GuiCircuitDocument) -> Self {
        Self {
            name: circuit.name.clone(),
            instances: circuit
                .canvas_instances
                .iter()
                .map(|instance| GuiProjectInstance {
                    id: instance.id,
                    name: exported_instance_name(instance),
                    primitive: instance.primitive_name.clone(),
                    position: GuiProjectPosition::from_pos(instance.position),
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
            connections: circuit
                .connections
                .iter()
                .map(GuiProjectConnection::from_canvas_connection)
                .collect(),
            next_instance_id: circuit.next_instance_id,
            next_label_pin_id: circuit.next_label_pin_id,
        }
    }

    fn into_circuit_document(self) -> GuiCircuitDocument {
        let canvas_instances = self
            .instances
            .into_iter()
            .map(|instance| CanvasInstance {
                id: instance.id,
                instance_name: instance.name,
                primitive_name: instance.primitive,
                position: instance.position.to_pos(),
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
        let connections = self
            .connections
            .into_iter()
            .map(GuiProjectConnection::into_canvas_connection)
            .filter(|connection| {
                project_connection_endpoint_exists(&connection.from, &canvas_instances, &label_pins)
                    && project_connection_endpoint_exists(
                        &connection.to,
                        &canvas_instances,
                        &label_pins,
                    )
            })
            .collect();

        GuiCircuitDocument {
            name: self.name,
            canvas_instances,
            label_pins,
            connections,
            next_instance_id: self.next_instance_id,
            next_label_pin_id: self.next_label_pin_id,
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
        }
    }

    fn into_canvas_endpoint(self) -> CanvasEndpoint {
        match self {
            Self::PrimitivePin { instance_id, pin } => CanvasEndpoint::PrimitivePin {
                instance_id,
                pin_name: pin,
            },
            Self::LabelPin { label_id } => CanvasEndpoint::LabelPin { label_id },
        }
    }
}

fn show_testbench_editor(ui: &mut egui::Ui, testbench_index: usize, testbench: &mut GuiTestbench) {
    ui.heading("Testbench");
    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.text_edit_singleline(&mut testbench.name);
    });

    ui.separator();
    ui.horizontal(|ui| {
        ui.heading("Elements");
        if ui.button("Add element").clicked() {
            let index = testbench.elements.len() + 1;
            testbench.elements.push(GuiTestbenchElement::new(
                GuiTestbenchElementKind::VoltageSource,
                index,
            ));
        }
    });

    let mut remove_element = None;
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
                ui.label(node_a_label(element.kind));
                ui.text_edit_singleline(&mut element.nplus);
                ui.label(node_b_label(element.kind));
                ui.text_edit_singleline(&mut element.nminus);
                ui.label("Value:");
                ui.text_edit_singleline(&mut element.value);
            });
        });
    }

    if let Some(element_index) = remove_element {
        testbench.elements.remove(element_index);
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

fn gui_testbenches_to_specs(testbenches: &[GuiTestbench]) -> Result<Vec<TestbenchSpec>, String> {
    let mut specs = Vec::with_capacity(testbenches.len());

    for (index, testbench) in testbenches.iter().enumerate() {
        let name = required_text(&testbench.name, &format!("testbench {}", index + 1), "name")?;
        let mut spec = TestbenchSpec::new(name);

        for (element_index, element) in testbench.elements.iter().enumerate() {
            spec = spec.with_element(gui_testbench_element_to_spec(
                element,
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
    testbench_index: usize,
    element_index: usize,
) -> Result<TestbenchElement, String> {
    let context = format!("testbench {testbench_index}, element {element_index}");
    let name = required_text(&element.name, &context, "name")?;
    let nplus = required_text(&element.nplus, &context, node_a_label(element.kind))?;
    let nminus = required_text(&element.nminus, &context, node_b_label(element.kind))?;
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

    draw_instance_pins(&painter, symbol_rect, 0, primitive, None);
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

    if let Some(CanvasEndpoint::PrimitivePin {
        instance_id,
        pin_name,
    }) = selected_endpoint.filter(|endpoint| match endpoint {
        CanvasEndpoint::PrimitivePin { instance_id, .. } => *instance_id == instance.id,
        CanvasEndpoint::LabelPin { .. } => false,
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
) -> bool {
    match endpoint {
        CanvasEndpoint::PrimitivePin { instance_id, .. } => {
            instances.iter().any(|instance| instance.id == *instance_id)
        }
        CanvasEndpoint::LabelPin { label_id } => {
            label_pins.iter().any(|label_pin| label_pin.id == *label_id)
        }
    }
}

fn draw_canvas_connections(
    painter: &egui::Painter,
    canvas: &CanvasView,
    instances: &[CanvasInstance],
    label_pins: &[CanvasLabelPin],
    catalog: Option<&PrimitiveCatalog>,
    connections: &[CanvasConnection],
) {
    for connection in connections {
        let Some(from) = endpoint_view(canvas, instances, label_pins, catalog, &connection.from)
        else {
            continue;
        };
        let Some(to) = endpoint_view(canvas, instances, label_pins, catalog, &connection.to) else {
            continue;
        };

        draw_manhattan_connection(painter, &from, &to);
    }
}

fn endpoint_view(
    canvas: &CanvasView,
    instances: &[CanvasInstance],
    label_pins: &[CanvasLabelPin],
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

            pin_views(rect, primitive)
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

fn next_available_circuit_name(circuits: &[GuiCircuitDocument]) -> String {
    let mut index = 1;

    loop {
        let candidate = format!("circuit_{index}");
        let is_available = circuits.iter().all(|circuit| circuit.name != candidate);

        if is_available {
            return candidate;
        }

        index += 1;
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
    }
}

fn format_mna_output(output: &CircuitMnaOutput, circuit_path: &std::path::Path) -> String {
    let mut lines = Vec::new();

    lines.push("MNA completed".to_string());
    lines.push(format!("Circuit JSON: {}", circuit_path.display()));
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

fn format_circuit_summary(circuit: &Circuit) -> String {
    let mut lines = Vec::new();

    lines.push(format!("instances: {}", circuit.instances.len()));
    for instance in &circuit.instances {
        lines.push(format!("  {}: {}", instance.id, instance.primitive));
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
        draw_instance_pins(painter, rect, instance.id, primitive, selected_endpoint);
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

fn draw_instance_pins(
    painter: &egui::Painter,
    rect: egui::Rect,
    instance_id: usize,
    primitive: &PrimitiveManifest,
    selected_endpoint: Option<&CanvasEndpoint>,
) {
    for pin_view in pin_views(rect, primitive) {
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

fn pin_views(rect: egui::Rect, primitive: &PrimitiveManifest) -> Vec<PinView> {
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

            views.push(pin_view_at(
                rect,
                side,
                symbol_pin.offset.clamp(0.0, 1.0),
                &pin.name,
                &pin.role,
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

fn pin_role_label(role: &PinRole) -> &'static str {
    match role {
        PinRole::Input => "input",
        PinRole::Output => "output",
        PinRole::Bias => "bias",
        PinRole::Supply => "supply",
        PinRole::Internal => "internal",
    }
}
