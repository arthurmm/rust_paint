mod drawing;
mod tools;

use eframe::egui::{
    self, Color32, Sense, Stroke,
};

use crate::drawing::PaintStroke;
use crate::tools::{PaintTool, ToolSettings};

pub struct PaintApp {
    strokes: Vec<PaintStroke>,
    active_tool: PaintTool,
    tool_settings: ToolSettings,
    frames: u32,
}

impl Default for PaintApp {
    fn default() -> Self {
        Self {
            strokes: Vec::new(),
            active_tool: PaintTool::default(),
            tool_settings: ToolSettings::default(),
            frames: 0,
        }
    }
}

impl PaintApp {
    fn new_stroke(&mut self) {
        let stroke = PaintStroke {
            points: vec![],
            color: Color32::from_rgb(
                (self.tool_settings.color[0] * 255.0) as u8,
                (self.tool_settings.color[1] * 255.0) as u8,
                (self.tool_settings.color[2] * 255.0) as u8,
            ),
            width: self.tool_settings.width,
        };
        self.strokes.push(stroke);
    }

    fn is_point_erased(point: egui::Vec2, eraser_pos: egui::Vec2, radius: f32) -> bool {
        (point - eraser_pos).length() <= radius
    }

    fn split_points(points: &[egui::Vec2], eraser_pos: egui::Vec2, radius: f32) -> Vec<Vec<egui::Vec2>> {
        let mut temp_points = Vec::new();
        let mut return_strokes = Vec::new();
        for point in points {
            if Self::is_point_erased(*point, eraser_pos, radius) {
                return_strokes.push(std::mem::take(&mut temp_points));
            } else {
                temp_points.push(*point);
            }
        }
        return_strokes.push(std::mem::take(&mut temp_points));
        return_strokes.retain(|stroke| !stroke.is_empty());
        return_strokes
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frames += 1;
        if self.frames == 1 {
            egui::CentralPanel::default().show(ctx, |_| {});
            ctx.request_repaint();
            return;
        }
        
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.heading("Rust Paint");

            if ui.button("Limpar").clicked() {
                self.strokes.clear();
            }

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tool, PaintTool::Brush, "Pincel");
                ui.selectable_value(&mut self.active_tool, PaintTool::Eraser, "Borracha");
            });

            ui.horizontal(|ui| {
                ui.color_edit_button_rgb(&mut self.tool_settings.color);
                ui.add(egui::Slider::new(&mut self.tool_settings.width, 1.0..=10.0).text("Largura"));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::drag());
            let size = ui.available_size().max(egui::vec2(600.0, 400.0));
            // let size = egui::vec2(600.0, 400.0);
            let (response, painter) = ui.allocate_painter(size, Sense::drag());

            // Fundo branco
            painter.rect_filled(response.rect, 0.0, Color32::WHITE);
        
            // Começa um novo traço
            if response.drag_started() {
                match self.active_tool {
                    PaintTool::Brush => self.new_stroke(),
                    PaintTool::Eraser => (),
                }
            }

            // Adiciona pontos enquanto arrastamos
            if response.dragged() {
                match self.active_tool {
                    PaintTool::Brush => {
                        if let Some(pos) = response.interact_pointer_pos() {
                            if let Some(stroke) = self.strokes.last_mut() {
                                let relative_pos = pos - response.rect.min;
                                stroke.points.push(relative_pos);
                            }
                        }
                    }
                    PaintTool::Eraser => {
                        if let Some(pos) = response.interact_pointer_pos() {
                            let eraser_pos = pos - response.rect.min;
                            let radius = self.tool_settings.width / 2.0;
                            let mut full_new_strokes = Vec::new();
                            for stroke in &self.strokes {
                                let points = Self::split_points(&stroke.points, eraser_pos, radius);
                                for points in points {
                                    full_new_strokes.push(PaintStroke { points, color: stroke.color, width: stroke.width });
                                }
                            }
                            self.strokes = std::mem::take(&mut full_new_strokes);
                        }
                    }
                }
            }

            // Renderiza todos os traços
            for paint_stroke in &self.strokes {
                for stroke in paint_stroke.points.windows(2) {
                    painter.line_segment(
                        [response.rect.min + stroke[0], response.rect.min + stroke[1]], 
                        Stroke::new(paint_stroke.width, paint_stroke.color)
                    );
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Rust Paint", 
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(PaintApp::default())))
    )
}