use glium::glutin;

use {Screen, Key, MouseButton};

impl<'a> Screen<'a> {
    pub fn key_press<I: Into<glutin::VirtualKeyCode>>(&mut self, button: I) -> bool {
        match self.keypressed {
            Some(k) => {
                let b: glutin::VirtualKeyCode = button.into();
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
        self.events_loop.run_forever(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::KeyboardInput { input, .. }
                        if glutin::ElementState::Pressed == input.state => {
                        match input.virtual_keycode {
                            Some(glutin::VirtualKeyCode::Space) => {
                                return glutin::ControlFlow::Break;
                            }
                            _ => return glutin::ControlFlow::Continue,
                        }
                    }
                    _ => return glutin::ControlFlow::Continue,
                }
            }
            _ => return glutin::ControlFlow::Continue,
        });
    }

    pub fn MousePress<B: Into<glutin::MouseButton>>(&mut self, button: B) -> bool {
        match self.mousepressed {
            Some(b) => {
                let btn: glutin::MouseButton = button.into();
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

    pub fn MouseRelease<B: Into<glutin::MouseButton>>(&mut self, button: B) -> bool {
        match self.mousereleased {
            Some(b) => {
                let btn: glutin::MouseButton = button.into();
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
    
    pub fn poll_events(&mut self) {
        let mut kp = None;
        let mut mp = None;
        let mut mr = None;
        let mut mpos = (-100., -100.);
        self.events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::Closed => panic!("need a smoother way to quit..."),
                        glutin::WindowEvent::KeyboardInput { input, .. }
                            if glutin::ElementState::Pressed == input.state => {
                            match input.virtual_keycode {
                                Some(b) => {
                                    kp = Some(b);
                                }
                                _ => (),
                            }
                        }
                        glutin::WindowEvent::MouseInput {
                            state: s,
                            button: b,
                            ..
                        } if glutin::ElementState::Pressed == s => {
                            mp = Some(b);
                        }
                        glutin::WindowEvent::MouseInput {
                            state: s,
                            button: b,
                            ..
                        } if glutin::ElementState::Released == s => {
                            mr = Some(b);
                        }
                        glutin::WindowEvent::MouseMoved { position, .. } => {
                            mpos = position;
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        });

        self.keypressed = kp;
        self.mousepressed = mp;
        self.mousereleased = mr;
        self.mousepos = mpos;
    }
}
