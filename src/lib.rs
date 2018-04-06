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

use std::collections::HashMap;

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
    pub currMatrix: Matrix4<f32>,
    matrixStack: Vec<Matrix4<f32>>,
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

#[cfg(not(feature = "glfw"))]
pub struct Screen<'a> {
    FBTexture: glium::texture::Texture2d,
    fb_shape_buffer: glium::VertexBuffer<DFBFDVertex>,
    fb_index_buffer: glium::index::IndexBuffer<u16>,
    FBO: owning_ref::OwningHandle<Box<FBtexs>, Box<glium::framebuffer::SimpleFrameBuffer<'a>>>,
    display: ScreenType,
    events_loop: glutin::EventsLoop,
    draw_params: glium::draw_parameters::DrawParameters<'a>,
    pub matrices: GLmatStruct,
    bgCol: Vec<f32>,
    fillStuff: bool,
    fillCol: Vec<f32>,
    strokeStuff: bool,
    strokeCol: Vec<f32>,
    tintStuff: bool,
    tintCol: Vec<f32>,
    shader_bank: Vec<glium::program::Program>,
    drawTexture: bool,
    aspectRatio: f32,
    preserveAspectRatio: bool,
    fbSize: Vec<u32>,
    strokeWeight: f32,
    fontFace: String,
    textSize: f32,
    height: u32,
    width: u32,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    cMode: String,
    title: String,
    ellipseMode: String,
    rectMode: String,
    shapeMode: String,
    imageMode: String,
    frameRate: isize,
    frameCount: isize,
    fontsInitialized: bool,
    CurrShader: usize,
    currCursor: glium::glutin::MouseCursor,
    wrap: glium::uniforms::SamplerWrapFunction,
    AlternateShader: usize,
    CurrTexture: Option<glium::texture::Texture2d>,
    UsingAlternateShader: bool,
    GlslVersion: String,
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
#[cfg(feature = "glfw")]
pub struct Screen<'a> {
    FBTexture: glium::texture::Texture2d,
    fb_shape_buffer: glium::VertexBuffer<DFBFDVertex>,
    fb_index_buffer: glium::index::IndexBuffer<u16>,
    FBO: owning_ref::OwningHandle<Box<FBtexs>, Box<glium::framebuffer::SimpleFrameBuffer<'a>>>,
    display: ScreenType,
    glfw: glfw::Glfw,
    events_loop: Receiver<(f64, glfw::WindowEvent)>,
    draw_params: glium::draw_parameters::DrawParameters<'a>,
    pub matrices: GLmatStruct,
    bgCol: Vec<f32>,
    fillStuff: bool,
    fillCol: Vec<f32>,
    strokeStuff: bool,
    strokeCol: Vec<f32>,
    tintStuff: bool,
    tintCol: Vec<f32>,
    shader_bank: Vec<glium::program::Program>,
    drawTexture: bool,
    aspectRatio: f32,
    preserveAspectRatio: bool,
    fbSize: Vec<u32>,
    strokeWeight: f32,
    fontFace: String,
    textSize: f32,
    height: u32,
    width: u32,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    cMode: String,
    title: String,
    ellipseMode: String,
    rectMode: String,
    shapeMode: String,
    imageMode: String,
    frameRate: isize,
    frameCount: isize,
    fontsInitialized: bool,
    CurrShader: usize,
    currCursor: glfw::StandardCursor,
    wrap: glium::uniforms::SamplerWrapFunction,
    AlternateShader: usize,
    CurrTexture: Option<glium::texture::Texture2d>,
    UsingAlternateShader: bool,
    GlslVersion: String,
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
