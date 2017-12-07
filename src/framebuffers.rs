use glium;
use glium::Surface;
use glium::uniforms::Uniforms;

use shapes::{Shape, IndexType};
use shapes::mould::Mould;
use {Screen, ScreenType};

impl<'a> Screen<'a> {
    #[inline]
    pub fn framebuffer(
        &self,
        fbtex: &'a glium::texture::Texture2d,
    ) -> glium::framebuffer::SimpleFrameBuffer {
        match self.display {
            ScreenType::Window(ref d) => {
                glium::framebuffer::SimpleFrameBuffer::new(d, fbtex).unwrap()
            }
            ScreenType::Headless(ref d) => {
                glium::framebuffer::SimpleFrameBuffer::new(d, fbtex).unwrap()
            } 
        }
    }

    #[inline]
    pub fn clear_framebuffer(
        &self,
        framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        framebuffer.clear_color_srgb(r, g, b, a);
    }

    #[inline]
    pub fn draw_onto_framebuffer<S: Shape>(
        &self,
        shape: &S,
        framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        // see if there is a decent way to use this
        // let mut t = self.draw_params.clone();
        // t.depth.write = false;
        let t = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                write: false,
                test: glium::DepthTest::Overwrite,
                ..Default::default()
            },
            multisampling: false,
            smooth: None,
            ..Default::default()
        };
        if let Some(tex) = shape.get_texture() {
            let prog = &self.shader_bank[1];
            let u = create_uniforms!{self, tex: *tex};
            if self.fillStuff {
                match *shape.fill_indices() {
                    &IndexType::Buffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &t)
                            .unwrap()
                    }
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &t)
                            .unwrap()
                    }
                }
            }
            if self.strokeStuff {
                match *shape.stroke_indices() {
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.stroke_buffer(), ib, prog, &u, &t)
                            .unwrap()
                    }
                    _ => {}
                }
            }
        } else {
            let prog = &self.shader_bank[0];
            let u = create_uniforms!{self};
            if self.fillStuff {
                match *shape.fill_indices() {
                    &IndexType::Buffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &t)
                            .unwrap()
                    }
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &t)
                            .unwrap()
                    }
                }
            }
            if self.strokeStuff {
                match *shape.stroke_indices() {
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.stroke_buffer(), ib, prog, &u, &t)
                            .unwrap()
                    }
                    _ => {}
                }
            }
        }

        // if self.drew_points {
        // self.draw_params.smooth = Some(glium::draw_parameters::Smooth::Nicest);
        // self.drew_points = false;
        // }
    }

    #[inline]
    pub fn draw_mould_onto_framebuffer<S: Shape, U: Uniforms>(
        &self,
        mould: &Mould<U, S>,
        framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let shader = mould.get_shader();
        let shape = mould.get_shape();
        let prog = &self.shader_bank[shader.get_idx()];
        let uniforms = shader.get_uniforms();
        // let mut t = self.draw_params.clone();
        // t.depth.write = false;
        let t = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                write: false,
                test: glium::DepthTest::Overwrite,
                ..Default::default()
            },
            multisampling: false,
            smooth: None,
            ..Default::default()
        };
        if self.fillStuff {
            match *shape.fill_indices() {
                &IndexType::Buffer { ind: ref ib } => {
                    framebuffer
                        .draw(
                            *shape.fill_buffer(),
                            ib,
                            prog,
                            uniforms,
                            &Default::default(),
                        )
                        .unwrap()
                }
                &IndexType::NoBuffer { ind: ref ib } => {
                    framebuffer
                        .draw(
                            *shape.fill_buffer(),
                            ib,
                            prog,
                            uniforms,
                            &Default::default(),
                        )
                        .unwrap()
                }
            }
        }
        if self.strokeStuff {
            match *shape.stroke_indices() {
                &IndexType::NoBuffer { ind: ref ib } => {
                    framebuffer
                        .draw(
                            *shape.stroke_buffer(),
                            ib,
                            &prog,
                            uniforms,
                            &Default::default(),
                        )
                        .unwrap()
                }
                _ => {}
            }
        }
    }
}
