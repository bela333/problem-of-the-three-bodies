use super::{circle::Circle, glow::Glow};

pub fn new(pos: nalgebra::Vector2<f32>, radius: f32, color: nalgebra::Vector4<f32>) -> (Circle, Glow) {
    (Circle::new(pos, radius, color), Glow::new(pos, radius, color))
}