use glium;
use glium::Surface;
use glium::uniforms::{UniformValue, AsUniformValue};

use shapes::{Shape, IndexType};
use shapes::mould::Mould;
use {Screen, ScreenType};
use errors::ProcessingErr;

impl<'a> Screen<'a> {
	/// Create a new framebuffer from a texture. This is necessary if you want
	/// to draw to a texture, which is useful in combination with shaders that
	/// perform some post-processing on the texture. In the case that you will also
	/// use a shader for post-processing, you will also need to use a Mould.
    #[inline]
    pub fn framebuffer(
        &self,
        fbtex: &'a glium::texture::Texture2d,
    ) -> Result<glium::framebuffer::SimpleFrameBuffer, ProcessingErr> {
        match self.display {
            ScreenType::Window(ref d) => {
                glium::framebuffer::SimpleFrameBuffer::new(d, fbtex).map_err(|e| ProcessingErr::FBNoCreate(e))
            }
            ScreenType::Headless(ref d) => {
                glium::framebuffer::SimpleFrameBuffer::new(d, fbtex).map_err(|e| ProcessingErr::FBNoCreate(e))
            } 
        }
    }

	/// Erase the framebuffer and set it to the given color.
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

	/// Draw the given shape onto the given framebuffer.
    #[inline]
    pub fn draw_onto_framebuffer<S: Shape>(
        &self,
        shape: &S,
        framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) -> Result<(), ProcessingErr> {
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
            let u = create_uniforms!{self, tex: (*tex).as_uniform_value()};
            if self.fill_stuff {
                match *shape.fill_indices() {
                    &IndexType::Buffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &t)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &t)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                }
            };
            if self.stroke_stuff {
                match *shape.stroke_indices() {
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.stroke_buffer(), ib, prog, &u, &t)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                    _ => {}
                }
            };
        } else {
            let prog = &self.shader_bank[0];
            let u = create_uniforms!{self};
            if self.fill_stuff {
                match *shape.fill_indices() {
                    &IndexType::Buffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &t)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &t)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                }
            };
            if self.stroke_stuff {
                match *shape.stroke_indices() {
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.stroke_buffer(), ib, prog, &u, &t)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                    _ => {}
                }
            };
        }
        
        Ok(())

        // if self.drew_points {
        // self.draw_params.smooth = Some(glium::draw_parameters::Smooth::Nicest);
        // self.drew_points = false;
        // }
    }

	/// Draw a Mould (i.e., a shape plus a custom shader) onto the given framebuffer.
    #[inline]
    pub fn draw_mould_onto_framebuffer<S: Shape>(
        &self,
        mould: &Mould<S>,
        framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) -> Result<(), ProcessingErr> {
        let shader = mould.get_shader();
        let shape = mould.get_shape();
        let prog = &self.shader_bank[shader.get_idx()];
        let uniforms = shader.get_uniforms();
        // let t = glium::draw_parameters::DrawParameters {
        //     depth: glium::Depth {
        //         write: false,
        //         test: glium::DepthTest::Overwrite,
        //         ..Default::default()
        //     },
        //     multisampling: false,
        //     smooth: None,
        //     ..Default::default()
        // };
        if self.fill_stuff {
            match *shape.fill_indices() {
                &IndexType::Buffer { ind: ref ib } => {
                    framebuffer
                        .draw(
                            *shape.fill_buffer(),
                            ib,
                            prog,
                            &uniforms,
                            &Default::default(),
                        )
                        .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                }
                &IndexType::NoBuffer { ind: ref ib } => {
                    framebuffer
                        .draw(
                            *shape.fill_buffer(),
                            ib,
                            prog,
                            &uniforms,
                            &Default::default(),
                        )
                        .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                }
            }
        };
        if self.stroke_stuff {
            match *shape.stroke_indices() {
                &IndexType::NoBuffer { ind: ref ib } => {
                    framebuffer
                        .draw(
                            *shape.stroke_buffer(),
                            ib,
                            &prog,
                            &uniforms,
                            &Default::default(),
                        )
                        .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                }
                _ => {}
            }
        };
        
        Ok(())
    }
}
