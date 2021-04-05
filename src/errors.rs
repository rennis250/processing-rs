use glium::backend::glutin::DisplayCreationError;
use glium::framebuffer::ValidationError;
use glium::glutin::ContextError;
use glium::glutin::CreationError;
use glium::index;
use glium::program::ProgramCreationError;
use glium::texture::TextureCreationError;
use glium::vertex;
use glium::DrawError;
use glium::IncompatibleOpenGl;
use glium::SwapBuffersError;
use image_ext::ImageError;

use std::error::Error;
use std::fmt;
use std::io;

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
	ImageNotSaved(ImageError),
	ErrorReadingInclude(io::Error),
	ErrorReadingShader(usize, io::Error),
	DisplayNoCreate(DisplayCreationError),
	ContextNoCreate(IncompatibleOpenGl),
	CursorStateNotSet(String),
	HeadlessRendererNoBuild(CreationError),
	HeadlessContextError(ContextError),
	HeadlessNoCreate(IncompatibleOpenGl),
	GLFWWindowNoCreate,
	GLFWAlreadyInited,
	GLFWInternal,
}

#[derive(Debug)]
pub struct ErrorReadingIncludeLineInShader {
	details: String,
}

impl ErrorReadingIncludeLineInShader {
	pub fn new(msg: &str) -> Self {
		ErrorReadingIncludeLineInShader {
			details: msg.to_string(),
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
