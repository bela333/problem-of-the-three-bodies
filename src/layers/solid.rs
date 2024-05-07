use super::Layer;

pub struct Solid{
    color: nalgebra::Vector4<f32>
}

impl Solid{
    pub fn new(color: nalgebra::Vector4<f32>) -> Self{
        Self { color }
    }
}

impl Layer for Solid{
    fn render(&self, pos: nalgebra::Vector2<f32>, frame: u32) -> nalgebra::Vector4<f32> {
        self.color
    }
}