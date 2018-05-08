#[cfg(feature = "glfw")]

use glfw;

use {Screen, Key, MouseButton};

impl<'a> Screen<'a> {
	/// Check if the given key was pressed since the last call to screen.release()
	/// or screen.poll_events().
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

	/// Pause the program and wait for the space bar to be pressed. This is a
	/// convienence which is useful for debugging and also in psychological experiments.
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

	/// Check if the given mouse button was pressed since the last call to
	/// screen.reveal() or screen.poll_events().
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

	/// Check if the given mouse button was released since the last call to
	/// screen.reveal() or screen.poll_events().
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

	/// What was the x-coordinate of the mouse at the last call to screen.reveal()
	/// or screen.poll_events().
    pub fn MouseX(&mut self) -> f64 {
        self.mousepos.0
    }

	/// What was the y-coordinate of the mouse at the last call to screen.reveal()
	/// or screen.poll_events().
    pub fn MouseY(&mut self) -> f64 {
        self.mousepos.1
    }
}
