use std::f32;

use glium;
use glium::uniforms::Uniforms;

use {Screen, ScreenType};
use errors::ProcessingErr;

use shapes::{Shape, ShapeVertex, IndexType, load_colors};

/// A line joins two points and is straight. It is completely specified by its two
/// endpoints.
pub struct Line {
    fill_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    stroke_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    fill_index_buffer: IndexType,
    stroke_index_buffer: IndexType,
}

impl Shape for Line {
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

impl Line {
	/// Create a new Line to be drawn later. It is specified by two endpoints, located
	/// at positions 1 (x1i, y1i, z1i) and 2 (x2i, y2i, z2i).
    #[inline]
    pub fn new(
        screen: &Screen,
        x1i: &[f64],
        y1i: &[f64],
        z1i: &[f64],
        x2i: &[f64],
        y2i: &[f64],
        z2i: &[f64],
    ) -> Result<Self, ProcessingErr> {
        let mut x1 = x1i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y1 = y1i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut z1 = z1i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut x2 = x2i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut y2 = y2i.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut z2 = z2i.iter().map(|&v| v).collect::<Vec<f64>>();
        if screen.preserveAspectRatio {
            if screen.aspectRatio > 1f32 {
                for i in 0..x1.len() {
                    x1[i] = x1[i] / screen.aspectRatio as f64;
                    x2[i] = x2[i] / screen.aspectRatio as f64;
                }
            } else {
                for i in 0..x1.len() {
                    y1[i] = y1[i] * screen.aspectRatio as f64;
                    y2[i] = y2[i] * screen.aspectRatio as f64;
                }
            }
        }

        if screen.strokeStuff {
            let eps = f32::EPSILON;
            let mut shape = vec![];
            for (i, _) in x1.iter().enumerate() {
                let vertex = ShapeVertex {
                    position: [
                        x1[i] as f32,
                        y1[i] as f32,
                        // if z1[c] == 0.0 {
                        eps * i as f32 // } else {
                                       // z1[c] as f32
                                       // },
                    ],
                    color: [0.0, 0.0, 0.0, 0.0],
                    texcoord: [0f32, 0.],
                };
                shape.push(vertex);
                let vertex = ShapeVertex {
                    position: [
                        x2[i] as f32,
                        y2[i] as f32,
                        // if z1[c] == 0.0 {
                        eps * i as f32 // } else {
                                       // z1[c] as f32
                                       // },
                    ],
                    color: [0.0, 0.0, 0.0, 0.0],
                    texcoord: [0f32, 0.],
                };
                shape.push(vertex);
            }

            // lines need a custom load_colors implementation
            if screen.strokeCol.len() == 4 {
                for x in (0..shape.len()).filter(|x| x % 2 == 0) {
                    shape[x].color = [
                        screen.strokeCol[0],
                        screen.strokeCol[1],
                        screen.strokeCol[2],
                        screen.strokeCol[3],
                    ];
                    shape[x + 1].color = [
                        screen.strokeCol[0],
                        screen.strokeCol[1],
                        screen.strokeCol[2],
                        screen.strokeCol[3],
                    ];
                }
            } else {
                for c in 0..screen.strokeCol.len() / 4 {
                    let idx = c * 4;
                    // for x in (c * shapeStride..(c + 1) * shapeStride).filter(|x| x % 4 == 0) {
                    // $buffer[x].color = [
                    // $colorMat[idx],
                    // $colorMat[idx + 1],
                    // $colorMat[idx + 2],
                    // $colorMat[idx + 3],
                    // ];
                    // }
                }
            }

            let fill_shape_buffer = match screen.display {
                ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
                ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            };
            let stroke_shape_buffer = match screen.display {
                ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
                ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            };

            return Ok(Line {
                fill_buffer: fill_shape_buffer,
                stroke_buffer: stroke_shape_buffer,
                fill_index_buffer: IndexType::NoBuffer {
                    ind: glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
                },
                stroke_index_buffer: IndexType::NoBuffer {
                    ind: glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
                },
            });
        }

        return Ok(Line {
            fill_buffer: match screen.display {
                ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &vec![])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
                ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &vec![])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            },
            stroke_buffer: match screen.display {
                ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &vec![])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
                ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &vec![])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            },
            fill_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
            },
            stroke_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
            },
        });
    }
}
