#[cfg(feature = "glfw")]

use glium;
use glfw;

use Screen;

impl<'a> Screen<'a> {
    #[inline]
    pub fn reset_cursor(&mut self) {
        self.currCursor = glfw::StandardCursor::Arrow;
        self.display.gl_window_mut().set_cursor(
            Some(glfw::Cursor::standard(
                self.currCursor,
            )),
        );
    }

    #[inline]
    pub fn cursor(&mut self, cursorType: &str) {
        self.display.gl_window_mut().set_cursor_mode(
            glfw::CursorMode::Normal,
        );
        if cursorType == "HAND" {
            self.currCursor = glfw::StandardCursor::Hand;
        } else if cursorType == "ARROW" {
            self.currCursor = glfw::StandardCursor::Arrow;
        } else if cursorType == "CROSS" {
            self.currCursor = glfw::StandardCursor::Crosshair;
        } else if cursorType == "MOVE" {
            self.currCursor = glfw::StandardCursor::Crosshair;
        } else if cursorType == "TEXT" {
            self.currCursor = glfw::StandardCursor::IBeam;
        } else if cursorType == "WAIT" {
            // self.currCursor = glfw::StandardCursor::Wait;
        }
        self.display.gl_window_mut().set_cursor(
            Some(glfw::Cursor::standard(
                self.currCursor,
            )),
        );
    }

    #[inline]
    pub fn focused(&mut self) -> bool {
        let mut focused = false;
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events_loop) {
            match event {
                glfw::WindowEvent::Focus(true) => {
                    focused = true;
                }
                _ => (),
            }
        }

        focused
    }

    #[inline]
    pub fn frameCount(&self) -> isize {
        self.frameCount
    }

    #[inline]
    pub fn get_frameRate(&self) -> isize {
        self.frameRate
    }

    #[inline]
    pub fn set_frameRate(&mut self, fRate: isize) {
        self.frameRate = fRate;
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn noCursor(&mut self) {
        self.display.gl_window_mut().set_cursor_mode(
            glfw::CursorMode::Hidden,
        );
    }

    #[inline]
    pub fn noSmooth(&mut self) {
        self.draw_params = glium::draw_parameters::DrawParameters {
            smooth: None,
            ..self.draw_params.clone()
        };
    }

    #[inline]
    pub fn smooth(&mut self) {
        self.draw_params = glium::draw_parameters::DrawParameters {
            smooth: Some(glium::draw_parameters::Smooth::Nicest),
            ..self.draw_params.clone()
        };
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }
}
