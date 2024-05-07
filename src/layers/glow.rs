use nalgebra::Vector4;

use crate::colors::VectorExtensions;

use super::Layer;

pub struct Glow{
    pub pos: nalgebra::Vector2<f32>,
    pub radius: f32,
    pub color: nalgebra::Vector4<f32>,
    pub lambda: f32
}

impl Glow{
    pub fn new(pos: nalgebra::Vector2<f32>, radius: f32, color: nalgebra::Vector4<f32>) -> Self {
        Self {
            radius,
            pos,
            color,
            lambda: radius.powi(2)
        }
    }
}


impl Layer for Glow{
    fn render(&self, pos: nalgebra::Vector2<f32>, frame: u32) -> nalgebra::Vector4<f32> {
        let dist = (self.pos-pos).norm();
        if dist < self.radius{
            return Vector4::zeros();
        }
        let inverse_square = dist.powi(2).recip() * self.lambda;
        return self.color.set_alpha(self.color.w*inverse_square);
    }
}