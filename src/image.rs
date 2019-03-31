// Loading & Displaying
use std::mem;
use std::path::Path;

use image_ext;
use gl;
use errors::ProcessingErr;

/// A convienence function that will open an image (using the `image` crate) and
/// transform it to the format that glium expects for textures. Use the output of
/// this function as the input to screen.texture(). It is not necessary to use this
/// function, though. You can use whatever image crate and approach you want, just make /// sure that the input to screen.texture() is an image:::RgbaImage.
pub fn load_image(filename: &str) -> Result<image_ext::RgbaImage, ProcessingErr> {
    image_ext::open(filename).and_then(|img| Ok(img.to_rgba())).map_err(|e| ProcessingErr::ImageNotFound(e))
}

use Screen;

impl<'a> Screen<'a> {
	/// Not really useful in `processing-rs` since you will typically draw a texture to
	/// the screen by attaching it to a Rect, rather than having a separate function
	/// just for drawing an image to the screen. Should probably be removed.
    pub fn image_mode(&mut self, mode: &str) {
        self.image_mode = mode.to_owned();
    }

	/// Stop applying tint to a drawn image.
    pub fn no_tint(&mut self) {
        self.tint_stuff = false
    }

	/// Save the current state of the screen to an image. The format will be determined
	/// by the file extension.
    pub fn save(&self, filename: &str) -> Result<(), ProcessingErr> {
        let data = vec![0f32; self.fb_size[0] as usize * self.fb_size[1] as usize * 4 * 4];
        unsafe {
            gl::ReadPixels(
                0,
                0,
                self.fb_size[0] as gl::types::GLsizei,
                self.fb_size[1] as gl::types::GLsizei,
                gl::RGBA,
                gl::FLOAT,
                mem::transmute(&data[0]),
            );
        }

        let mut img = image_ext::ImageBuffer::new(self.fb_size[0], self.fb_size[1]);
        let mut i = 0;
        for pixel in img.pixels_mut() {
            *pixel = image_ext::Rgba(
                [
                    (data[i] * 255.0) as u8,
                    (data[i + 1] * 255.0) as u8,
                    (data[i + 2] * 255.0) as u8,
                    255u8,
                ],
            );
            i += 4;
        }
        let fimg = image_ext::imageops::flip_vertical(&img);

        fimg.save(&Path::new(filename)).map_err(|e| ProcessingErr::ImageNotSaved(e))
    }
}
