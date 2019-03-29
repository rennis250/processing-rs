use std::f32;

use glium;

use {Screen, ScreenType};
use errors::ProcessingErr;

use shapes::{Shape, ShapeVertex, IndexType, load_colors};

/// An arc is essentially just part of an ellipse, so it traces out part of the contour
/// by starting at a certain position (specified in radians) and ending at a certain
/// position (also specified in radians).
pub struct Arc {
    fill_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    stroke_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    fill_index_buffer: IndexType,
    stroke_index_buffer: IndexType,
}


impl Shape for Arc {
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

impl Arc {
	/// Create a new Arc object to be drawn later. It has a center position
	/// (xci, yci, zci), a width (wi), a height (hi), a starting point for
	/// drawing (starti, in radians), and a stopping point for drawing
	/// (stopi, in radians).
    #[inline]
    pub fn new(
        screen: &Screen,
        xci: &[f64],
        yci: &[f64],
        zci: &[f64],
        wi: &[f64],
        hi: &[f64],
        starti: &[f64],
        stopi: &[f64],
    ) -> Result<Self, ProcessingErr> {
        let mut xc = xci.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut yc = yci.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut zc = zci.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut w = wi.iter().map(|&v| v).collect::<Vec<f64>>();
        let mut h = hi.iter().map(|&v| v).collect::<Vec<f64>>();
        let start = starti.iter().map(|&v| v).collect::<Vec<f64>>();
        let stop = stopi.iter().map(|&v| v).collect::<Vec<f64>>();
        if screen.preserveAspectRatio && screen.aspectRatio != 1f32 {
            if screen.aspectRatio > 1f32 {
                for i in 0..w.len() {
                    xc[i] = xc[i] / screen.aspectRatio as f64;
                    w[i] = w[i] / screen.aspectRatio as f64;
                }
            } else {
                for i in 0..w.len() {
                    yc[i] = yc[i] * screen.aspectRatio as f64;
                    h[i] = h[i] * screen.aspectRatio as f64;
                }
            }
        }

        if screen.ellipseMode == "CENTER" {
            for i in 0..w.len() {
                w[i] = w[i] / 2.0;
                h[i] = h[i] / 2.0;
            }
        } else if screen.ellipseMode == "CORNER" {
            //  w = w ./ 2
            //  h = h ./ 2
            //  xc = xc .+ w
            //  yc = yc .+ h
        } else if screen.ellipseMode == "CORNERS" {
            //  xc = (w .- xc)./2
            //  yc = (h .- yc)./2
            //  w = w ./ 2
            //  h = h ./ 2
        }

        let numSlices = 200.0 + 2.0;
        let mut cs: Vec<Vec<f64>> = Vec::with_capacity(xc.len());
        let mut ss: Vec<Vec<f64>> = Vec::with_capacity(xc.len());
        for j in 0..xc.len() {
            cs.push(Vec::with_capacity(numSlices as usize - 1));
            ss.push(Vec::with_capacity(numSlices as usize - 1));
            let step = 1.0 / (numSlices - 1.0);
            let end = stop[j];
            let diff = end - start[j];
            for i in 0..numSlices as usize {
                let p = start[j] + diff * i as f64 * step;
                cs[j].push(p.cos());
                ss[j].push(p.sin());
            }
            let p = start[j] + diff * (numSlices - 1.) * step;
            cs[j].push(p.cos());
            ss[j].push(p.sin());
        }

        let eps = f32::EPSILON;
        let mut shape = vec![];
        for (i, _) in xc.iter().enumerate() {
            for j in 0..numSlices as usize {
                let vertex = if j == 0 {
                    ShapeVertex {
                        position: [
                            xc[i] as f32,
                            yc[i] as f32,
                            // if z1[c] == 0.0 {
                            eps * i as f32 // } else {
                                           // z1[c] as f32
                                           // },
                        ],
                        color: [0.0, 0.0, 0.0, 0.0],
                        texcoord: [0f32, 0.],
                    }
                } else {
                    ShapeVertex {
                        position: [
                            (cs[i][j - 1] * w[i] + xc[i]) as f32,
                            (ss[i][j - 1] * h[i] + yc[i]) as f32,
                            // if z1[c] == 0.0 {
                            eps * i as f32 // } else {
                                           // z1[c] as f32
                                           // },
                        ],
                        color: [0.0, 0.0, 0.0, 0.0],
                        texcoord: [0f32, 0.],
                    }
                };
                shape.push(vertex);
            }
        }

        load_colors(&mut shape, &screen.fillCol);
        let fill_shape_buffer = match screen.display {
            ScreenType::Window(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
            ScreenType::Headless(ref d) => glium::VertexBuffer::new(d, &shape)
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?,
        };

        load_colors(&mut shape, &screen.strokeCol);
        let stroke_shape_buffer = match screen.display {
            ScreenType::Window(ref d) => {
                glium::VertexBuffer::new(d, &shape[1..shape.len() - 1])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?
            }
            ScreenType::Headless(ref d) => {
                glium::VertexBuffer::new(d, &shape[1..shape.len() - 1])
                	.map_err(|e| ProcessingErr::VBNoCreate(e))?
            }
        };

        Ok(Arc {
            fill_buffer: fill_shape_buffer,
            stroke_buffer: stroke_shape_buffer,
            fill_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
            },
            stroke_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::LineStrip),
            },
        })
    }
}
