#[derive(Debug, Clone, Copy, Default)]
pub enum GlfwRenderStrategy {
    #[default]
    Uncapped,
    Interval(f64),
    FPS(u16)
}