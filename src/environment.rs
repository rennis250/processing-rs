use glium;
use glium::glutin;

use {Screen, ScreenType};

impl<'a> Screen<'a> {
	/// Change the cursor back to the default that is used when a new Screen is made.
	/// This is operating system dependent, but is usually an arrow.
    #[inline]
    pub fn reset_cursor(&mut self) {
        match self.display {
            ScreenType::Window(ref d) => { (*d).gl_window().set_cursor_state(glutin::CursorState::Normal); },
            _ => (),
        };
        self.currCursor = glium::glutin::MouseCursor::Default;
        match self.display {
            ScreenType::Window(ref d) => {
                (*d).gl_window().set_cursor(
                    glium::glutin::MouseCursor::Default,
                )
            }
            _ => (),
        };
    }

	/// Change the cursor. Possible types are "HAND", "ARROW", "CROSS", "MOVE", "TEXT",
	/// and "WAIT", all following the convention of Processing. These will probably be
	/// changed to enums in the future.
    #[inline]
    pub fn cursor(&mut self, cursorType: &str) {
        match self.display {
            ScreenType::Window(ref d) => { (*d).gl_window().set_cursor_state(glutin::CursorState::Normal); },
            _ => (),
        };
        if cursorType == "HAND" {
            self.currCursor = glium::glutin::MouseCursor::Hand;
        } else if cursorType == "ARROW" {
            self.currCursor = glium::glutin::MouseCursor::Arrow;
        } else if cursorType == "CROSS" {
            self.currCursor = glium::glutin::MouseCursor::Crosshair;
        } else if cursorType == "MOVE" {
            self.currCursor = glium::glutin::MouseCursor::Move;
        } else if cursorType == "TEXT" {
            self.currCursor = glium::glutin::MouseCursor::Text;
        } else if cursorType == "WAIT" {
            self.currCursor = glium::glutin::MouseCursor::Wait;
        }
        match self.display {
            ScreenType::Window(ref d) => (*d).gl_window().set_cursor(self.currCursor),
            _ => (),
        };
    }

	/// Test if this screen is the currently focused screen.
    #[inline]
    pub fn focused(&mut self) -> bool {
        let mut focused = false;
        self.events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Focused(b) => {
                        focused = true;
                    }
                    _ => (),
                }
            }
            _ => (),
        });

        focused
    }

	/// How many frames have already been revealed.
    #[inline]
    pub fn frameCount(&self) -> isize {
        self.frameCount
    }

	/// What is the current framerate of the screen.
    #[inline]
    pub fn get_frameRate(&self) -> isize {
        self.frameRate
    }

	/// Change the framerate of the screen.
    #[inline]
    pub fn set_frameRate(&mut self, fRate: isize) {
        self.frameRate = fRate;
    }

	/// What is the height of the screen.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

	/// Disable the cursor so that it cannot be seen.
    #[inline]
    pub fn noCursor(&mut self) {
        match self.display {
            ScreenType::Window(ref d) => { (*d).gl_window().set_cursor_state(glutin::CursorState::Hide); },
            _ => (),
        };
        // GLFW.SetInputMode(self, GLFW.CURSOR, GLFW.CURSOR_HIDDEN);
    }

	/// Draw shapes without antialiasing, so that individual pixels can be more readily
	/// observed.
    #[inline]
    pub fn noSmooth(&mut self) {
        self.draw_params = glium::draw_parameters::DrawParameters {
            smooth: None,
            ..self.draw_params.clone()
        };
    }

	/// Draw shapes with antialiasing for a more pleasing visual appearence.
    #[inline]
    pub fn smooth(&mut self) {
        self.draw_params = glium::draw_parameters::DrawParameters {
            smooth: Some(glium::draw_parameters::Smooth::Nicest),
            ..self.draw_params.clone()
        };
    }

	/// What is the width of the screen.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }
}
