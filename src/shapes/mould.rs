use glium::uniforms::UniformValue;

use shaders::ShaderInfo;
use shapes::Shape;

/// A Mould is a Shape that has been paired with a custom shader. This is useful when
/// you want to modify the way a few shapes are drawn, without completely altering 
/// the standard `processing-rs` rendering state. The concept was borrowed from
/// libCinder.
pub struct Mould<'a, S: Shape> {
    shape: &'a S,
    shader: &'a mut ShaderInfo<'a>,
}

impl<'a, S: Shape> Mould<'a, S> {
    pub fn new(shape: &'a S, shader: &'a mut ShaderInfo<'a>) -> Self {
        Mould {
            shape: shape,
            shader: shader,
        }
    }

    pub fn get_shape(&self) -> &S {
        self.shape
    }

    pub fn get_shader(&self) -> &ShaderInfo<'a> {
        self.shader
    }

    pub fn set(&mut self, uni_name: &'a str, uniform: UniformValue<'a>) {
        self.shader.set(uni_name, uniform)
    }
}
