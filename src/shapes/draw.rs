use glium;
use glium::Surface;
use glium::uniforms::Uniforms;

use shapes::{Shape, IndexType};
use shapes::mould::Mould;

use Screen;

impl<'a> Screen<'a> {
    #[inline]
    pub fn draw<S: Shape>(&mut self, shape: &S) {
        let framebuffer = &mut self.FBO;
        if let Some(tex) = shape.get_texture() {
            let prog = &self.shader_bank[1];
            let u = create_uniforms!{self, tex: *tex};
            if self.fillStuff {
                match *shape.fill_indices() {
                    &IndexType::Buffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &self.draw_params)
                            .unwrap()
                    }
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &self.draw_params)
                            .unwrap()
                    }
                }
            }
            if self.strokeStuff {
                match *shape.stroke_indices() {
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.stroke_buffer(), ib, prog, &u, &self.draw_params)
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
                            .draw(*shape.fill_buffer(), ib, prog, &u, &self.draw_params)
                            .unwrap()
                    }
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.fill_buffer(), ib, prog, &u, &self.draw_params)
                            .unwrap()
                    }
                }
            }
            if self.strokeStuff {
                match *shape.stroke_indices() {
                    &IndexType::NoBuffer { ind: ref ib } => {
                        framebuffer
                            .draw(*shape.stroke_buffer(), ib, prog, &u, &self.draw_params)
                            .unwrap()
                    }
                    _ => {}
                }
            }
        }

        // if shape.drew_points {
        // self.draw_params.smooth = Some(glium::draw_parameters::Smooth::Nicest);
        // self.drew_points = false;
        // }
    }

    #[inline]
    pub fn draw_mould<S: Shape, U: Uniforms>(&mut self, mould: &Mould<U, S>) {
        let shader = mould.get_shader();
        let shape = mould.get_shape();
        let prog = &self.shader_bank[shader.get_idx()];
        let uniforms = shader.get_uniforms();
        let framebuffer = &mut self.FBO;
        if self.fillStuff {
            match *shape.fill_indices() {
                &IndexType::Buffer { ind: ref ib } => {
                    framebuffer
                        .draw(*shape.fill_buffer(), ib, prog, uniforms, &self.draw_params)
                        .unwrap()
                }
                &IndexType::NoBuffer { ind: ref ib } => {
                    framebuffer
                        .draw(*shape.fill_buffer(), ib, prog, uniforms, &self.draw_params)
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
                            &self.draw_params,
                        )
                        .unwrap()
                }
                _ => {}
            }
        }
    }

    // #[inline]
    // pub fn draw_onto_texture<S: Shape>(&self, shape: &S, tex: &glium::texture::Texture2d) {
    //     let mut framebuffer = tex.as_surface();
    //     let mut t = self.draw_params.clone();
    //     t.depth.write = false;
    //     if let Some(tex) = shape.get_texture() {
    //         let prog = &self.shader_bank[1];
    //         let u = create_uniforms!{self, tex: *tex};
    //         if self.fillStuff {
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
    //         if self.strokeStuff {
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
    //         if self.fillStuff {
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
    //         if self.strokeStuff {
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
    //     if self.fillStuff {
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
    //     if self.strokeStuff {
    //         match *shape.stroke_indices() {
    //             &IndexType::NoBuffer { ind: ref ib } => {
    //                 framebuffer.draw(*shape.stroke_buffer(), ib, &prog, uniforms, &t)
    //                     .unwrap()
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    pub fn stroke_weight(&mut self, newWeight: f32) {
        self.draw_params.point_size = Some(newWeight);
        self.draw_params.line_width = Some(newWeight);
    }

    pub fn ellipse_mode(&mut self, mode: &str) {
        self.ellipseMode = mode.to_owned();
    }

    pub fn rect_mode(&mut self, mode: &str) {
        self.rectMode = mode.to_owned();
    }

    pub fn shape_mode(&mut self, mode: &str) {
        self.shapeMode = mode.to_owned();
    }
}
