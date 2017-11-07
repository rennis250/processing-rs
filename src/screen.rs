use std::str;
use std::collections::HashMap;

use owning_ref;
use Matrix4;
use gl;
use glium;
use glium::backend::Facade;
use glium::{glutin, Surface, GlObject};
use glium::uniforms::Uniforms;
use glium::glutin::GlContext;

use shapes::ShapeVertex;
use shaders::ShaderInfo;
use {GLmatStruct, FBtexs, Screen, DFBFDVertex};

#[cfg(target_os = "macos")]
use mac_priority;

impl<'a> Screen<'a> {
    pub fn new(
        width: u32,
        height: u32,
        fullscreen: bool,
        preserveAspectRatio: bool,
        vsync: bool,
    ) -> Screen<'a> {
        #[cfg(target_os = "macos")] mac_priority();

        let mut w = width;
        let mut h = height;
        let events_loop = glutin::EventsLoop::new();
        let window;
        if fullscreen {
            let m = events_loop.get_primary_monitor();
            let wh = m.get_dimensions();
            w = wh.0;
            h = wh.1;
            window = glutin::WindowBuilder::new()
                .with_title("Processing-rs")
                .with_visibility(true)
                .with_fullscreen(Some(m))
                .with_decorations(false)
                .with_dimensions(w, h);
        } else {
            window = glutin::WindowBuilder::new()
                .with_title("Processing-rs")
                .with_visibility(true)
                .with_dimensions(w, h);
        }
        let context = glutin::ContextBuilder::new()
            .with_vsync(vsync)
            //.with_pixel_format(30, 2)
            //.with_depth_buffer(32)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 1)));
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // Load the OpenGL function pointers
        // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
        gl::load_with(|symbol| {
            (*display.gl_window()).get_proc_address(symbol) as *const _
        });

        let mut GlslVersion;
        {
            let glt = display.get_context().get_opengl_version();
            GlslVersion = format!("{}{:0<2}", glt.1, glt.2).to_owned();
            println!("OpenGL version {}", GlslVersion);
        }

        // if frameRate == 0 {
        // glfw.window_hint(glfw::WindowHint::RefreshRate(Some(60)));
        // } else if frameRate == -1 {
        // let system determine frame rate
        // } else {
        // glfw.window_hint(glfw::WindowHint::RefreshRate(
        // Some(frameRate as u32),
        // ));
        // }

        display.gl_window().show();
        display.gl_window().set_inner_size(w, h);

        if let Some(fb) = display.gl_window().get_inner_size_pixels() {
            w = fb.0;
            h = fb.1;
        }

        let aspectRatio = w as f32 / h as f32;

        let fbSize = vec![w, h];
        unsafe {
            gl::Viewport(
                0,
                0,
                fbSize[0] as gl::types::GLsizei,
                fbSize[1] as gl::types::GLsizei,
            );
        }

        let FBTexture = glium::texture::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            w,
            h,
        ).unwrap();
        let fbid = FBTexture.get_id();
        let depthtexture = glium::texture::DepthTexture2d::empty_with_format(
            &display,
            glium::texture::DepthFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            w,
            h,
        ).unwrap();
        let oh = owning_ref::OwningHandle::new_with_fn(
            Box::new(FBtexs {
                fbtex: FBTexture,
                depthtexture: depthtexture,
            }),
            |v| unsafe {
                Box::new(
                    glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                        &display,
                        &(*v).fbtex,
                        &(*v).depthtexture,
                    ).unwrap(),
                )
            },
        );
        let FBTexture = unsafe {
            glium::texture::Texture2d::from_id(
                &display,
                glium::texture::UncompressedFloatFormat::F32F32F32F32,
                fbid,
                true,
                glium::texture::MipmapsOption::NoMipmap,
                glium::texture::Dimensions::Texture2d {
                    width: w,
                    height: h,
                },
            )
        };

        let draw_params = glium::draw_parameters::DrawParameters {
            point_size: Some(2f32),
            line_width: Some(2f32),
            depth: glium::Depth {
                write: true,
                test: glium::DepthTest::Overwrite,
                ..Default::default()
            },
            blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            },
            multisampling: false,
            // smooth: Some(glium::draw_parameters::Smooth::Nicest),
            smooth: None,
            ..Default::default()
        };

        display.gl_window().set_cursor(
            glium::glutin::MouseCursor::Default,
        );

        // if !fontsInitialized {
        // setupFontCharacters()
        // }
        // fontsInitialized = true

        // glCheckError("Screen initilization.");

        // by default, use system fonts that are known to basically always be available
        let mut fontFace = "".to_owned();
        if cfg!(target_os = "windows") {
            fontFace = "C:/Windows/Fonts/arial.ttf".to_owned();
        } else if cfg!(target_os = "linux") {
            fontFace = "/usr/share/fonts/ttf-dejavu-ib/DejaVuSansMono.ttf".to_owned();
        } else if cfg!(target_os = "macos") {
            fontFace = "/Users/rje/Library/Fonts/Go-Regular.ttf".to_owned();
        }

        let shader_bank = init_shaders(&display, &GlslVersion);

        let vertex1 = DFBFDVertex {
            position: [-1.0, 1.0],
            texcoord: [0.0, 0.0],
        };
        let vertex2 = DFBFDVertex {
            position: [1.0, 1.0],
            texcoord: [1.0, 0.0],
        };
        let vertex3 = DFBFDVertex {
            position: [1.0, -1.0],
            texcoord: [1.0, 1.0],
        };
        let vertex4 = DFBFDVertex {
            position: [-1.0, -1.0],
            texcoord: [0.0, 1.0],
        };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        let fb_shape_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let fb_index_buffer = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &[0u16, 1, 2, 0, 2, 3],
        ).unwrap();

        Screen {
            // start with default identity matrix, as expected.
            matrices: GLmatStruct {
                currMatrix: Matrix4::new(
                    1.0f32,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                ),
                matrixStack: vec![
                    Matrix4::new(
                    1.0f32,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0
                );
                    1
                ],
            },
            FBO: oh,
            FBTexture: FBTexture,
            fb_shape_buffer: fb_shape_buffer,
            fb_index_buffer: fb_index_buffer,
            display: display,
            events_loop: events_loop,
            draw_params: draw_params,
            bgCol: vec![0.8f32, 0.8, 0.8, 0.8],
            fillStuff: true,
            fillCol: vec![1.0f32, 1.0, 1.0, 1.0],
            strokeStuff: true,
            strokeCol: vec![0.0f32, 0.0, 0.0, 1.0],
            tintStuff: false,
            tintCol: vec![1.0f32, 1.0, 1.0, 1.0],
            shader_bank: shader_bank,
            drawTexture: false,
            aspectRatio: aspectRatio,
            preserveAspectRatio: preserveAspectRatio,
            fbSize: fbSize,
            strokeWeight: 1.0,
            fontFace: fontFace,
            textSize: 0.4,
            height: height,
            width: width,
            left: -1f32,
            right: 1f32,
            top: 1f32,
            bottom: -1f32,
            cMode: "RGB".to_owned(),
            title: "processing-rs".to_owned(),
            ellipseMode: "CENTER".to_owned(),
            rectMode: "CORNER".to_owned(),
            shapeMode: "CORNER".to_owned(),
            imageMode: "CORNER".to_owned(),
            frameRate: 60,
            frameCount: 0,
            fontsInitialized: false,
            CurrShader: 0,
            currCursor: glium::glutin::MouseCursor::Default,
            wrap: glium::uniforms::SamplerWrapFunction::Repeat,
            CurrTexture: None,
            AlternateShader: 1 << 20,
            UsingAlternateShader: false,
            GlslVersion: GlslVersion,
            drew_points: false,
            keypressed: None,
            mousepressed: None,
            mousereleased: None,
            mousepos: (-100., -100.),
        }
    }

    #[inline]
    pub fn reveal(&mut self) {
        let mut target = self.display.draw();
        {
            let uniforms = uniform! { texFramebuffer: &self.FBTexture };
            let p = &self.shader_bank[3];
            target
                .draw(
                    &self.fb_shape_buffer,
                    &self.fb_index_buffer,
                    p,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
            target.finish().unwrap();
        }

        let mut kp = None;
        let mut mp = None;
        let mut mr = None;
        let mut mpos = (-100., -100.);
        self.events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => panic!("need a smoother way to quit..."),
                    glutin::WindowEvent::KeyboardInput { input, .. }
                        if glutin::ElementState::Pressed == input.state => {
                        match input.virtual_keycode {
                            Some(b) => {
                                kp = Some(b);
                            }
                            _ => (),
                        }
                    }
                    glutin::WindowEvent::MouseInput {
                        state: s,
                        button: b,
                        ..
                    } if glutin::ElementState::Pressed == s => {
                        mp = Some(b);
                    }
                    glutin::WindowEvent::MouseInput {
                        state: s,
                        button: b,
                        ..
                    } if glutin::ElementState::Released == s => {
                        mr = Some(b);
                    }
                    glutin::WindowEvent::MouseMoved { position, .. } => {
                        mpos = position;
                    }
                    _ => (),
                }
            }
            _ => (),
        });

        self.keypressed = kp;
        self.mousepressed = mp;
        self.mousereleased = mr;
        self.mousepos = mpos;

        self.frameCount += 1;
    }

    pub fn clone_display(&self) -> glium::Display {
        self.display.clone()
    }

    pub fn end_drawing(self) {}
}

//
// pub fn drawing_window(glfw: &mut glfw::Glfw, window: &mut glfw::Window) {
//     glfw.make_context_current(Some(window));
//     window.show();
// }


pub fn init_shaders(display: &glium::Display, GlslVersion: &str) -> Vec<glium::program::Program> {
    let mut shader_bank = Vec::new();

    // basicShapes
    let vshBS = "
    #version "
        .to_owned() + &GlslVersion +
        "

    in vec3 position;
    in vec4 color;

    out vec4 vColor;

    uniform \
                 mat4 MVP;

    void main() {
        vColor = color;
        gl_Position = MVP * \
                 vec4(position, 1.0);
    }
    ";

    let fshBS = "
    #version "
        .to_owned() + &GlslVersion +
        "

    in vec4 vColor;

    out vec4 outColor;

    void main() {
        \
                 outColor = vColor;
    }
    ";

    let bs_program = glium::Program::new(
        display,
        glium::program::ProgramCreationInput::SourceCode {
            vertex_shader: &vshBS,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: &fshBS,
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: true,
        },
    ).unwrap();
    shader_bank.push(bs_program);

    // texturedShapes
    let vshTS = "
    #version "
        .to_owned() + &GlslVersion +
        "

    in vec3 position;
    in vec4 color;
    in vec2 texcoord;

    out vec4 \
                 vColor;
    out vec2 Texcoord;

    uniform mat4 MVP;

    void main() {
        \
                 vColor = color;
        Texcoord = texcoord;
        gl_Position = MVP * \
                 vec4(position, 1.0);
    }
    ";

    let fshTS = "
    #version "
        .to_owned() + &GlslVersion +
        "

    in vec4 vColor;
    in vec2 Texcoord;

    out vec4 outColor;

    uniform \
                 sampler2D tex;

    void main() {
        outColor = texture(tex, Texcoord) * \
                 vColor;
    }
    ";

    let ts_program = glium::Program::new(
        display,
        glium::program::ProgramCreationInput::SourceCode {
            vertex_shader: &vshTS,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: &fshTS,
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: true,
        },
    ).unwrap();
    shader_bank.push(ts_program);

    // fontDrawing
    let vshFD = "
    #version "
        .to_owned() + &GlslVersion +
        "

    in vec2 position;
    in vec2 texcoord;

    out vec2 TexCoord;

    \
                 uniform mat4 proj;

    void main()
    {
        gl_Position = proj * \
                 vec4(position, 0.0, 1.0);
        TexCoord = texcoord;
    }
    ";

    let fshFD = "
    #version "
        .to_owned() + &GlslVersion +
        "

    in vec2 TexCoord;

    out vec4 color;

    uniform sampler2D text;
    \
                 uniform vec3 textColor;

    void main()
    {
        vec4 sampled = vec4(1.0, \
                 1.0, 1.0, texture(text, TexCoord).r);
        color = vec4(textColor, 1.0) * \
                 sampled;
    }
    ";

    let fd_program = glium::Program::new(
        display,
        glium::program::ProgramCreationInput::SourceCode {
            vertex_shader: &vshFD,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: &fshFD,
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: true,
        },
    ).unwrap();
    shader_bank.push(fd_program);

    // text is rendered with an orthographic projection
    // let projection = vec![
    //     2.0f32 / width as f32,
    //     0.,
    //     0.,
    //     -1.,
    //     0.,
    //     2.0 / height as f32,
    //     0.,
    //     -1.,
    //     0.,
    //     0.,
    //     -1.,
    //     0.,
    //     0.,
    //     0.,
    //     0.,
    //     1.,
    // ];

    // unsafe {
    //     gl::UniformMatrix4fv(
    //         gl::GetUniformLocation(
    //             *shader_bank.get("fontDrawing").unwrap(),
    //             CString::new("proj").unwrap().as_ptr(),
    //         ),
    //         1,
    //         gl::FALSE as GLboolean,
    //         mem::transmute(&projection[0]),
    //     );
    // }

    // framebuffer
    let vshDFB = "
    #version "
        .to_owned() + &GlslVersion +
        "

    in vec2 position;
    in vec2 texcoord;

    out vec2 Texcoord;

    void \
                  main() {
        Texcoord = texcoord;
        gl_Position = vec4(position, 0.0, \
                  1.0);
    }
    ";

    let fshDFB = "
    #version "
        .to_owned() + &GlslVersion +
        "

    in vec2 Texcoord;

    out vec4 outColor;

    uniform sampler2D \
                  texFramebuffer;

    void main() {
        outColor = texture(texFramebuffer, \
                  Texcoord);
    }
    ";

    let dfb_program = glium::Program::new(
        display,
        glium::program::ProgramCreationInput::SourceCode {
            vertex_shader: &vshDFB,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: &fshDFB,
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: true,
        },
    ).unwrap();
    shader_bank.push(dfb_program);

    shader_bank
}
