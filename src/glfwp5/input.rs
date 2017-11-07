#[cfg(feature = "glfw")]

use glfw;

use {Screen, Key, MouseButton};

impl<'a> Screen<'a> {
    pub fn key_press<I: Into<glfw::Key>>(&mut self, button: I) -> bool {
        match self.keypressed {
            Some(k) => {
                let b: glfw::Key = button.into();
                if k == b {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn SpaceWait(&mut self) {
        self.glfw.wait_events();
        for (_, event) in glfw::flush_messages(&self.events_loop) {
            match event {
                glfw::WindowEvent::Key(key, scancode, action, mods) => {
                    match (key, action) {
                        (glfw::Key::Space, glfw::Action::Press) => break,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    pub fn MousePress<B: Into<glfw::MouseButton>>(&mut self, button: B) -> bool {
        match self.mousepressed {
            Some(b) => {
                let btn: glfw::MouseButton = button.into();
                if b == btn {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn MouseRelease<B: Into<glfw::MouseButton>>(&mut self, button: B) -> bool {
        match self.mousereleased {
            Some(b) => {
                let btn: glfw::MouseButton = button.into();
                if b == btn {
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    pub fn MouseX(&mut self) -> f64 {
        self.mousepos.0
    }

    pub fn MouseY(&mut self) -> f64 {
        self.mousepos.1
    }
}
