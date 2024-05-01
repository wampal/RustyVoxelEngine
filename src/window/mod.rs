pub mod events;
mod camera;
pub use camera::Camera;

use glfw::{ fail_on_errors, ffi::glfwTerminate, Context, GlfwReceiver, WindowEvent };

pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    pub receiver: GlfwReceiver<(f64, WindowEvent)>
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, String> {
        let mut glfw = glfw
            ::init(fail_on_errors!())
            .map_err(|e| format!("GLFW initialization failed: {}", e))?;

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::Resizable(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .ok_or_else(|| String::from("Failed to create GLFW window"))?;
        window.make_current();
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Ok(Self { glfw, window , receiver: events})
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }
    pub fn _get_size(&self) -> (i32, i32) {
        self.window.get_size()
    }
    pub fn width(&self) -> i32{
        self.window.get_size().0
    }
    pub fn height(&self) -> i32{
        self.window.get_size().1
    }

    pub fn set_should_close(&mut self, should_close: bool) {
        self.window.set_should_close(should_close)
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }
    pub fn terminate(&mut self) {
        unsafe { glfwTerminate() }
    }
}
