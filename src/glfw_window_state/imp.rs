use crate::glfw_window_state::GlfwWindowState;

impl <'a> GlfwWindowState<'a> {
    pub fn force_next_render(&mut self, render: bool) {
        self.force_render = render;
    }
}