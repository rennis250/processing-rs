use std::f32;

use glium;

use {Screen, ScreenType};
use errors::ProcessingErr;

use shapes::{Shape, ShapeVertex, IndexType, load_colors};

/// A quadrilateral is a shape with four sides that are straight lines, so it is
/// completely specified by the four points (vertices) at the corners where the
/// lines meet.
pub struct Quad<'a> {
    fill_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    stroke_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    fill_index_buffer: IndexType,
    stroke_index_buffer: IndexType,
    texture: Option<&'a glium::texture::Texture2d>,
}

impl<'a> Shape for Quad<'a> {
    fn fill_buffer(&self) -> Box<&glium::vertex::VertexBuffer<ShapeVertex>> {
        Box::new(&self.fill_buffer)
    }

    fn stroke_buffer(&self) -> Box<&glium::vertex::VertexBuffer<ShapeVertex>> {
        Box::new(&self.stroke_buffer)
    }

    fn fill_indices(&self) -> Box<&IndexType> {
        Box::new(&self.fill_index_buffer)
    }

    fn stroke_indices(&self) -> Box<&IndexType> {
        Box::new(&self.stroke_index_buffer)
    }

    fn get_texture(&self) -> Option<Box<&glium::texture::Texture2d>> {
        match self.texture {
            Some(t) => Some(Box::new(t)),
            _ => None,
        }
    }
}

impl<'a> Quad<'a> {
	/// Create a new Quad to be drawn later. It is specified by four corners at
	/// positions 1 (x1i, y1i, z1i), 2 (x2i, y2i, z2i), 3 (x3i, y3i, z3i), and
	/// 4 (x4i, y4i, z4i).
    pub fn new(
        screen: &Screen,
        x1i: &[f64],
        y1i: &[f64],
        z1i: &[f64],
        x2i: &[f64],
        y2i: &[f64],
        z2i: &[f64],
        x3i: &[f64],
        y3i: &[f64],
        z3i: &[f64],
        x4i: &[f64],
        y4i: &[f64],
        z4i: &[f64],
    ) -> Result<Self, ProcessingErr> {
        let mut x1 = x1i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y1 = y1i.iter().map(|&v| v).collect::<Vec<f64>>();
        let z1 = z1i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut x2 = x2i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y2 = y2i.iter().map(|&v| v).collect::<Vec<f64>>();
        let z2 = z2i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut x3 = x3i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y3 = y3i.iter().map(|&v| v).collect::<Vec<f64>>();
        let z3 = z3i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut x4 = x4i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y4 = y4i.iter().map(|&v| v).collect::<Vec<f64>>();
        let z4 = z4i.iter().map(|&v| v).collect::<Vec<f64>>();
        if screen.preserve_aspect_ratio {
            if screen.aspect_ratio > 1f32 {
                for i in 0..x1.len() {
                    x1[i] = x1[i] / screen.aspect_ratio as f64;
                    x2[i] = x2[i] / screen.aspect_ratio as f64;
                    x3[i] = x3[i] / screen.aspect_ratio as f64;
                    x4[i] = x4[i] / screen.aspect_ratio as f64;
                }
            } else {
                for i in 0..x1.len() {
                    y1[i] = y1[i] * screen.aspect_ratio as f64;
                    y2[i] = y2[i] * screen.aspect_ratio as f64;
                    y3[i] = y3[i] * screen.aspect_ratio as f64;
                    y4[i] = y4[i] * screen.aspect_ratio as f64;
                }
            }
        }

        let eps = f32::EPSILON;
        let mut shape = vec![];
        for c in 0..x1.len() {
            let vertex = ShapeVertex {
                position: [
                    x1[c] as f32,
                    y1[c] as f32,
                    if z1[c] == 0.0 {
                        eps * c as f32
                    } else {
                        z1[c] as f32
                    },
                ],
                color: [0.0, 0.0, 0.0, 0.0],
                texcoord: [0f32, 0.],
            };
            shape.push(vertex);
            let vertex = ShapeVertex {
                position: [
                    x2[c] as f32,
                    y2[c] as f32,
                    if z2[c] == 0.0 {
                        eps * c as f32
                    } else {
                        z2[c] as f32
                    },
                ],
                color: [0.0, 0.0, 0.0, 0.0],
                texcoord: [1f32, 0.],
            };
            shape.push(vertex);
            let vertex = ShapeVertex {
                position: [
                    x3[c] as f32,
                    y3[c] as f32,
                    if z3[c] == 0.0 {
                        eps * c as f32
                    } else {
                        z3[c] as f32
                    },
                ],
                color: [0.0, 0.0, 0.0, 0.0],
                texcoord: [1f32, 1.],
            };
            shape.push(vertex);
            let vertex = ShapeVertex {
                position: [
                    x4[c] as f32,
                    y4[c] as f32,
                    if z4[c] == 0.0 {
                        eps * c as f32
                    } else {
                        z4[c] as f32
                    },
                ],
                color: [0.0, 0.0, 0.0, 0.0],
                texcoord: [0f32, 1.],
            };
            shape.push(vertex);
        }

        let mut elements = vec![0u32; 6 * x1.len()];

        elements[0] = 0;
        elements[1] = 1;
        elements[2] = 2;
        elements[3] = 2;
        elements[4] = 3;
        elements[5] = 0;

        let mut index = 6;
        for _ in 1..x1.len() {
            elements[index] = elements[index - 6] + 4;
            elements[index + 1] = elements[(index - 6) + 1] + 4;
            elements[index + 2] = elements[(index - 6) + 2] + 4;
            elements[index + 3] = elements[(index - 6) + 3] + 4;
            elements[index + 4] = elements[(index - 6) + 4] + 4;
            elements[index + 5] = elements[(index - 6) + 5] + 4;
            index += 6;
        }

        let index_buffer = match screen.display {
            ScreenType::Window(ref d) => {
                glium::IndexBuffer::new(d, glium::index::PrimitiveType::TrianglesList, &elements)
                    .map_err(|e| ProcessingErr::IBNoCreate(e))?
            }
            ScreenType::Headless(ref d) => {
                glium::IndexBuffer::new(d, glium::index::PrimitiveType::TrianglesList, &elements)
                    .map_err(|e| ProcessingErr::IBNoCreate(e))?
            }
        };

        load_colors(&mut shape, &screen.fill_col);
        let fill_shape_buffer = match screen.display {
            ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &shape)
            	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &shape)
            	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
        };

        load_colors(&mut shape, &screen.stroke_col);
        let stroke_shape_buffer = match screen.display {
            ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &shape)
            	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &shape)
            	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
        };

        // screen.draw(fill_shape_buffer, stroke_shape_buffer, Some(index_buffer));
        Ok(Quad {
            fill_buffer: fill_shape_buffer,
            stroke_buffer: stroke_shape_buffer,
            fill_index_buffer: IndexType::Buffer { ind: index_buffer },
            stroke_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::LineLoop),
            },
            texture: None,
        })
    }

    pub fn attach_texture(&mut self, tex: &'a glium::texture::Texture2d) {
        self.texture = Some(tex);
    }
}
