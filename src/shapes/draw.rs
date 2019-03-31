use glium::Surface;
use glium::uniforms::Uniforms;

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
            let prog = &self.shader_bank[1];
            let u = create_uniforms!{self, tex: *tex};
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
            let prog = &self.shader_bank[0];
            let u = create_uniforms!{self};
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
    pub fn draw_mould<S: Shape, U: Uniforms>(&mut self, mould: &Mould<U, S>) -> Result<(), ProcessingErr> {
        let shader = mould.get_shader();
        let shape = mould.get_shape();
        let prog = &self.shader_bank[shader.get_idx()];
        let uniforms = shader.get_uniforms();
        let framebuffer = &mut self.fbo;
        if self.fill_stuff {
            match *shape.fill_indices() {
                &IndexType::Buffer { ind: ref ib } => {
                    framebuffer
                        .draw(*shape.fill_buffer(), ib, prog, uniforms, &self.draw_params)
                        .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                }
                &IndexType::NoBuffer { ind: ref ib } => {
                    framebuffer
                        .draw(*shape.fill_buffer(), ib, prog, uniforms, &self.draw_params)
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
                            uniforms,
                            &self.draw_params,
                        )
                        .map_err(|e| ProcessingErr::FBDrawFailed(e))?
                }
                _ => {}
            }
        };
        
        Ok(())
    }

    // #[inline]
    // pub fn draw_onto_texture<S: Shape>(&self, shape: &S, tex: &glium::texture::Texture2d) {
    //     let mut framebuffer = tex.as_surface();
    //     let mut t = self.draw_params.clone();
    //     t.depth.write = false;
    //     if let Some(tex) = shape.get_texture() {
    //         let prog = &self.shader_bank[1];
    //         let u = create_uniforms!{self, tex: *tex};
    //         if self.fill_stuff {
    //             match *shape.fill_indices() {
    //                 &IndexType::Buffer { ind: ref ib } => {
    //                     framebuffer.draw(*shape.fill_buffer(), ib, prog, &u, &t)
    //                         .unwrap()
    //                 }
    //                 &IndexType::NoBuffer { ind: ref ib } => {
    //                     framebuffer.draw(*shape.fill_buffer(), ib, prog, &u, &t)
    //                         .unwrap()
    //                 }
    //             }
    //         }
    //         if self.stroke_stuff {
    //             match *shape.stroke_indices() {
    //                 &IndexType::NoBuffer { ind: ref ib } => {
    //                     framebuffer.draw(*shape.stroke_buffer(), ib, prog, &u, &t)
    //                         .unwrap()
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     } else {
    //         let prog = &self.shader_bank[0];
    //         let u = create_uniforms!{self};
    //         if self.fill_stuff {
    //             match *shape.fill_indices() {
    //                 &IndexType::Buffer { ind: ref ib } => {
    //                     framebuffer.draw(*shape.fill_buffer(), ib, prog, &u, &t)
    //                         .unwrap()
    //                 }
    //                 &IndexType::NoBuffer { ind: ref ib } => {
    //                     framebuffer.draw(*shape.fill_buffer(), ib, prog, &u, &t)
    //                         .unwrap()
    //                 }
    //             }
    //         }
    //         if self.stroke_stuff {
    //             match *shape.stroke_indices() {
    //                 &IndexType::NoBuffer { ind: ref ib } => {
    //                     framebuffer.draw(*shape.stroke_buffer(), ib, prog, &u, &t)
    //                         .unwrap()
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }

    //     // if self.drew_points {
    //     // self.draw_params.smooth = Some(glium::draw_parameters::Smooth::Nicest);
    //     // self.drew_points = false;
    //     // }
    // }

    // #[inline]
    // pub fn draw_mould_onto_texture<S: Shape, U: Uniforms>(&self,
    //                                                       mould: &Mould<U, S>,
    //                                                       tex: &glium::texture::Texture2d) {
    //     let shader = mould.get_shader();
    //     let shape = mould.get_shape();
    //     let prog = &self.shader_bank[shader.get_idx()];
    //     let uniforms = shader.get_uniforms();
    //     let mut framebuffer = tex.as_surface();
    //     let mut t = self.draw_params.clone();
    //     t.depth.write = false;
    //     if self.fill_stuff {
    //         match *shape.fill_indices() {
    //             &IndexType::Buffer { ind: ref ib } => {
    //                 framebuffer.draw(*shape.fill_buffer(), ib, prog, uniforms, &t)
    //                     .unwrap()
    //             }
    //             &IndexType::NoBuffer { ind: ref ib } => {
    //                 framebuffer.draw(*shape.fill_buffer(), ib, prog, uniforms, &t)
    //                     .unwrap()
    //             }
    //         }
    //     }
    //     if self.stroke_stuff {
    //         match *shape.stroke_indices() {
    //             &IndexType::NoBuffer { ind: ref ib } => {
    //                 framebuffer.draw(*shape.stroke_buffer(), ib, &prog, uniforms, &t)
    //                     .unwrap()
    //             }
    //             _ => {}
    //         }
    //     }
    // }

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
