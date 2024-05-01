use glfw::{Action};
use super::Window;

pub struct Events {
    keys: [bool;1032],
    frames: [u32;1032],
    current: u32,
    pub delta_x: f32,
    pub delta_y: f32,
    x: f64,
    y: f64,
    pub cursor_locked: bool,
    cursor_started: bool,
}

impl Events {
    pub fn new() -> Self {
        let keys = [false; 1032];
        let frames = [0; 1032];
        Self {
            keys,
            frames,
            current: 0,
            delta_x: 0.0,
            delta_y: 0.0,
            x: 0.0,
            y: 0.0,
            cursor_locked: false,
            cursor_started: false,
        }
    }

    pub fn initialize(&mut self, window: &mut Window) {
        window.window.set_key_polling(true);
        window.window.set_mouse_button_polling(true);
        window.window.set_cursor_pos_polling(true);
        window.window.set_cursor_enter_polling(true);
        window.window.set_size_polling(true);
    }

    fn set_key(&mut self, key: usize, action: Action) {
        match action {
            Action::Press => {
                self.keys[key] = true;
                self.frames[key] = self.current;
            }
            Action::Release => {
                self.keys[key] = false;
                self.frames[key] = self.current;
            }
            _ => {}
        }
    }

    pub fn pull_events(&mut self, window: &mut Window) {
        self.current += 1;
        self.delta_x = 0.0;
        self.delta_y = 0.0;

        for (_, event) in glfw::flush_messages(&window.receiver) {
            match event {
                glfw::WindowEvent::Size(w, h) => {
                    unsafe {
                        gl::Viewport(0, 0, w, h);
                    }
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    if self.cursor_started {
                        self.delta_x += (xpos - self.x) as f32;
                        self.delta_y += (ypos - self.y) as f32;
                    } else {
                        self.cursor_started = true;
                    }
                    self.x = xpos;
                    self.y = ypos;
                }
                glfw::WindowEvent::MouseButton(button, action, _) => {
                    let button_index = match button {
                        glfw::MouseButton::Button1 => 1024,
                        glfw::MouseButton::Button2 => 1025,
                        glfw::MouseButton::Button3 => 1026,
                        glfw::MouseButton::Button4 => 1027,
                        glfw::MouseButton::Button5 => 1028,
                        glfw::MouseButton::Button6 => 1029,
                        glfw::MouseButton::Button7 => 1030,
                        glfw::MouseButton::Button8 => 1031,
                    };

                    self.set_key(button_index, action);
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    self.set_key(key as usize, action);
                }
                _ => {}
            }
        }
    }

    pub fn pressed(&self, keycode: i32) -> bool {
        let keycode = keycode as usize;
        if keycode >= 1032 {
            return false;
        }
        self.keys[keycode]
    }

    pub fn jpressed(&self, keycode: i32) -> bool {
        let keycode = keycode as usize;
        if keycode >= 1032 {
            return false;
        }
        self.keys[keycode] && self.frames[keycode] == self.current
    }

    pub fn _clicked(&self, button: i32) -> bool {
        let button_index = (button + 1024) as usize;
        self.keys[button_index]
    }

    pub fn jclicked(&self, button: i32) -> bool {
        let button_index = (button + 1024) as usize;
        self.keys[button_index] && self.frames[button_index] == self.current
    }
    pub fn toggle_cursor(&mut self) -> glfw::CursorMode{
            self.cursor_locked = !self.cursor_locked;
            if self.cursor_locked {
                glfw::CursorMode::Disabled
            } else {
                glfw::CursorMode::Normal
            }
    }
}
