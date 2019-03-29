use glium::backend::glutin::DisplayCreationError;
use glium::texture::TextureCreationError;
use glium::framebuffer::ValidationError;
use glium::index;
use glium::vertex;
use glium::IncompatibleOpenGl;
use glium::glutin::ContextError;
use glium::glutin::CreationError;
use glium::DrawError;
use glium::SwapBuffersError;
use glium::program::ProgramCreationError;
use image_ext::ImageError;

use std::io;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ProcessingErr {
	TextureNoCreate(TextureCreationError),
	ShaderCompileFail(ProgramCreationError),
	VBNoCreate(vertex::BufferCreationError),
	IBNoCreate(index::BufferCreationError),
	DrawFailed(DrawError),
	SwapFailed(SwapBuffersError),
	FBNoCreate(ValidationError),
	FBDrawFailed(DrawError),
	ShaderNotFound(io::Error),
	IncludeNotFound(io::Error),
	FullShaderNoCreate(io::Error),
	FullShaderNoWrite(io::Error),
	ImageNotFound(ImageError),
	ImageNotSaved(io::Error),
	ErrorReadingInclude(io::Error),
	ErrorReadingShader(usize, io::Error),
	DisplayNoCreate(DisplayCreationError),
	ContextNoCreate(IncompatibleOpenGl),
	HeadlessRendererNoBuild(CreationError),
	HeadlessContextError(ContextError),
	HeadlessNoCreate(IncompatibleOpenGl),
	GLFWWindowNoCreate,
	GLFWAlreadyInited,
	GLFWInternal
}

#[derive(Debug)]
pub struct ErrorReadingIncludeLineInShader {
	details: String
}

impl ErrorReadingIncludeLineInShader {
	pub fn new(msg: &str) -> Self {
		ErrorReadingIncludeLineInShader {
			details: msg.to_string()
		}
	}
}

impl fmt::Display for ErrorReadingIncludeLineInShader {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Perhaps missing file name for include?")
	}
}

impl Error for ErrorReadingIncludeLineInShader {
	fn description(&self) -> &str {
		&self.details
	}
}