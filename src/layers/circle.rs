use nalgebra::Vector4;

use super::Layer;

pub struct Circle {
    pub pos: nalgebra::Vector2<f32>,
    pub radius_squared: f32,
    pub color: nalgebra::Vector4<f32>,
}

impl Circle {
    pub fn new(pos: nalgebra::Vector2<f32>, radius: f32, color: nalgebra::Vector4<f32>) -> Self {
        Self {
            radius_squared: radius.powi(2),
            pos,
            color,
        }
    }
}

impl Layer for Circle {
    fn render(&self, pos: nalgebra::Vector2<f32>, frame: u32) -> nalgebra::Vector4<f32> {
        let dist = (pos - self.pos).norm_squared();
        if dist < self.radius_squared {
            return self.color;
        }
        return Vector4::zeros();
    }
}
