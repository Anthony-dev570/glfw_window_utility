pub mod imp;

#[derive(Debug, Clone, Copy)]
pub struct GlfwClock {
    elapsed: f64,
    delta: f64,
    fr: f64,
}