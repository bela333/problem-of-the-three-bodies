use nalgebra::{vector, Vector4};

pub const RED: Vector4<f32> = vector![1.0, 0.0, 0.0, 1.0];
pub const GREEN: Vector4<f32> = vector![0.0, 1.0, 0.0, 1.0];
pub const BLUE: Vector4<f32> = vector![0.0, 0.0, 1.0, 1.0];

pub trait VectorExtensions {
    fn set_alpha(self, a: f32) -> Self;
    fn to_srgb(self) -> Self;
    fn from_srgb(self) -> Self;
}
impl VectorExtensions for Vector4<f32> {
    fn set_alpha(mut self, a: f32) -> Self {
        self[3] = a;
        self
    }

    fn to_srgb(mut self) -> Self {
        self[0] = self[0].powf(2.2f32.recip());
        self[1] = self[1].powf(2.2f32.recip());
        self[2] = self[2].powf(2.2f32.recip());

        self
    }

    fn from_srgb(mut self) -> Self {
        self[0] = self[0].powf(2.2);
        self[1] = self[1].powf(2.2);
        self[2] = self[2].powf(2.2);

        self
    }
}
