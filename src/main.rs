use eframe::egui::{self, Color32, Button, color_picker, Sense, Stroke, Vec2, Pos2};

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Hue", native_options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))));
}

pub struct MyApp {
    picked_color : Color32,
    strokes: Vec<(Pos2, Pos2)>,
    last_pos: Option<Pos2>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            picked_color: Color32::from_rgb(0, 0, 0),
            strokes: vec![],
            last_pos: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

           if ui.add(
                Button::new("Clear")
            ).clicked() {
                self.strokes.clear();
            }
            color_picker::color_edit_button_srgba(ui, &mut self.picked_color, egui::color_picker::Alpha::OnlyBlend);

            let canvas_size = Vec2::new(1000.0, 1000.0);

            // Allocate space + painter
            let (response, painter) = ui.allocate_painter(canvas_size, Sense::drag());

            // Draw canvas background
            painter.rect_filled(response.rect, 0.0, Color32::WHITE);

            // Handle mouse input
            if response.dragged() {
                if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
                    if let Some(last) = self.last_pos {
                        self.strokes.push((last, pos));
                    }
                    self.last_pos = Some(pos);
                }
            } else {
                self.last_pos = None;
            }

            // Draw all strokes
            for (start, end) in &self.strokes {
                painter.line_segment(
                    [*start, *end],
                    Stroke::new(2.0, self.picked_color),
                );
            }
        });
    }
}
