use glium;

pub mod draw;
pub mod rect;
pub mod arc;
pub mod ellipse;
pub mod point;
pub mod line;
pub mod quad;
pub mod triangle;
pub mod cube;
pub mod mould;

pub enum IndexType {
    Buffer { ind: glium::index::IndexBuffer<u32> },
    NoBuffer { ind: glium::index::NoIndices },
}

#[derive(Copy, Clone)]
pub struct ShapeVertex {
    position: [f32; 3],
    color: [f32; 4],
    texcoord: [f32; 2],
}

implement_vertex!(ShapeVertex, position, color, texcoord);

pub trait Shape {
    fn fill_buffer(&self) -> Box<&glium::vertex::VertexBuffer<ShapeVertex>>;
    fn stroke_buffer(&self) -> Box<&glium::vertex::VertexBuffer<ShapeVertex>>;
    fn fill_indices(&self) -> Box<&IndexType>;
    fn stroke_indices(&self) -> Box<&IndexType>;
    fn get_texture(&self) -> Option<Box<&glium::texture::Texture2d>>;
}

fn load_colors(buffer: &mut [ShapeVertex], colorMat: &[f32]) {
    if colorMat.len() == 4 {
        for x in 0..buffer.len() {
            buffer[x].color = [colorMat[0], colorMat[1], colorMat[2], colorMat[3]];
        }
    } else {
        for c in 0..colorMat.len() / 4 {
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
}
