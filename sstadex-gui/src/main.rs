use std::collections::HashMap;
use std::path::PathBuf;

use eframe::egui;
use libsstadex::analysis::{CircuitMnaOutput, analyze_circuit_mna};
use libsstadex::catalog::{load_primitive_catalog, PrimitiveCatalog};
use libsstadex::circuit::{Circuit, Connection, Instance, PinRef, save_circuit};
use libsstadex::primitive::manifest::{PinRole, PrimitiveManifest, SymbolPinSide};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "SSTADEx",
        options,
        Box::new(|_cc| Ok(Box::new(SstadexApp::default()))),
    )
}

struct SstadexApp {
    primitives_dir: PathBuf,
    catalog: Option<PrimitiveCatalog>,
    selected_primitive: Option<String>,
    load_error: Option<String>,
    canvas_instances: Vec<CanvasInstance>,
    label_pins: Vec<CanvasLabelPin>,
    selected_instance_id: Option<usize>,
    selected_endpoint: Option<CanvasEndpoint>,
    pending_connection: Option<CanvasEndpoint>,
    connections: Vec<CanvasConnection>,
    output_log: String,
    next_instance_id: usize,
    next_label_pin_id: usize,
}

struct CanvasInstance {
    id: usize,
    primitive_name: String,
    position: egui::Pos2,
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum CanvasEndpoint {
    PrimitivePin { instance_id: usize, pin_name: String },
    LabelPin { label_id: usize },
}

struct CanvasLabelPin {
    id: usize,
    name: String,
    position: egui::Pos2,
}

struct CanvasConnection {
    from: CanvasEndpoint,
    to: CanvasEndpoint,
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
            primitives_dir,
            catalog,
            selected_primitive: None,
            load_error,
            canvas_instances: Vec::new(),
            label_pins: Vec::new(),
            selected_instance_id: None,
            selected_endpoint: None,
            pending_connection: None,
            connections: Vec::new(),
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
            ui.horizontal(|ui| {
                let _ = ui.button("Open circuit");
                let _ = ui.button("Save circuit");
                if ui.button("Add lab pin").clicked() {
                    self.add_label_pin();
                }
                if ui.button("Run MNA").clicked() {
                    self.output_log = self.run_mna_from_canvas();
                }
            });
        });

        egui::SidePanel::left("primitive_catalog")
            .resizable(true)
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Primitive catalog");
                ui.label(self.primitives_dir.display().to_string());
                ui.separator();

                if let Some(error) = &self.load_error {
                    ui.label(format!("Failed to load catalog: {error}"));
                    return;
                }

                if let Some(catalog) = &self.catalog {
                    for primitive in catalog.list() {
                        let response = ui.selectable_value(
                            &mut self.selected_primitive,
                            Some(primitive.name.clone()),
                            &primitive.name,
                        );

                        if response.clicked() {
                            self.selected_instance_id = None;
                            self.selected_endpoint = None;
                            self.pending_connection = None;
                        }
                    }
                } else {
                    ui.label("No primitives loaded yet");
                }
            });

        egui::SidePanel::right("details")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("Details");
                ui.separator();

                if let Some(label_pin) = self.selected_label_pin_mut() {
                    show_label_pin_details(ui, label_pin);
                } else if let Some(instance) = self.selected_instance() {
                    show_instance_details(ui, instance, self.selected_endpoint.as_ref());
                } else if let Some(primitive) = self.selected_primitive().cloned() {
                    show_primitive_details(ui, &primitive);

                    ui.separator();
                    if ui.button("Add to canvas").clicked() {
                        self.add_canvas_instance(&primitive.name);
                    }
                } else {
                    ui.label("Nothing selected");
                }
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(180.0)
            .show(ctx, |ui| {
                ui.heading("Output");
                ui.separator();
                ui.label(format!(
                    "Selected primitive: {}",
                    self.selected_primitive.as_deref().unwrap_or("none")
                ));
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
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Canvas");
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
    fn selected_primitive(&self) -> Option<&PrimitiveManifest> {
        let catalog = self.catalog.as_ref()?;
        let name = self.selected_primitive.as_ref()?;

        catalog.get(name)
    }

    fn selected_instance(&self) -> Option<&CanvasInstance> {
        let id = self.selected_instance_id?;

        self.canvas_instances
            .iter()
            .find(|instance| instance.id == id)
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

        self.canvas_instances.push(CanvasInstance {
            id,
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

    fn build_circuit_from_canvas(&self) -> Circuit {
        let mut circuit = Circuit::new("gui_canvas");

        for instance in &self.canvas_instances {
            circuit.add_instance(Instance::new(
                circuit_instance_id(instance.id),
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
                    circuit.connect(Connection::new(
                        PinRef::new(circuit_instance_id(instance_id), pin_name),
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

fn show_primitive_details(ui: &mut egui::Ui, primitive: &PrimitiveManifest) {
    ui.heading(&primitive.name);
    ui.label(format!("Subckt: {}", primitive.subckt_name));
    ui.label(format!("Version: {}", primitive.version));
    ui.label(format!("Netlist: {}", primitive.files.netlist));

    if let Some(description) = &primitive.description {
        ui.separator();
        ui.label(description);
    }

    ui.separator();
    ui.heading("Pins");

    for pin in &primitive.pins {
        ui.horizontal(|ui| {
            ui.label(&pin.name);
            ui.label(pin_role_label(&pin.role));
        });
    }
}

fn show_instance_details(
    ui: &mut egui::Ui,
    instance: &CanvasInstance,
    selected_endpoint: Option<&CanvasEndpoint>,
) {
    ui.heading(format!("{}_{}", instance.primitive_name, instance.id));
    ui.label(format!("Primitive: {}", instance.primitive_name));
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

        group.sort_by(|left, right| {
            endpoint_sort_key(left).cmp(&endpoint_sort_key(right))
        });
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
        format!("{}_{}", instance.primitive_name, instance.id),
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

        painter.circle_filled(pin_view.position, if selected { 5.5 } else { 4.0 }, pin_color);
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
