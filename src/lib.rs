pub mod glfw_event_callbacks;
pub mod glfw_window;
pub mod glfw_error;
pub mod glfw_window_state;
pub mod glfw_render_strategy;
pub mod glfw_clock;
pub mod glfw_input;

#[cfg(test)]
mod tests {
    use crate::glfw_render_strategy::GlfwRenderStrategy::FPS;
    use crate::glfw_window::GlfwWindow;

    #[test]
    fn it_works() {
        let window = GlfwWindow::new([500; 2], "Hello")
            .with_render_strategy(FPS(60))
            .with_on_window_close(|_state| {})
            .with_on_window_update(|state| {
                state.window.set_title(&format!("{:?}", state.input.mouse().mouse_button_held()));
            });
        window.run()
            .unwrap();
    }
}
