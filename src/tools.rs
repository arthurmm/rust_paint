#[derive(Default, PartialEq)]
pub enum PaintTool {
    #[default]
    Brush,
    Eraser,
}

pub struct ToolSettings {
    pub color: [f32; 3],
    pub width: f32,
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            color: [0.0, 0.0, 0.0],
            width: 2.0_f32,
        }
    }
}