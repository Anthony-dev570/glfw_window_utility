use crate::glfw_clock::GlfwClock;
use crate::glfw_render_strategy::GlfwRenderStrategy;

impl Default for GlfwClock {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            delta: 0.0,
            fr: 0.0,
        }
    }
}

impl GlfwClock {
    pub(crate) fn update(&mut self, elapsed: f64, glfw_render_strategy: &GlfwRenderStrategy) -> bool {
        self.delta = elapsed - self.elapsed;
        self.elapsed = elapsed;

        match glfw_render_strategy {
            GlfwRenderStrategy::Uncapped => true,
            GlfwRenderStrategy::Interval(f) => {
                let mut out = false;

                self.fr += f;
                if self.fr >= *f {
                    self.fr -= *f;
                    out = true;
                }

                out
            }
            GlfwRenderStrategy::FPS(f) => {
                let mut out = false;

                let f = *f as f64 / 60_f64 / 60_f64;
                self.fr += self.delta;

                if self.fr >= f {
                    self.fr -= f;
                    out = true;
                }
                out
            }
        }
    }
    pub fn elapsed(&self) -> f64 {
        self.elapsed
    }
    pub fn delta(&self) -> f64 {
        self.delta
    }
}