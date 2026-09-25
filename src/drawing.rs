use eframe::egui::{
    Color32, Vec2,
};

#[derive(Debug, Clone)]
pub struct PaintStroke {
    pub points: Vec<Vec2>,
    pub color: Color32,
    pub width: f32,
}

impl Default for PaintStroke {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            color: Color32::BLACK,
            width: 2.0_f32,
        }
    }
}