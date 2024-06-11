use std::path::PathBuf;
use glfw::{Action, Key, Modifiers, MouseButton, Scancode};

use crate::glfw_window_state::GlfwWindowState;

pub type GlfwWindowInitialize = dyn Fn(&mut GlfwWindowState);
pub type GlfwWindowUpdate = dyn Fn(&mut GlfwWindowState, f64);
pub type GlfwWindowRender = dyn Fn(&mut GlfwWindowState, f64);

pub type GlfwWindowPositionChanged = dyn Fn(&mut GlfwWindowState, i32, i32);
pub type GlfwWindowSizeChanged = dyn Fn(&mut GlfwWindowState, i32, i32);
pub type GlfwWindowClose = dyn Fn(&mut GlfwWindowState);
pub type GlfwWindowRefresh = dyn Fn(&mut GlfwWindowState);
pub type GlfwWindowFocus = dyn Fn(&mut GlfwWindowState, bool);
pub type GlfwWindowIconify = dyn Fn(&mut GlfwWindowState, bool);
pub type GlfwWindowFrameBufferSizeChanged = dyn Fn(&mut GlfwWindowState, i32, i32);
pub type GlfwWindowMouseButton = dyn Fn(&mut GlfwWindowState, MouseButton, Action, Modifiers);
pub type GlfwCursorPositionChanged = dyn Fn(&mut GlfwWindowState, f64, f64);
pub type GlfwCursorEnter = dyn Fn(&mut GlfwWindowState, bool);
pub type GlfwScroll = dyn Fn(&mut GlfwWindowState, f64, f64);
pub type GlfwWindowKey = dyn Fn(&mut GlfwWindowState, Key, Scancode, Action, Modifiers);
pub type GlfwWindowChar = dyn Fn(&mut GlfwWindowState, char);
pub type GlfwWindowCharModifiers = dyn Fn(&mut GlfwWindowState, char, Modifiers);
pub type GlfwWindowFileDrop = dyn Fn(&mut GlfwWindowState, Vec<PathBuf>);
pub type GlfwWindowMaximize = dyn Fn(&mut GlfwWindowState, bool);
pub type GlfwWindowContentScale = dyn Fn(&mut GlfwWindowState, f32, f32);