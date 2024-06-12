pub mod imp;

#[derive(Debug, Clone, Copy)]
pub struct GlfwMouseInput {
    mouse_position: [f64; 2],
    mouse_delta: [f64; 2],

    relative_mouse_position: [f64; 2],
    relative_mouse_position_delta: [f64; 2],

    mouse_button_pressed: [bool; 128],
    mouse_button_released: [bool; 128],
    mouse_button_held: [bool; 128],
}