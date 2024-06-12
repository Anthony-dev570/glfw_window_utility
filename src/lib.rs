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
        let mut window = GlfwWindow::new([500; 2], "Hello");
        window.render_strategy = FPS(60);
        window.on_window_close.push(Box::new(|_| {

        }));
        window.on_window_update.push(Box::new(|_a, t| {
            //_a.window.set_title(&format!("{t}"));
            _a.window.set_title(&format!("{:?}", _a.input.mouse()));
            if t >= 10.0 {
                _a.window.set_should_close(true);
            }
        }));

        window.run()
            .unwrap();
    }
}
