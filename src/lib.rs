//! # processing-rs
//!
//! `processing-rs` is a crate that is designed to make graphics
//! programming as easy in Rust as it is in the popular Processing
//! environment, without sacrificing performance or easy access to
//! other parts of your system. It achieves this by essentially being
//! a convienence layer built on top of glium and glutin/glfw (either
//! can be chosen depending on your preference). It mostly mimics Processing,
//! but diverges in a few areas to either accomodate Rust's and glium's safety
//! features or to incorporate some ideas from libCinder and openFrameworks. For
//! instance, there are now `setup` and `draw` loops in `processing-rs`. This is
//! intended to enable a more flexible and modular workflow for graphics. In addition,
//! since `processing-rs` is essentialy a thin wrapper, you can use glium, glutin, and
//! raw OpenGL calls as you see fit. They should usually blend nicely with the
//! `processing-rs` calls and allow for more opportunities to create fun and
//! interesting displays.
//!
//! Typical usage follows a common pattern:
//!
//!     1. Open a screen with processing::Screen::new(). This will return a
//!        Screen struct, which is the central struct for coordinating draw calls,
//!        managing shaders, and maintaining render state.
//!
//!     2. Pre-define a few shapes. This allows caching shape data on the CPU and GPU,
//!        giving some marginal speed boosts and allowing the possibility to store
//!        shapes in collections, such as Vectors. All shapes (Rect, Ellipse, etc.) in
//!        this crate implement the Shape trait, which defines a common interface for
//!        entities that can be drawn to a screen in a meaningful way.
//!
//!     3. Draw the shapes to the screen with screen.draw(). This is also where you will
//!        want to use commands like screen.fill() and screen.stroke() to change the
//!        colors of objects.
//!
//!     4. For any pattern drawn, you will also need to flip the framebuffers, so that
//!        the pattern is synchronized with your monitor. This is achieved by
//!        screen.reveal().
//!
//!     5. Use commands like screen.key_press() to get input from users, if needed.
//!
//!     6. Have fun! :-)
//!
//! Basically, all commands follow the same call conventions as those from Processing,
//! so you can also use the Processing reference as additional documentation and for
//! some basic examples of what you can do.
//!
//! Additionally, `processing-rs` has a number of features that are intended to make
//! it useful for psychological research, including color vision, material perception,
//! motion, etc. For example, it tries to enable a 10-bit framebuffer if possible, for
//! increased color fidelity, which is important in most color vision research. Besides
//! this, it also aims to make sure that frame draws are as precisely synchronized with
//! the monitor refresh as possible. This works better with glutin than with glfw, and
//! on Mac, a few Objective-C functions are called to give the program elevated status
//! for resources and to disable AppNap and such things while the program is running
//! (code taken from the helpful PsychToolbox). In addition, when the crate is built
//! on a Mac, it will also compile a C library (called `libpri`, short for "priority
//! library") that asks the operating system for some additional priorities. The crate
//! will automatically call the library throguh the C FFI at screen initialization. It
//! shouldn't interfere with normal operation of the operating system, so you can
//! probably just accept the default behaviour. With all of this, combined with a change
//! to a setting on Mac that allows one to quit Finder, one can achieve slightly better
//! synchronization than PsychToolbox on Mac, satisfying Psychtoolbox's requirements for
//! good frame synchronization. However, this has only been tested on a Mac laptop with
//! 10.13.3 and an Intel graphics card.

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(deprecated)]

#[macro_use]
extern crate glium;
extern crate gl;
extern crate nalgebra;
//extern crate rand;
extern crate image as image_ext;
extern crate owning_ref;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
use cocoa::foundation::{NSProcessInfo, NSString};
#[cfg(target_os = "macos")]
use cocoa::base::nil;

use nalgebra::{Matrix4, Vector3, Unit};

#[cfg(not(feature = "glfw"))]
pub use glium::*;
#[cfg(not(feature = "glfw"))]
pub mod screen;

#[cfg(feature = "glfw")]
extern crate glfw;
#[cfg(feature = "glfw")]
pub mod glfwp5;
#[cfg(feature = "glfw")]
pub use glfwp5::screen;

#[macro_use]
pub mod shaders;
pub mod color;
pub mod shapes;
pub mod textures;
pub mod framebuffers;
pub mod transform;
pub mod rendering;
pub mod image;
pub mod errors;

#[cfg(not(feature = "glfw"))]
pub mod environment;
#[cfg(feature = "glfw")]
pub use glfwp5::environment;

#[cfg(not(feature = "glfw"))]
pub mod input;
#[cfg(feature = "glfw")]
pub use glfwp5::input;

#[cfg(not(feature = "glfw"))]
pub mod constants;
#[cfg(feature = "glfw")]
pub use glfwp5::constants;

pub use constants::{Key, MouseButton};

pub use image::load_image;

#[derive(Debug)]
pub struct GLmatStruct {
    pub curr_matrix: Matrix4<f32>,
    matrix_stack: Vec<Matrix4<f32>>,
}

pub struct FBtexs {
    fbtex: glium::texture::Texture2d,
    depthtexture: glium::texture::DepthTexture2d,
}

#[derive(Copy, Clone)]
pub struct DFBFDVertex {
    position: [f32; 2],
    texcoord: [f32; 2],
}

implement_vertex!(DFBFDVertex, position, texcoord);

#[cfg(not(feature = "glfw"))]
enum ScreenType {
    Window(glium::Display),
    Headless(glium::HeadlessRenderer),
}

/// This is essentially the central struct of `processing-rs`. It not only contains the
/// the display window returned by glutin, but it also has a number of elements that
/// maintain the render state and that manage a framebuffer for increased color
/// fidelity. Its internal elements are mostly private and an instance of a Screen
/// struct should be interacted with through the public functions provided in other
/// modules, such as the shapes, environment, or textures modules.
#[cfg(not(feature = "glfw"))]
pub struct Screen<'a> {
    fbtexture: glium::texture::Texture2d,
    fb_shape_buffer: glium::VertexBuffer<DFBFDVertex>,
    fb_index_buffer: glium::index::IndexBuffer<u16>,
    fbo: owning_ref::OwningHandle<Box<FBtexs>, Box<glium::framebuffer::SimpleFrameBuffer<'a>>>,
    display: ScreenType,
    events_loop: glutin::EventsLoop,
    draw_params: glium::draw_parameters::DrawParameters<'a>,
    pub matrices: GLmatStruct,
    bg_col: Vec<f32>,
    fill_stuff: bool,
    fill_col: Vec<f32>,
    stroke_stuff: bool,
    stroke_col: Vec<f32>,
    tint_stuff: bool,
    tint_col: Vec<f32>,
    shader_bank: Vec<glium::program::Program>,
    draw_texture: bool,
    aspect_ratio: f32,
    preserve_aspect_ratio: bool,
    fb_size: Vec<u32>,
    stroke_weight: f32,
    font_face: String,
    text_size: f32,
    height: u32,
    width: u32,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    c_mode: String,
    title: String,
    ellipse_mode: String,
    rect_mode: String,
    shape_mode: String,
    image_mode: String,
    frame_rate: isize,
    frame_count: isize,
    fonts_initialized: bool,
    curr_shader: usize,
    curr_cursor: glium::glutin::MouseCursor,
    wrap: glium::uniforms::SamplerWrapFunction,
    alternate_shader: usize,
    curr_texture: Option<glium::texture::Texture2d>,
    using_alternate_shader: bool,
    uniforms: Option<glium::uniforms::UniformsStorage<'a>>,
    texture_list: Option<Vec<(String, gl::types::GLuint)>>,
    glsl_version: String,
    drew_points: bool,
    keypressed: Option<glutin::VirtualKeyCode>,
    mousepressed: Option<glutin::MouseButton>,
    mousereleased: Option<glutin::MouseButton>,
    mousepos: (f64, f64),
    headless: bool,
}

#[cfg(feature = "glfw")]
use std::sync::mpsc::Receiver;
#[cfg(feature = "glfw")]
use glfwp5::backend::Display;
#[cfg(feature = "glfw")]
enum ScreenType {
    Window(Display),
    Headless(Display),
}

/// This is essentially the central struct of `processing-rs`. It not only contains the
/// the display window returned by glutin, but it also has a number of elements that
/// maintain the render state and that manage a framebuffer for increased color
/// fidelity. Its internal elements are mostly private and an instance of a Screen
/// struct should be interacted with through the public functions provided in other
/// modules, such as the shapes, environment, or textures modules.
#[cfg(feature = "glfw")]
pub struct Screen<'a> {
    fbtexture: glium::texture::Texture2d,
    fb_shape_buffer: glium::VertexBuffer<DFBFDVertex>,
    fb_index_buffer: glium::index::IndexBuffer<u16>,
    fbo: owning_ref::OwningHandle<Box<FBtexs>, Box<glium::framebuffer::SimpleFrameBuffer<'a>>>,
    display: ScreenType,
    glfw: glfw::Glfw,
    events_loop: Receiver<(f64, glfw::WindowEvent)>,
    draw_params: glium::draw_parameters::DrawParameters<'a>,
    pub matrices: GLmatStruct,
    bg_col: Vec<f32>,
    fill_stuff: bool,
    fill_col: Vec<f32>,
    stroke_stuff: bool,
    stroke_col: Vec<f32>,
    tint_stuff: bool,
    tint_col: Vec<f32>,
    shader_bank: Vec<glium::program::Program>,
    draw_texture: bool,
    aspect_ratio: f32,
    preserve_aspect_ratio: bool,
    fb_size: Vec<u32>,
    stroke_weight: f32,
    font_face: String,
    text_size: f32,
    height: u32,
    width: u32,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    c_mode: String,
    title: String,
    ellipse_mode: String,
    rect_mode: String,
    shape_mode: String,
    image_mode: String,
    frame_rate: isize,
    frame_count: isize,
    fonts_initialized: bool,
    curr_shader: usize,
    curr_cursor: glfw::StandardCursor,
    wrap: glium::uniforms::SamplerWrapFunction,
    alternate_shader: usize,
    curr_texture: Option<glium::texture::Texture2d>,
    using_alternate_shader: bool,
    uniforms: Option<glium::uniforms::UniformsStorage<'a>>,
    texture_list: Option<Vec<(String, gl::types::GLuint)>>,
    glsl_version: String,
    drew_points: bool,
    keypressed: Option<glfw::Key>,
    mousepressed: Option<glfw::MouseButton>,
    mousereleased: Option<glfw::MouseButton>,
    mousepos: (f64, f64),
    headless: bool,
}

// #[derive(Default)]
// struct vertexStruct {
// shapeVertices: Vec<f32>,
// textureCoords: Vec<f32>,
// vertexStride: isize,
// nVertices: isize,
// shapeType: u32,
// }

#[cfg(target_os = "macos")]
#[link(name = "pri")]
extern "C" {
    fn setMaxPriority();
}

#[cfg(target_os = "macos")]
fn mac_priority() {
    // Prevent display from sleeping/powering down, prevent system
    // from sleeping, prevent sudden termination for any reason:
    let NSActivityIdleDisplaySleepDisabled = (1u64 << 40);
    let NSActivityIdleSystemSleepDisabled = (1u64 << 20);
    let NSActivitySuddenTerminationDisabled = (1u64 << 14);
    let NSActivityAutomaticTerminationDisabled = (1u64 << 15);
    let NSActivityUserInitiated = (0x00FFFFFFu64 | NSActivityIdleSystemSleepDisabled);
    let NSActivityLatencyCritical = 0xFF00000000u64;

    let options = NSActivityIdleDisplaySleepDisabled | NSActivityIdleSystemSleepDisabled |
        NSActivitySuddenTerminationDisabled |
        NSActivityAutomaticTerminationDisabled;
    let options = options | NSActivityUserInitiated | NSActivityLatencyCritical;

    unsafe {
        let pinfo = NSProcessInfo::processInfo(nil);
        let s = NSString::alloc(nil).init_str("timing");
        msg_send![pinfo, beginActivityWithOptions:options reason:s];

        setMaxPriority();
    }
}
