use std::path::PathBuf;

use eframe::egui;
use libsstadex::catalog::{load_primitive_catalog, PrimitiveCatalog};
use libsstadex::primitive::manifest::{PinRole, PrimitiveManifest};

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
    selected_instance_id: Option<usize>,
    selected_pin: Option<SelectedPin>,
    pending_connection: Option<SelectedPin>,
    connections: Vec<CanvasConnection>,
    next_instance_id: usize,
}

struct CanvasInstance {
    id: usize,
    primitive_name: String,
    position: egui::Pos2,
}

#[derive(Clone, PartialEq, Eq)]
struct SelectedPin {
    instance_id: usize,
    pin_name: String,
}

struct CanvasConnection {
    from: SelectedPin,
    to: SelectedPin,
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
            selected_instance_id: None,
            selected_pin: None,
            pending_connection: None,
            connections: Vec::new(),
            next_instance_id: 1,
        }
    }
}

impl eframe::App for SstadexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let _ = ui.button("Open circuit");
                let _ = ui.button("Save circuit");
                let _ = ui.button("Run MNA");
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
                            self.selected_pin = None;
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

                if let Some(instance) = self.selected_instance() {
                    show_instance_details(ui, instance, self.selected_pin.as_ref());
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
                    "Selected pin: {}",
                    self.selected_pin
                        .as_ref()
                        .map(|pin| pin.pin_name.as_str())
                        .unwrap_or("none")
                ));
                ui.label(format!(
                    "Pending connection: {}",
                    self.pending_connection
                        .as_ref()
                        .map(format_selected_pin)
                        .unwrap_or_else(|| "none".to_string())
                ));
                ui.label(format!("Connections: {}", self.connections.len()));
                ui.label("Logs, netlists, and MNA results will appear here");
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
                            let selected_pin = SelectedPin {
                                instance_id: instance.id,
                                pin_name: pin_view.name.clone(),
                            };

                            self.selected_instance_id = Some(instance.id);
                            self.selected_pin = Some(selected_pin.clone());
                            update_pending_connection(
                                &mut self.pending_connection,
                                &mut self.connections,
                                selected_pin,
                            );
                        }
                    }
                }

                draw_canvas_instance(
                    &painter,
                    rect,
                    instance,
                    primitive,
                    self.selected_pin.as_ref(),
                    selected,
                );
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

    fn add_canvas_instance(&mut self, primitive_name: &str) {
        let offset = 28.0 * self.canvas_instances.len() as f32;
        let id = self.next_instance_id;

        self.canvas_instances.push(CanvasInstance {
            id,
            primitive_name: primitive_name.to_string(),
            position: egui::pos2(40.0 + offset, 40.0 + offset),
        });

        self.selected_instance_id = Some(id);
        self.selected_pin = None;
        self.pending_connection = None;
        self.next_instance_id += 1;
    }
}

impl CanvasView {
    fn instance_rect(&self, instance: &CanvasInstance) -> egui::Rect {
        egui::Rect::from_min_size(self.to_screen(instance.position), egui::vec2(160.0, 72.0))
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
    selected_pin: Option<&SelectedPin>,
) {
    ui.heading(format!("{}_{}", instance.primitive_name, instance.id));
    ui.label(format!("Primitive: {}", instance.primitive_name));
    ui.label(format!(
        "Position: {:.0}, {:.0}",
        instance.position.x, instance.position.y
    ));

    if let Some(pin) = selected_pin.filter(|pin| pin.instance_id == instance.id) {
        ui.separator();
        ui.label(format!("Selected pin: {}", pin.pin_name));
    }
}

fn update_pending_connection(
    pending_connection: &mut Option<SelectedPin>,
    connections: &mut Vec<CanvasConnection>,
    selected_pin: SelectedPin,
) {
    match pending_connection.take() {
        Some(from) if from != selected_pin => {
            connections.push(CanvasConnection {
                from,
                to: selected_pin,
            });
        }
        _ => {
            *pending_connection = Some(selected_pin);
        }
    }
}

fn draw_canvas_connections(
    painter: &egui::Painter,
    canvas: &CanvasView,
    instances: &[CanvasInstance],
    catalog: Option<&PrimitiveCatalog>,
    connections: &[CanvasConnection],
) {
    for connection in connections {
        let Some(from) = pin_view_for_selected_pin(canvas, instances, catalog, &connection.from)
        else {
            continue;
        };
        let Some(to) = pin_view_for_selected_pin(canvas, instances, catalog, &connection.to) else {
            continue;
        };

        draw_manhattan_connection(painter, &from, &to);
    }
}

fn pin_view_for_selected_pin(
    canvas: &CanvasView,
    instances: &[CanvasInstance],
    catalog: Option<&PrimitiveCatalog>,
    selected_pin: &SelectedPin,
) -> Option<PinView> {
    let catalog = catalog?;
    let instance = instances
        .iter()
        .find(|instance| instance.id == selected_pin.instance_id)?;
    let primitive = catalog.get(&instance.primitive_name)?;
    let rect = canvas.instance_rect(instance);

    pin_views(rect, primitive)
        .into_iter()
        .find(|pin| pin.name == selected_pin.pin_name)
}

fn draw_manhattan_connection(painter: &egui::Painter, from: &PinView, to: &PinView) {
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

fn pin_escape_position(pin: &PinView) -> egui::Pos2 {
    let escape = 24.0;

    match pin.side {
        PinSide::Left => pin.position - egui::vec2(escape, 0.0),
        PinSide::Right => pin.position + egui::vec2(escape, 0.0),
        PinSide::Top => pin.position - egui::vec2(0.0, escape),
        PinSide::Bottom => pin.position + egui::vec2(0.0, escape),
    }
}

fn format_selected_pin(pin: &SelectedPin) -> String {
    format!("{}:{}", pin.instance_id, pin.pin_name)
}

fn draw_canvas_instance(
    painter: &egui::Painter,
    rect: egui::Rect,
    instance: &CanvasInstance,
    primitive: Option<&PrimitiveManifest>,
    selected_pin: Option<&SelectedPin>,
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
        draw_instance_pins(painter, rect, instance.id, primitive, selected_pin);
    }
}

fn draw_instance_pins(
    painter: &egui::Painter,
    rect: egui::Rect,
    instance_id: usize,
    primitive: &PrimitiveManifest,
    selected_pin: Option<&SelectedPin>,
) {
    for pin_view in pin_views(rect, primitive) {
        let selected = selected_pin.is_some_and(|selected_pin| {
            selected_pin.instance_id == instance_id && selected_pin.pin_name == pin_view.name
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

struct PinView {
    name: String,
    role: PinRole,
    side: PinSide,
    position: egui::Pos2,
    label_position: egui::Pos2,
    align: egui::Align2,
}

fn pin_views(rect: egui::Rect, primitive: &PrimitiveManifest) -> Vec<PinView> {
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
        let (position, label_position, align) = match side {
            PinSide::Left => {
                let y = egui::lerp(rect.top() + 14.0..=rect.bottom() - 14.0, t);
                (
                    egui::pos2(rect.left(), y),
                    egui::pos2(rect.left() + 8.0, y),
                    egui::Align2::LEFT_CENTER,
                )
            }
            PinSide::Right => {
                let y = egui::lerp(rect.top() + 14.0..=rect.bottom() - 14.0, t);
                (
                    egui::pos2(rect.right(), y),
                    egui::pos2(rect.right() - 8.0, y),
                    egui::Align2::RIGHT_CENTER,
                )
            }
            PinSide::Top => {
                let x = egui::lerp(rect.left() + 18.0..=rect.right() - 18.0, t);
                (
                    egui::pos2(x, rect.top()),
                    egui::pos2(x, rect.top() + 8.0),
                    egui::Align2::CENTER_TOP,
                )
            }
            PinSide::Bottom => {
                let x = egui::lerp(rect.left() + 18.0..=rect.right() - 18.0, t);
                (
                    egui::pos2(x, rect.bottom()),
                    egui::pos2(x, rect.bottom() - 8.0),
                    egui::Align2::CENTER_BOTTOM,
                )
            }
        };

        views.push(PinView {
            name: pin.name.clone(),
            role: pin.role.clone(),
            side,
            position,
            label_position,
            align,
        });
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
