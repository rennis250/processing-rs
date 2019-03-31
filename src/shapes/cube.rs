use glium;

use {Screen, ScreenType};
use errors::ProcessingErr;

use shapes::{Shape, ShapeVertex, IndexType, load_colors};

/// A cube is a three-dimensional volume with equal width, height, and depth, so a box.
pub struct Cube {
    fill_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    stroke_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    fill_index_buffer: IndexType,
    stroke_index_buffer: IndexType,
}

impl Shape for Cube {
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

impl Cube {
	/// Create a new Cube of a given scale (s) to be drawn later.
	/// A scale of 1 is the standard "unit" cube and any other values
	/// give back a version of this cube scaled up or down.
    pub fn new(screen: &Screen, s: &[f64]) -> Result<Self, ProcessingErr> {
        let cube_vertices = [
            -1.0f32,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            1.0,
            -1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            1.0,
            -1.0,
            1.0,
            -1.0,
            1.0,
            -1.0,
            -1.0,
            -1.0,
            1.0,
            -1.0,
            -1.0,
            1.0,
            1.0,
            -1.0,
            1.0,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            1.0,
            1.0,
            -1.0,
            1.0,
            -1.0,
            1.0,
            -1.0,
            1.0,
            -1.0,
            -1.0,
            1.0,
            -1.0,
            -1.0,
            -1.0,
            -1.0,
            1.0,
            1.0,
            -1.0,
            -1.0,
            1.0,
            1.0,
            -1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            -1.0,
            -1.0,
            1.0,
            1.0,
            -1.0,
            1.0,
            -1.0,
            -1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            -1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            -1.0,
            -1.0,
            1.0,
            -1.0,
            1.0,
            1.0,
            1.0,
            -1.0,
            1.0,
            -1.0,
            -1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            -1.0,
            1.0,
            1.0,
            1.0,
            -1.0,
            1.0,
        ];

        let mut shape = vec![];
        for c in 0..s.len() {
            let mut x = 0;
            for _ in 0..35 {
                let vertex = ShapeVertex {
                    position: [
                        cube_vertices[x] * s[c] as f32,
                        cube_vertices[x + 1] * s[c] as f32,
                        cube_vertices[x + 2] * s[c] as f32,
                    ],
                    color: [0.0, 0.0, 0.0, 0.0],
                    texcoord: [0f32, 0.],
                };
                shape.push(vertex);
                x += 3;
            }
        }

        // if RenderState.drawTexture
        //	// texcoords
        //	texData = zeros(GLfloat, num_slices*4*len(xc))
        //	texData[8:vertexStride:end] = 0
        //	texData[9:vertexStride:end] = 0

        //	texData[17:vertexStride:end] = 1
        //	texData[18:vertexStride:end] = 0

        //	texData[26:vertexStride:end] = 1
        //	texData[27:vertexStride:end] = 1

        //	texData[35:vertexStride:end] = 0
        //	texData[36:vertexStride:end] = 1
        // end

        // elements = zeros(GLuint, 6*len(x1))

        // elements[1] = 0
        // elements[2] = 1
        // elements[3] = 2
        // elements[4] = 2
        // elements[5] = 3
        // elements[6] = 0

        // index = 7
        // for x = 2:len(x1)
        //	elements[index] = elements[index-6] + 4
        //	elements[index+1] = elements[(index-6)+1] + 4
        //	elements[index+2] = elements[(index-6)+2] + 4
        //	elements[index+3] = elements[(index-6)+3] + 4
        //	elements[index+4] = elements[(index-6)+4] + 4
        //	elements[index+5] = elements[(index-6)+5] + 4
        //	index += 6
        // end

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
        Ok(Cube {
            fill_buffer: fill_shape_buffer,
            stroke_buffer: stroke_shape_buffer,
            fill_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            },
            stroke_index_buffer: IndexType::NoBuffer {
                ind: glium::index::NoIndices(glium::index::PrimitiveType::LineLoop),
            },
        })
    }
}
