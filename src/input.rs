use glium::glutin;

use Screen;

impl<'a> Screen<'a> {
    /// Check if the given key was pressed since the last call to screen.release()
    /// or screen.poll_events().
    pub fn key_press<I: Into<glutin::event::VirtualKeyCode>>(&mut self, button: I) -> bool {
        match self.keypressed {
            Some(k) => {
                let b: glutin::event::VirtualKeyCode = button.into();
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
    // pub fn space_wait(&mut self) {
    //     self.events_loop.run_forever(|event| match event {
    //         glutin::event::Event::WindowEvent { event, .. } => match event {
    //             glutin::event::WindowEvent::KeyboardInput { input, .. }
    //                 if glutin::event::ElementState::Pressed == input.state =>
    //             {
    //                 match input.virtual_keycode {
    //                     Some(glutin::event::VirtualKeyCode::Space) => {
    //                         return glutin::event_loop::ControlFlow::Break;
    //                     }
    //                     _ => return glutin::event_loop::ControlFlow::Continue,
    //                 }
    //             }
    //             _ => return glutin::event_loop::ControlFlow::Continue,
    //         },
    //         _ => return glutin::event_loop::ControlFlow::Continue,
    //     });
    // }

    /// Check if the given mouse button was pressed since the last call to
    /// screen.reveal() or screen.poll_events().
    pub fn mouse_press<B: Into<glutin::event::MouseButton>>(&mut self, button: B) -> bool {
        match self.mousepressed {
            Some(b) => {
                let btn: glutin::event::MouseButton = button.into();
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
    pub fn mouse_release<B: Into<glutin::event::MouseButton>>(&mut self, button: B) -> bool {
        match self.mousereleased {
            Some(b) => {
                let btn: glutin::event::MouseButton = button.into();
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
    pub fn mouse_x(&mut self) -> f64 {
        self.mousepos.x
    }

    /// What was the y-coordinate of the mouse at the last call to screen.reveal()
    /// or screen.poll_events().
    pub fn mouse_y(&mut self) -> f64 {
        self.mousepos.y
    }

    /// Rather than wait for screen.reveal() to be called to see if any events occurred,
    /// you can manually check for events with this function. Once it has been called,
    /// you can then check for specific events using the other functions in this
    pub fn poll_events(&mut self) {
        let mut kp = None;
        let mut mp = None;
        let mut mr = None;
        let mut mpos = glutin::dpi::PhysicalPosition::new(-100., -100.);
        self.events_loop.poll_events(|event| match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    panic!("need a smoother way to quit...")
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. }
                    if glutin::event::ElementState::Pressed == input.state =>
                {
                    match input.virtual_keycode {
                        Some(b) => {
                            kp = Some(b);
                        }
                        _ => (),
                    }
                }
                glutin::event::WindowEvent::MouseInput {
                    state: s,
                    button: b,
                    ..
                } if glutin::event::ElementState::Pressed == s => {
                    mp = Some(b);
                }
                glutin::event::WindowEvent::MouseInput {
                    state: s,
                    button: b,
                    ..
                } if glutin::event::ElementState::Released == s => {
                    mr = Some(b);
                }
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    mpos = position;
                }
                _ => (),
            },
            _ => (),
        });

        self.keypressed = kp;
        self.mousepressed = mp;
        self.mousereleased = mr;
        self.mousepos = mpos;
    }
}
