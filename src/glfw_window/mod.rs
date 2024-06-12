use crate::glfw_event_callbacks::*;
use crate::glfw_render_strategy::GlfwRenderStrategy;

pub mod imp;
pub mod builder_macro;

#[derive(Default)]
pub struct GlfwWindow<'a> {
    pub size: [u32; 2],
    pub title: &'a str,
    render_strategy: GlfwRenderStrategy,

    on_position_changed: Vec<Box<GlfwWindowPositionChanged>>,
    on_size_changed: Vec<Box<GlfwWindowSizeChanged>>,
    on_window_close: Vec<Box<GlfwWindowClose>>,
    on_window_refresh: Vec<Box<GlfwWindowRefresh>>,
    on_window_focus: Vec<Box<GlfwWindowFocus>>,
    on_window_iconify: Vec<Box<GlfwWindowIconify>>,
    on_frame_buffer_size_changed: Vec<Box<GlfwWindowFrameBufferSizeChanged>>,
    on_mouse_button: Vec<Box<GlfwWindowMouseButton>>,
    on_cursor_position_changed: Vec<Box<GlfwCursorPositionChanged>>,
    on_cursor_enter: Vec<Box<GlfwCursorEnter>>,
    on_scroll: Vec<Box<GlfwScroll>>,
    on_key: Vec<Box<GlfwWindowKey>>,
    on_char: Vec<Box<GlfwWindowChar>>,
    on_char_modifiers: Vec<Box<GlfwWindowCharModifiers>>,
    on_file_drop: Vec<Box<GlfwWindowFileDrop>>,
    on_maximize: Vec<Box<GlfwWindowMaximize>>,
    on_content_scale: Vec<Box<GlfwWindowContentScale>>,

    on_window_init: Vec<Box<GlfwWindowInitialize>>,
    on_window_update: Vec<Box<GlfwWindowUpdate>>,
    on_window_render: Vec<Box<GlfwWindowRender>>,
}