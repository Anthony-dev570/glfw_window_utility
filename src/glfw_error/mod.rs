use glfw::InitError;

#[derive(Debug, Clone)]
pub enum GlfwError {
    Init(InitError),
    FailedToCreateWindow
}