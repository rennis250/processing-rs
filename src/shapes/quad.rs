use std::f32;

use glium;

use Screen;

use shapes::{Shape, ShapeVertex, IndexType, load_colors};

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
    ) -> Self {
        let mut x1 = x1i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y1 = y1i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut z1 = z1i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut x2 = x2i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y2 = y2i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut z2 = z2i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut x3 = x3i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y3 = y3i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut z3 = z3i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut x4 = x4i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y4 = y4i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut z4 = z4i.iter().map(|&v| v).collect::<Vec<f64>>();
        if screen.preserveAspectRatio {
            if screen.aspectRatio > 1f32 {
                for i in 0..x1.len() {
                    x1[i] = x1[i] / screen.aspectRatio as f64;
                    x2[i] = x2[i] / screen.aspectRatio as f64;
                    x3[i] = x3[i] / screen.aspectRatio as f64;
                    x4[i] = x4[i] / screen.aspectRatio as f64;
                }
            } else {
                for i in 0..x1.len() {
                    y1[i] = y1[i] * screen.aspectRatio as f64;
                    y2[i] = y2[i] * screen.aspectRatio as f64;
                    y3[i] = y3[i] * screen.aspectRatio as f64;
                    y4[i] = y4[i] * screen.aspectRatio as f64;
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
                    // if z1[c] == 0.0 {
                    eps * c as f32 // } else {
                                   // z1[c] as f32
                                   // },
                ],
                color: [0.0, 0.0, 0.0, 0.0],
                texcoord: [0f32, 0.],
            };
            shape.push(vertex);
            let vertex = ShapeVertex {
                position: [
                    x2[c] as f32,
                    y2[c] as f32,
                    // if z1[c] == 0.0 {
                    eps * c as f32 // } else {
                                   // z1[c] as f32
                                   // },
                ],
                color: [0.0, 0.0, 0.0, 0.0],
                texcoord: [1f32, 0.],
            };
            shape.push(vertex);
            let vertex = ShapeVertex {
                position: [
                    x3[c] as f32,
                    y3[c] as f32,
                    // if z1[c] == 0.0 {
                    eps * c as f32 // } else {
                                   // z1[c] as f32
                                   // },
                ],
                color: [0.0, 0.0, 0.0, 0.0],
                texcoord: [1f32, 1.],
            };
            shape.push(vertex);
            let vertex = ShapeVertex {
                position: [
                    x4[c] as f32,
                    y4[c] as f32,
                    // if z1[c] == 0.0 {
                    eps * c as f32 // } else {
                                   // z1[c] as f32
                                   // },
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
        for x in 1..x1.len() {
            elements[index] = elements[index - 6] + 4;
            elements[index + 1] = elements[(index - 6) + 1] + 4;
            elements[index + 2] = elements[(index - 6) + 2] + 4;
            elements[index + 3] = elements[(index - 6) + 3] + 4;
            elements[index + 4] = elements[(index - 6) + 4] + 4;
            elements[index + 5] = elements[(index - 6) + 5] + 4;
            index += 6;
        }

        let index_buffer = glium::IndexBuffer::new(
            &screen.display,
            glium::index::PrimitiveType::TrianglesList,
            &elements,
        ).unwrap();

        load_colors(&mut shape, &screen.fillCol);
        let fill_shape_buffer = glium::VertexBuffer::new(&screen.display, &shape).unwrap();

        load_colors(&mut shape, &screen.strokeCol);
        let stroke_shape_buffer = glium::VertexBuffer::new(&screen.display, &shape).unwrap();

        // screen.draw(fill_shape_buffer, stroke_shape_buffer, Some(index_buffer));
        Quad {
            fill_buffer: fill_shape_buffer,
            stroke_buffer: stroke_shape_buffer,
            fill_index_buffer: IndexType::Buffer { ind: index_buffer },
            stroke_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::LineLoop),
            },
            texture: None,
        }
    }

    pub fn attach_texture(mut self, tex: &'a glium::texture::Texture2d) -> Self {
        self.texture = Some(tex);
        self
    }
}
