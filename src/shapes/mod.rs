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

/// A shape is a generic concept, as expressed by the Shape trait here. From the point
/// of view of `processing-rs`, a shape is anything that has fill and stroke buffer
/// arrays reserved for it on the GPU, has indices that determine whether or not it
/// should be drawn as specified by an element buffer, and may or may not have a
/// a texture attached to it. This concept could be modified and expanded with time, but
/// the details need not concern the average user. All of the standard Processing shapes
/// are provided in this module and they all already implement the Shape trait.
pub trait Shape {
    fn fill_buffer(&self) -> Box<&glium::vertex::VertexBuffer<ShapeVertex>>;
    fn stroke_buffer(&self) -> Box<&glium::vertex::VertexBuffer<ShapeVertex>>;
    fn fill_indices(&self) -> Box<&IndexType>;
    fn stroke_indices(&self) -> Box<&IndexType>;
    fn get_texture(&self) -> Option<Box<&glium::texture::Texture2d>>;
}

fn load_colors(buffer: &mut [ShapeVertex], color_mat: &[f32]) {
    if color_mat.len() == 4 {
        for x in 0..buffer.len() {
            buffer[x].color = [color_mat[0], color_mat[1], color_mat[2], color_mat[3]];
        }
    } else {
        // for c in 0..color_mat.len() / 4 {
            // let idx = c * 4;
            // for x in (c * shapeStride..(c + 1) * shapeStride).filter(|x| x % 4 == 0) {
            // $buffer[x].color = [
            // $color_mat[idx],
            // $color_mat[idx + 1],
            // $color_mat[idx + 2],
            // $color_mat[idx + 3],
            // ];
            // }
        // }
    }
}
