use std::path::PathBuf;
use glfw::*;
use glfw::WindowMode::Windowed;

use crate::glfw_clock::GlfwClock;
use crate::glfw_error::GlfwError;
use crate::glfw_error::GlfwError::FailedToCreateWindow;
use crate::glfw_input::GlfwInput;
use crate::glfw_render_strategy::GlfwRenderStrategy;
use crate::glfw_window::GlfwWindow;
use crate::glfw_window_state::GlfwWindowState;

impl<'a> GlfwWindow<'a> {
    pub fn new(size: [u32; 2], title: &'a str) -> GlfwWindow<'a> {
        Self {
            size,
            title,
            ..Default::default()
        }
    }

    pub fn with_render_strategy(mut self, glfw_render_strategy: GlfwRenderStrategy) -> Self {
        self.render_strategy = glfw_render_strategy;
        self
    }

    crate::builder! {
        on_window_init => Fn(&mut GlfwWindowState),
        on_window_update => Fn(&mut GlfwWindowState),
        on_window_render => Fn(&mut GlfwWindowState),

        on_position_changed => Fn(&mut GlfwWindowState),
        on_size_changed => Fn(&mut GlfwWindowState),
        on_window_close => Fn(&mut GlfwWindowState),
        on_window_refresh => Fn(&mut GlfwWindowState),
        on_window_focus => Fn(&mut GlfwWindowState, bool),
        on_window_iconify => Fn(&mut GlfwWindowState, bool),
        on_frame_buffer_size_changed => Fn(&mut GlfwWindowState, i32, i32),
        on_mouse_button => Fn(&mut GlfwWindowState, MouseButton, Action, Modifiers),
        on_cursor_position_changed => Fn(&mut GlfwWindowState),
        on_cursor_enter => Fn(&mut GlfwWindowState, bool),
        on_scroll => Fn(&mut GlfwWindowState, f64, f64),
        on_key => Fn(&mut GlfwWindowState, Key, Scancode, Action, Modifiers),
        on_char => Fn(&mut GlfwWindowState, char),
        on_char_modifiers => Fn(&mut GlfwWindowState, char, Modifiers),
        on_file_drop => Fn(&mut GlfwWindowState, Vec<PathBuf>),
        on_maximize => Fn(&mut GlfwWindowState, bool),
        on_content_scale => Fn(&mut GlfwWindowState, f32, f32)
    }

    pub fn with_resize_viewport(mut self, resize_viewport: bool) -> Self {
        self.resize_viewport_on_resize = resize_viewport;
        self
    }

    pub fn run(self) -> Result<(), GlfwError> {
        let mut glfw = init(fail_on_errors!()).map_err(|e| GlfwError::Init(e))?;


        let mut input = GlfwInput::default();

        let (mut window, receiver) = glfw.create_window(self.size[0], self.size[1], self.title, Windowed).ok_or(FailedToCreateWindow)?;
        window.set_all_polling(true);


        let mut size = self.size;
        let mut position = {
            let p = window.get_pos();
            [p.0, p.1]
        };

        gl::load_with(|s| window.get_proc_address(s));

        let mut clock = GlfwClock::default();

        self.on_window_init.iter().for_each(|init| {
            (&*init)(&mut GlfwWindowState {
                glfw: &mut glfw,
                window: &mut window,
                input: &mut input,
                clock: &clock,
                window_position: position,
                force_render: false,
                window_size: size,
            })
        });

        loop {
            let render = clock.update(glfw.get_time(), &self.render_strategy);

            let should_close = window.should_close();
            if should_close {
                return Ok(());
            }
            let mut window_state = GlfwWindowState {
                glfw: &mut glfw,
                window: &mut window,
                input: &mut input,
                clock: &clock,
                window_position: position,
                force_render: false,
                window_size: size,
            };
            for (_, b) in flush_messages(&receiver) {
                match b {
                    WindowEvent::Pos(x, y) => {
                        position = [x, y];
                        window_state.window_position = position;
                        for opc in &self.on_position_changed {
                            (&*opc)(&mut window_state);
                        }
                    }
                    WindowEvent::Size(w, h) => {
                        size = [w as u32, h as u32];
                        if self.resize_viewport_on_resize {
                            unsafe {
                                gl::Viewport(0, 0, w, h);
                            }
                        }
                        for opc in &self.on_size_changed {
                            (&*opc)(&mut window_state);
                        }
                    }
                    WindowEvent::Close => {
                        for opc in &self.on_window_close {
                            (&*opc)(&mut window_state);
                        }
                    }
                    WindowEvent::Refresh => {
                        for opc in &self.on_window_refresh {
                            (&*opc)(&mut window_state);
                        }
                    }
                    WindowEvent::Focus(focus) => {
                        for opc in &self.on_window_focus {
                            (&*opc)(&mut window_state, focus);
                        }
                    }
                    WindowEvent::Iconify(iconify) => {
                        for opc in &self.on_window_iconify {
                            (&*opc)(&mut window_state, iconify);
                        }
                    }
                    WindowEvent::FramebufferSize(w, h) => {
                        for opc in &self.on_frame_buffer_size_changed {
                            (&*opc)(&mut window_state, w, h);
                        }
                    }
                    WindowEvent::MouseButton(a, b, c) => {
                        window_state.input.process_mouse_button(a, b);
                        for opc in &self.on_mouse_button {
                            (&*opc)(&mut window_state, a, b, c);
                        }
                    }
                    WindowEvent::CursorPos(x, y) => {
                        window_state.input.update_mouse_position(x, y, size[1] as f64);
                        for opc in &self.on_cursor_position_changed {
                            (&*opc)(&mut window_state);
                        }
                    }
                    WindowEvent::CursorEnter(enter) => {
                        for opc in &self.on_cursor_enter {
                            (&*opc)(&mut window_state, enter);
                        }
                    }
                    WindowEvent::Scroll(x, y) => {
                        for opc in &self.on_scroll {
                            (&*opc)(&mut window_state, x, y);
                        }
                    }
                    WindowEvent::Key(a, b, c, d) => {
                        window_state.input.process_key(a, c);
                        for opc in &self.on_key {
                            (&*opc)(&mut window_state, a, b, c, d);
                        }
                    }
                    WindowEvent::Char(c) => {
                        for opc in &self.on_char {
                            (&*opc)(&mut window_state, c);
                        }
                    }
                    WindowEvent::CharModifiers(a, b) => {
                        for opc in &self.on_char_modifiers {
                            (&*opc)(&mut window_state, a, b);
                        }
                    }
                    WindowEvent::FileDrop(files) => {
                        for opc in &self.on_file_drop {
                            (&*opc)(&mut window_state, files.clone());
                        }
                    }
                    WindowEvent::Maximize(max) => {
                        for opc in &self.on_maximize {
                            (&*opc)(&mut window_state, max);
                        }
                    }
                    WindowEvent::ContentScale(w, h) => {
                        for opc in &self.on_content_scale {
                            (&*opc)(&mut window_state, w, h);
                        }
                    }
                }
            }
            self.on_window_update.iter().for_each(|init| {
                (&*init)(&mut GlfwWindowState {
                    glfw: &mut glfw,
                    window: &mut window,
                    input: &mut input,
                    clock: &clock,
                    window_position: position,
                    force_render: false,
                    window_size: size,
                })
            });
            if render {
                self.on_window_render.iter().for_each(|init| {
                    (&*init)(&mut GlfwWindowState {
                        glfw: &mut glfw,
                        window: &mut window,
                        input: &mut input,
                        clock: &clock,
                        window_position: position,
                        force_render: false,
                        window_size: size,
                    })
                });
            }

            window.swap_buffers();
            glfw.poll_events();
            input.update();
        }
    }
}