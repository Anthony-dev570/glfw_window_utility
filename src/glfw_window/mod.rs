use crate::glfw_event_callbacks::*;
use crate::glfw_render_strategy::GlfwRenderStrategy;

pub mod imp;
pub mod builder_macro;

#[derive(Default)]
pub struct GlfwWindow<'a> {
    pub size: [u32; 2],
    pub title: &'a str,
    pub render_strategy: GlfwRenderStrategy,

    pub on_position_changed: Vec<Box<GlfwWindowPositionChanged>>,
    pub on_size_changed: Vec<Box<GlfwWindowSizeChanged>>,
    pub on_window_close: Vec<Box<GlfwWindowClose>>,
    pub on_window_refresh: Vec<Box<GlfwWindowRefresh>>,
    pub on_window_focus: Vec<Box<GlfwWindowFocus>>,
    pub on_window_iconify: Vec<Box<GlfwWindowIconify>>,
    pub on_frame_buffer_size_changed: Vec<Box<GlfwWindowFrameBufferSizeChanged>>,
    pub on_mouse_button: Vec<Box<GlfwWindowMouseButton>>,
    pub on_cursor_position_changed: Vec<Box<GlfwCursorPositionChanged>>,
    pub on_cursor_enter: Vec<Box<GlfwCursorEnter>>,
    pub on_scroll: Vec<Box<GlfwScroll>>,
    pub on_key: Vec<Box<GlfwWindowKey>>,
    pub on_char: Vec<Box<GlfwWindowChar>>,
    pub on_char_modifiers: Vec<Box<GlfwWindowCharModifiers>>,
    pub on_file_drop: Vec<Box<GlfwWindowFileDrop>>,
    pub on_maximize: Vec<Box<GlfwWindowMaximize>>,
    pub on_content_scale: Vec<Box<GlfwWindowContentScale>>,

    pub on_window_init: Vec<Box<GlfwWindowInitialize>>,
    pub on_window_update: Vec<Box<GlfwWindowUpdate>>,
    pub on_window_render: Vec<Box<GlfwWindowRender>>
}