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

    pub fn run(self) -> Result<(), GlfwError> {
        let mut glfw = init(fail_on_errors!()).map_err(|e| GlfwError::Init(e))?;

        let mut size = self.size;

        let mut input = GlfwInput::default();

        let (mut window, receiver) = glfw.create_window(size[0], size[1], self.title, Windowed).ok_or(FailedToCreateWindow)?;

        window.set_all_polling(true);

        gl::load_with(|s| window.get_proc_address(s));

        self.on_window_init.iter().for_each(|init| {
            (&*init)(&mut GlfwWindowState {
                glfw: &mut glfw,
                window: &mut window,
                input: &mut input
            })
        });

        let mut elapsed = glfw.get_time();
        let mut fr = 0_f64;
        #[allow(unused_variables)]
        let mut fc = 0;

        let mut clock = GlfwClock {
            elapsed,
            delta: 0.0,
        };

        loop {
            let e = elapsed;
            elapsed = glfw.get_time();
            clock.delta = elapsed - e;
            clock.elapsed = elapsed;
            let should_close = window.should_close();
            if should_close {
                return Ok(());
            }
            let mut window_state = GlfwWindowState {
                glfw: &mut glfw,
                window: &mut window,
                input: &mut input
            };
            for (_, b) in flush_messages(&receiver) {
                match b {
                    WindowEvent::Pos(x, y) => {
                        for opc in &self.on_position_changed {
                            (&*opc)(&mut window_state, x, y);
                        }
                    }
                    WindowEvent::Size(w, h) => {
                        size = [w as u32, h as u32];
                        for opc in &self.on_size_changed {
                            (&*opc)(&mut window_state, w, h);
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
                        for opc in &self.on_mouse_button {
                            (&*opc)(&mut window_state, a, b, c);
                        }
                    }
                    WindowEvent::CursorPos(x, y) => {
                        window_state.input.update_mouse_position(x, y, size[1] as f64);
                        for opc in &self.on_cursor_position_changed {
                            (&*opc)(&mut window_state, x, y);
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
                    input: &mut input
                }, elapsed)
            });
            let render = match self.render_strategy {
                GlfwRenderStrategy::Uncapped => true,
                GlfwRenderStrategy::Interval(f) => {
                    let mut out = false;

                    fr += clock.delta;
                    if fr >= f {
                        fr -= f;
                        fc += 1;
                        out = true;
                    }
                    out
                },
                GlfwRenderStrategy::FPS(fps) => {
                    let mut out = false;
                    let f = fps as f64 / 60_f64 / 60_f64;
                    fr += clock.delta;
                    if fr >= f {
                        fc += 1;
                        fr -= f;
                        out = true;
                    }
                    out
                }
            };

            if render {
                self.on_window_render.iter().for_each(|init| {
                    (&*init)(&mut GlfwWindowState {
                        glfw: &mut glfw,
                        window: &mut window,
                        input: &mut input
                    }, elapsed)
                });
            }

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}