pub mod imp;

#[derive(Debug, Clone)]
pub struct GlfwKeyboardInput {
    pressed: [bool; 512],
    released: [bool; 512],
    held: [bool; 512]
}