use std::f32;

use glium;
use glium::uniforms::Uniforms;

use {Screen, ScreenType};

use shapes::{Shape, ShapeVertex, IndexType, load_colors};

/// A rectangle is a quadrilateral whose sides meet at perpendicular angles. It is
/// specified by its width and height.
pub struct Rect<'a> {
    fill_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    stroke_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    fill_index_buffer: IndexType,
    stroke_index_buffer: IndexType,
    texture: Option<&'a glium::texture::Texture2d>,
}

impl<'a> Shape for Rect<'a> {
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


impl<'a> Rect<'a> {
	/// Create a new Rect to be drawn later. It is specified by the location of its
	/// top-left corner (xtoplefti, ytoplefti, ztoplefti) and its width (widthi) and
	/// height (heighti).
    #[inline]
    pub fn new(
        screen: &Screen,
        xtoplefti: &[f64],
        ytoplefti: &[f64],
        ztoplefti: &[f64],
        widthi: &[f64],
        heighti: &[f64],
    ) -> Self {
        let mut xtopleft = xtoplefti.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut ytopleft = ytoplefti.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut ztopleft = ztoplefti.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut width = widthi.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut height = heighti.iter().map(|&v| v).collect::<Vec<f64>>();

        if screen.preserveAspectRatio && screen.aspectRatio != 1f32 {
            if screen.aspectRatio > 1f32 {
                for i in 0..width.len() {
                    xtopleft[i] = xtopleft[i] / screen.aspectRatio as f64;
                    width[i] = width[i] / screen.aspectRatio as f64;
                }
            } else {
                for i in 0..width.len() {
                    ytopleft[i] = ytopleft[i] * screen.aspectRatio as f64;
                    height[i] = height[i] * screen.aspectRatio as f64;
                }
            }
        }

        if screen.rectMode == "CENTER" {
            for i in 0..xtopleft.len() {
                xtopleft[i] = xtopleft[i] - width[i] / 2.0;
                ytopleft[i] = ytopleft[i] - height[i] / 2.0;
            }
        } else if screen.rectMode == "RADIUS" {
            //  xtopleft = xtopleft .- width
            //  ytopleft = ytopleft .- height
            //  width = 2 .* width
            //  height = 2 .* height
        } else if screen.rectMode == "CORNERS" {
            //  width = width .- xtopleft
            //  height = height .- ytopleft
        }

        let x1 = xtopleft.clone();
        let y1 = ytopleft.clone();
        let y2 = ytopleft.clone();
        let x4 = xtopleft.clone();
        let mut x2: Vec<f64> = Vec::with_capacity(xtopleft.len());
        let mut x3: Vec<f64> = Vec::with_capacity(xtopleft.len());
        let mut y3: Vec<f64> = Vec::with_capacity(xtopleft.len());
        let mut y4: Vec<f64> = Vec::with_capacity(xtopleft.len());
        for i in 0..xtopleft.len() {
            x2.push(xtopleft[i] + width[i]);
            x3.push(xtopleft[i] + width[i]);
            y3.push(ytopleft[i] - height[i]);
            y4.push(ytopleft[i] - height[i]);
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
                texcoord: [0f32, 1.],
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
                texcoord: [1f32, 1.],
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
                texcoord: [1f32, 0.],
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
                texcoord: [0f32, 0.],
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

        let index_buffer = match screen.display {
            ScreenType::Window(ref d) => {
                glium::IndexBuffer::new(d, glium::index::PrimitiveType::TrianglesList, &elements)
                    .unwrap()
            }
            ScreenType::Headless(ref d) => {
                glium::IndexBuffer::new(d, glium::index::PrimitiveType::TrianglesList, &elements)
                    .unwrap()
            }
        };

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
        Rect {
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
