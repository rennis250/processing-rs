use glium;
use glium::uniforms::Uniforms;

use shaders::ShaderInfo;
use shapes::{Shape, IndexType};

pub struct Mould<U: Uniforms, S: Shape> {
    shape: S,
    shader: ShaderInfo<U>,
}

impl<U: Uniforms, S: Shape> Mould<U, S> {
    pub fn new(shape: S, shader: ShaderInfo<U>) -> Self {
        Mould {
            shape: shape,
            shader: shader,
        }
    }

    pub fn get_shape(&self) -> &S {
        &self.shape
    }

    pub fn get_shader(&self) -> &ShaderInfo<U> {
        &self.shader
    }

    pub fn set(&mut self, uniforms: U) {
        self.shader.set(uniforms)
    }
}
