use nalgebra::Vector4;

pub mod circle;
pub trait Layer: Sized {
    fn render(&self, pos: nalgebra::Vector2<f32>, frame: u32) -> nalgebra::Vector4<f32>;
    fn over<L: Layer + Sized>(self, top: L) -> (Self, L) {
        (self, top)
    }
}

impl Layer for () {
    fn render(&self, pos: nalgebra::Vector2<f32>, frame: u32) -> nalgebra::Vector4<f32> {
        return Vector4::zeros();
    }
}

impl<L1, L2> Layer for (L1, L2)
where
    L1: Layer,
    L2: Layer,
{
    fn render(&self, pos: nalgebra::Vector2<f32>, frame: u32) -> nalgebra::Vector4<f32> {
        let (l1, l2) = self;
        let l2color = l2.render(pos, frame);
        let alpha = l2color.w;
        if alpha >= 1.0 {
            return l2color;
        }
        let l1color = l1.render(pos, frame);
        if alpha <= 0.0 {
            return l1color;
        }
        return l1color.lerp(&l2color, alpha);
    }
}
