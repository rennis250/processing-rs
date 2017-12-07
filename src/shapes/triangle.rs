use std::f32;

use glium;
use glium::uniforms::Uniforms;

use {Screen, ScreenType};

use shapes::{Shape, ShapeVertex, IndexType, load_colors};

pub struct Triangle {
    fill_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    stroke_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    fill_index_buffer: IndexType,
    stroke_index_buffer: IndexType,
}

impl Shape for Triangle {
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
        None
    }
}

impl Triangle {
    #[inline]
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
        if screen.preserveAspectRatio {
            if screen.aspectRatio > 1f32 {
                for i in 0..x1.len() {
                    x1[i] = x1[i] / screen.aspectRatio as f64;
                    x2[i] = x2[i] / screen.aspectRatio as f64;
                    x3[i] = x3[i] / screen.aspectRatio as f64;
                }
            } else {
                for i in 0..x1.len() {
                    y1[i] = y1[i] * screen.aspectRatio as f64;
                    y2[i] = y2[i] * screen.aspectRatio as f64;
                    y3[i] = y3[i] * screen.aspectRatio as f64;
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
        }

        // if screen.drawTexture {
        // texcoords
        // texData = make([]float32, numSlices*4*len(xc))
        // texData[8:vertexStride:end] = 0
        // texData[9:vertexStride:end] = 0

        // texData[17:vertexStride:end] = 1
        // texData[18:vertexStride:end] = 0

        // texData[26:vertexStride:end] = 1
        // texData[27:vertexStride:end] = 1
        //
        // gl.BindBuffer(gl.ARRAY_BUFFER, GLobjs.TEXVBOs[GLobjs.TexInd])
        // gl.BufferData(gl.ARRAY_BUFFER, len(texData)*4, gl.Ptr(texData), gl.STATIC_DRAW)
        // }

        load_colors(&mut shape, &screen.fillCol);
        let fill_shape_buffer = match screen.display {
            ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &shape).unwrap(),
            ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &shape).unwrap(),
        };

        load_colors(&mut shape, &screen.strokeCol);
        let stroke_shape_buffer = match screen.display {
            ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &shape).unwrap(),
            ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &shape).unwrap(),
        };

        // screen.draw(fill_shape_buffer, stroke_shape_buffer, Some(index_buffer));
        Triangle {
            fill_buffer: fill_shape_buffer,
            stroke_buffer: stroke_shape_buffer,
            fill_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            },
            stroke_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::LineLoop),
            },
        }
    }
}
