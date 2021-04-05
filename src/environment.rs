use glium;
use glium::glutin;

use errors::ProcessingErr;
use {Screen, ScreenType};

impl<'a> Screen<'a> {
    /// Change the cursor back to the default that is used when a new Screen is made.
    /// This is operating system dependent, but is usually an arrow.
    #[inline]
    pub fn reset_cursor(&mut self) -> Result<(), ProcessingErr> {
        match self.display {
            ScreenType::Window(ref d) => (*d).gl_window().window().set_cursor_visible(true),
            _ => (),
        };
        self.curr_cursor = glium::glutin::window::CursorIcon::Default;
        match self.display {
            ScreenType::Window(ref d) => (*d)
                .gl_window()
                .window()
                .set_cursor_icon(glium::glutin::window::CursorIcon::Default),
            _ => (),
        };
        Ok(())
    }

    /// Change the cursor. Possible types are "HAND", "ARROW", "CROSS", "MOVE", "TEXT",
    /// and "WAIT", all following the convention of Processing. These will probably be
    /// changed to enums in the future.
    #[inline]
    pub fn cursor(&mut self, cursor_type: &str) -> Result<(), ProcessingErr> {
        match self.display {
            ScreenType::Window(ref d) => (*d).gl_window().window().set_cursor_visible(true),
            _ => (),
        };
        if cursor_type == "HAND" {
            self.curr_cursor = glium::glutin::window::CursorIcon::Hand;
        } else if cursor_type == "ARROW" {
            self.curr_cursor = glium::glutin::window::CursorIcon::Arrow;
        } else if cursor_type == "CROSS" {
            self.curr_cursor = glium::glutin::window::CursorIcon::Crosshair;
        } else if cursor_type == "MOVE" {
            self.curr_cursor = glium::glutin::window::CursorIcon::Move;
        } else if cursor_type == "TEXT" {
            self.curr_cursor = glium::glutin::window::CursorIcon::Text;
        } else if cursor_type == "WAIT" {
            self.curr_cursor = glium::glutin::window::CursorIcon::Wait;
        }
        match self.display {
            ScreenType::Window(ref d) => {
                (*d).gl_window().window().set_cursor_icon(self.curr_cursor)
            }
            _ => (),
        };
        Ok(())
    }

    /// Test if this screen is the currently focused screen.
    #[inline]
    pub fn focused(&mut self) -> bool {
        let mut focused = false;
        self.events_loop.poll_events(|event| match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::Focused(_) => {
                    focused = true;
                }
                _ => (),
            },
            _ => (),
        });

        focused
    }

    /// How many frames have already been revealed.
    #[inline]
    pub fn frame_count(&self) -> isize {
        self.frame_count
    }

    /// What is the current framerate of the screen.
    #[inline]
    pub fn get_frame_rate(&self) -> isize {
        self.frame_rate
    }

    /// Change the framerate of the screen.
    #[inline]
    pub fn set_frame_rate(&mut self, f_rate: isize) {
        self.frame_rate = f_rate;
    }

    /// What is the height of the screen.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Disable the cursor so that it cannot be seen.
    #[inline]
    pub fn no_cursor(&mut self) -> Result<(), ProcessingErr> {
        match self.display {
            ScreenType::Window(ref d) => (*d).gl_window().window().set_cursor_visible(false),
            _ => (),
        };
        Ok(())
    }

    /// Draw shapes without antialiasing, so that individual pixels can be more readily
    /// observed.
    #[inline]
    pub fn no_smooth(&mut self) {
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
