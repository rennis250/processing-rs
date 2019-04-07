use glium::Surface;
use glium::uniforms::{UniformValue, AsUniformValue};

use shapes::{Shape, IndexType};
use shapes::mould::Mould;
use errors::ProcessingErr;

use Screen;

impl<'a> Screen<'a> {
	/// Take a given shape and draw it onto the screen. Since the shape's properties
	/// were precomputed and its buffers were already uploaded to the GPU, drawing
	/// many shapes should be faster than in a standard Processing environment.
    #[inline]
    pub fn draw<S: Shape>(&mut self, shape: &S) -> Result<(), ProcessingErr> {
        let framebuffer = &mut self.fbo;
        if let Some(tex) = shape.get_texture() {
        	let mut u;
        	if self.using_alternate_shader {
        		if let Some(ref unis) = self.uniforms {
        			u = (*unis).clone();
				} else {
            		u = create_uniforms!{self, tex: (*tex).as_uniform_value()};
				}
				if let Some(ref texs) = self.texture_list {
					for (_, tex_id) in texs {
						unsafe {
							gl::ActiveTexture(gl::TEXTURE0);
							gl::BindTexture(gl::TEXTURE_2D, *tex_id);
						}
					}
				}
			} else {
            	u = create_uniforms!{self, tex: (*tex).as_uniform_value()};
			};
        	let prog_idx = if self.using_alternate_shader {
        		self.curr_shader
        	} else {
        		1
        	};
            let prog = &self.shader_bank[prog_idx];
            if self.fill_stuff {
                match *shape.fill_indices() {
                    &IndexType::Buffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &self.draw_params)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &self.draw_params)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                }
            };
            if self.stroke_stuff {
                match *shape.stroke_indices() {
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.stroke_buffer(), ib, prog, &u, &self.draw_params)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                    _ => {}
                }
            };
        } else {
        	let mut u;
        	if self.using_alternate_shader {
        		if let Some(ref unis) = self.uniforms {
        			u = (*unis).clone();
				} else {
            		u = create_uniforms!{self};
				}
				if let Some(ref texs) = self.texture_list {
					for (_, tex_id) in texs {
						unsafe {
							gl::ActiveTexture(gl::TEXTURE0);
							gl::BindTexture(gl::TEXTURE_2D, *tex_id);
						}
					}
				}
			} else {
            	u = create_uniforms!{self};
			};
        	let prog_idx = if self.using_alternate_shader {
        		self.curr_shader
        	} else {
        		0
        	};
            let prog = &self.shader_bank[prog_idx];
            if self.fill_stuff {
                match *shape.fill_indices() {
                    &IndexType::Buffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &self.draw_params)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &self.draw_params)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                }
            };
            if self.stroke_stuff {
                match *shape.stroke_indices() {
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.stroke_buffer(), ib, prog, &u, &self.draw_params)
                            .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                    }
                    _ => {}
                }
            };
        }
        
        Ok(())

        // if shape.drew_points {
        // self.draw_params.smooth = Some(glium::draw_parameters::Smooth::Nicest);
        // self.drew_points = false;
        // }
    }

	/// The same as screen.draw(), except now a Mould will be drawn to the screen.
	/// A Mould is just a shape that has been paired with a given shader which
	/// alters how it is typically drawn to the screen. This allows one to have
	/// a shader affect only one object instead of the whole drawing process.
	/// The concept is borrowed from libCinder.
    #[inline]
    pub fn draw_mould<S: Shape>(&mut self, mould: &Mould<S>) -> Result<(), ProcessingErr> {
        let shader = mould.get_shader();
        let shape = mould.get_shape();
        let prog = &self.shader_bank[shader.get_idx()];
        let uniforms = shader.get_uniforms();
        let framebuffer = &mut self.fbo;
        if self.fill_stuff {
            match *shape.fill_indices() {
                &IndexType::Buffer { ind: ref ib } => {
                    framebuffer
                        .draw(*shape.fill_buffer(), ib, prog, &uniforms, &self.draw_params)
                        .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                }
                &IndexType::NoBuffer { ind: ref ib } => {
                    framebuffer
                        .draw(*shape.fill_buffer(), ib, prog, &uniforms, &self.draw_params)
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
                            &self.draw_params,
                        )
                        .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                }
                _ => {}
            }
        };
        
        Ok(())
    }

    pub fn stroke_weight(&mut self, new_weight: f32) {
        self.draw_params.point_size = Some(new_weight);
        self.draw_params.line_width = Some(new_weight);
    }

    pub fn ellipse_mode(&mut self, mode: &str) {
        self.ellipse_mode = mode.to_owned();
    }

    pub fn rect_mode(&mut self, mode: &str) {
        self.rect_mode = mode.to_owned();
    }

    pub fn shape_mode(&mut self, mode: &str) {
        self.shape_mode = mode.to_owned();
    }
}
