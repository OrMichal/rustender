#[allow(dead_code)]
pub trait Shape {
    fn volume(&self) -> f32;
    fn area(&self) -> f32;
}
