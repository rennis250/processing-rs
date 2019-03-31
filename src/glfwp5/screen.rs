#[cfg(feature = "glfw")]

use std::str;
use std::sync::mpsc::Receiver;

#[cfg(target_os = "macos")]
use cocoa::foundation::{NSProcessInfo, NSString};
#[cfg(target_os = "macos")]
use cocoa::base::nil;

use owning_ref;
use gl;
use glium;
use glium::backend::Facade;
use glium::{Surface, GlObject};
use glfw;
use glfw::{Action, Context};

use glfwp5::backend::Display;
use Matrix4;
use {Screen, GLmatStruct, FBtexs, DFBFDVertex};
use ScreenType;
use errors::ProcessingErr;

#[cfg(target_os = "macos")]
use mac_priority;

impl<'a> Screen<'a> {
	/// If you are using glfw as a backend, then you need to call this before calling
	/// Screen::new(). It is required by the glfw library. The result will be a
	/// glfw::Glfw struct that should be passed to Screen::new() as input, which will
	/// handle the rest of the initialization.
    pub fn init() -> Result<glfw::Glfw, ProcessingErr> {
        match glfw::init(glfw::FAIL_ON_ERRORS) {
			Ok(res) => Ok(res),
			Err(e) => match e {
				glfw::InitError::AlreadyInitialized => Err(ProcessingErr::GLFWAlreadyInited),
				glfw::InitError::Internal => Err(ProcessingErr::GLFWInternal)
			}
        }
    }

	/// Create a new Screen struct with a given width and height. Also, specify if
	/// the Screen should be fullscreen, if it should preserve aspect ratio on wide
	/// monitors, and if it should be synchronize to the refresh rate of the monitor
	/// (this should always be true, except in rare circumstances when you need really
	/// high draw rates, such as when doing intense raymarching in a fragment shader).
	///
	/// In the case of glfw as a backend, you will also need to provide the glfw::Glfw
	/// struct that is returned by Screen::init().
	///
	/// It is necessary to call this function before everything else. It's what gets
	/// the whole show going. Once you have a Screen, you can then create shapes,
	/// load textures, draw, check for user input, etc.
	///
	/// Screen setup tries to choose a number of glutin and glium defaults that will
	/// satisfy most users, especially those that want speed but still have a
	/// visually pleasing display of shapes with good color fidelity, if possible.
    pub fn new(
        width: u32,
        height: u32,
        mut glfw: glfw::Glfw,
        fullscreen: bool,
        preserve_aspect_ratio: bool,
        headless: bool
    ) -> Result<Screen<'a>, ProcessingErr> {
        #[cfg(target_os = "macos")] mac_priority();

        glfw.window_hint(glfw::WindowHint::Visible(!headless));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        // try to activate 10 bpc support
        glfw.window_hint(glfw::WindowHint::RedBits(Some(10)));
        glfw.window_hint(glfw::WindowHint::GreenBits(Some(10)));
        glfw.window_hint(glfw::WindowHint::BlueBits(Some(10)));
        glfw.window_hint(glfw::WindowHint::AlphaBits(Some(2)));

        // anti-aliasing by default
        glfw.window_hint(glfw::WindowHint::Samples(Some(0)));

        let events_loop: Receiver<(f64, glfw::WindowEvent)>;
        let mut w = width;
        let mut h = height;
        let window;
        if fullscreen {
            let (mut win, e) = glfw.create_window(w, h, "processingrs", glfw::WindowMode::Windowed)
                .ok_or(ProcessingErr::GLFWWindowNoCreate)?;
            glfw.with_primary_monitor_mut(|_: &mut _, m: Option<&glfw::Monitor>| {
                let monitor = m.expect("Did not get access to a monitor.");

                let mode = monitor.get_video_mode().expect("Did not get access to the monitors preferred video mode.");

                w = mode.width;
                h = mode.height;

                win.set_monitor(
                    glfw::WindowMode::FullScreen(&monitor),
                    0,
                    0,
                    mode.width,
                    mode.height,
                    Some(mode.refresh_rate),
                );
            });
            window = win;
            events_loop = e;
        } else {
            let (mut win, e) = glfw.create_window(w, h, "processingrs", glfw::WindowMode::Windowed)
                .ok_or(ProcessingErr::GLFWWindowNoCreate)?;
            window = win;
            events_loop = e;
        }

        // let frame_rate = 0;
        // if frame_rate == 0 {
        glfw.window_hint(glfw::WindowHint::RefreshRate(Some(60)));
        // } else if frame_rate == 5000 {
        // let system determine frame rate
        // } else {
        // glfw.window_hint(glfw::WindowHint::RefreshRate(Some(frame_rate as u32)));
        // }

        let display = Display::new(window)?;

        display.gl_window_mut().set_key_polling(true);
        display.gl_window_mut().make_current();

        // Load the OpenGL function pointers
        // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
        gl::load_with(|symbol| {
            (*display.gl_window_mut()).get_proc_address(symbol) as *const _
        });

        let mut glsl_version;
        {
            let glt = display.get_context().get_opengl_version();
            glsl_version = format!("{}{:0<2}", glt.1, glt.2).to_owned();
            println!("OpenGL version {}", glsl_version);
        }

        display.gl_window_mut().show();
        display.gl_window_mut().set_size(w as i32, h as i32);

        let (fbw, fbh) = display.get_framebuffer_dimensions();
        let fb_size = vec![fbw, fbh];

        unsafe {
            gl::Viewport(
                0,
                0,
                fb_size[0] as gl::types::GLsizei,
                fb_size[1] as gl::types::GLsizei,
            );
        }

        // if frame_rate == 5000 {
        // let system determine frame rate
        // } else {
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        // }

        let aspect_ratio = w as f32 / h as f32;

        let fb_texture = glium::texture::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            w,
            h,
        ).map_err(|e| ProcessingErr::TextureNoCreate(e))?;
        let fbid = fb_texture.get_id();
        let depthtexture = glium::texture::DepthTexture2d::empty_with_format(
            &display,
            glium::texture::DepthFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            w,
            h,
        ).map_err(|e| ProcessingErr::TextureNoCreate(e))?;
        let oh = owning_ref::OwningHandle::new_with_fn(
            Box::new(FBtexs {
                fbtex: fb_texture,
                depthtexture: depthtexture,
            }),
            |v| unsafe {
                Box::new(
                    glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                        &display,
                        &(*v).fbtex,
                        &(*v).depthtexture,
                    ).expect("Could not create a SimpleFrameBuffer with attached DepthBuffer. Please check your graphics card, drivers, and OS."),
                )
            },
        );
        let fbtexture = unsafe {
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
            smooth: None,
            ..Default::default()
        };

        // start with arrow cursor by default, as expected
        let curr_cursor = glfw::Cursor::standard(glfw::StandardCursor::Arrow);
        display.gl_window_mut().set_cursor(Some(curr_cursor));

        // if !fonts_initialized {
        // setupFontCharacters()
        // }
        // fonts_initialized = true

        // glCheckError("Screen initilization.");

        // by default, use system fonts that are known to basically always be available
        let mut font_face = "".to_owned();
        if cfg!(target_os = "windows") {
            font_face = "C:/Windows/Fonts/arial.ttf".to_owned();
        } else if cfg!(target_os = "linux") {
            font_face = "/usr/share/fonts/ttf-dejavu-ib/DejaVuSansMono.ttf".to_owned();
        } else if cfg!(target_os = "macos") {
            font_face = "/Users/rje/Library/Fonts/Go-Regular.ttf".to_owned();
        }

        let shader_bank = init_shaders(&display, &glsl_version)?;

        let vertex1 = DFBFDVertex {
            position: [-1.0, -1.0],
            texcoord: [0.0, 0.0],
        };
        let vertex2 = DFBFDVertex {
            position: [1.0, -1.0],
            texcoord: [1.0, 0.0],
        };
        let vertex3 = DFBFDVertex {
            position: [1.0, 1.0],
            texcoord: [1.0, 1.0],
        };
        let vertex4 = DFBFDVertex {
            position: [-1.0, 1.0],
            texcoord: [0.0, 1.0],
        };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        let fb_shape_buffer = glium::VertexBuffer::new(&display, &shape)
        	.map_err(|e| ProcessingErr::VBNoCreate(e))?;
        let fb_index_buffer = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &[0u16, 1, 2, 0, 2, 3],
        ).map_err(|e| ProcessingErr::IBNoCreate(e))?;

        display.gl_window_mut().swap_buffers();
        glfw.poll_events();

        if display.gl_window().should_close() {
            // drop(window);
            panic!("Window prematurely terminated.");
        }

        Ok(Screen {
            // start with default identity matrix, as expected.
            matrices: GLmatStruct {
                curr_matrix: Matrix4::new(
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
                matrix_stack: vec![
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
            fbo: oh,
            fbtexture: fbtexture,
            fb_shape_buffer: fb_shape_buffer,
            fb_index_buffer: fb_index_buffer,
            display: if headless { ScreenType::Headless(display) } else { ScreenType::Window(display) },
            glfw: glfw,
            events_loop: events_loop,
            draw_params: draw_params,
            bg_col: vec![0.8f32, 0.8, 0.8, 0.8],
            fill_stuff: true,
            fill_col: vec![1.0f32, 1.0, 1.0, 1.0],
            stroke_stuff: true,
            stroke_col: vec![0.0f32, 0.0, 0.0, 1.0],
            tint_stuff: false,
            tint_col: vec![1.0f32, 1.0, 1.0, 1.0],
            shader_bank: shader_bank,
            draw_texture: false,
            aspect_ratio: aspect_ratio,
            preserve_aspect_ratio: preserve_aspect_ratio,
            fb_size: fb_size,
            stroke_weight: 1.0,
            font_face: font_face,
            text_size: 0.4,
            height: height,
            width: width,
            left: -1f32,
            right: 1f32,
            top: 1f32,
            bottom: -1f32,
            c_mode: "RGB".to_owned(),
            title: "processing-rs".to_owned(),
            ellipse_mode: "CENTER".to_owned(),
            rect_mode: "CORNER".to_owned(),
            shape_mode: "CORNER".to_owned(),
            image_mode: "CORNER".to_owned(),
            frame_rate: 60,
            frame_count: 0,
            fonts_initialized: false,
            curr_shader: 0,
            curr_cursor: glfw::StandardCursor::Arrow,
            wrap: glium::uniforms::SamplerWrapFunction::Repeat,
            curr_texture: None,
            alternate_shader: 1 << 20,
            using_alternate_shader: false,
            glsl_version: glsl_version,
            drew_points: false,
            keypressed: None,
            mousepressed: None,
            mousereleased: None,
            mousepos: (-100., -100.),
            headless: headless,
        })
    }

	
	/// Once you have finished drawing a number of shapes to the screen, you will need
	/// to call screen.reveal() for the result to be viewable on the monitor. This is
	/// because `processing-rs` uses double-buffering, whereby all of the drawing 
	/// happens on a separate, hidden buffer and once that is done, it is transferred
	/// to a viewable, monitor buffer. This is standard practice in graphics programming,
	/// since it makes drawing faster and reduces screen tearing.
    #[inline]
    pub fn reveal(&mut self) -> Result<(), ProcessingErr> {
        let mut target = match self.display {
            ScreenType::Window(ref d) => d.draw(),
            ScreenType::Headless(ref d) => d.draw(),
        };
        {
            let uniforms = uniform! { texFramebuffer: &self.fbtexture };
            let p = &self.shader_bank[3];
            target
                .draw(
                    &self.fb_shape_buffer,
                    &self.fb_index_buffer,
                    p,
                    &uniforms,
                    &Default::default(),
                )
                .map_err(|e| ProcessingErr::DrawFailed(e))?;
            target.finish().map_err(|e| ProcessingErr::SwapFailed(e))?;
        }

        let mut kp = None;
        let mut mp = None;
        let mut mr = None;
        let mut mpos = (-100., -100.);
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events_loop) {
            match event {
                // glfw::WindowEvent::Pos(x, y) => {
                //     window.set_title(&format!("Time: {:?}, Window pos: ({:?}, {:?})", time, x, y))
                // }
                // glfw::WindowEvent::Size(w, h) => {
                //     window.set_title(&format!("Time: {:?}, Window size: ({:?}, {:?})", time, w, h))
                // }
                glfw::WindowEvent::Close => panic!("need a smoother way to quit..."),
                // glfw::WindowEvent::Refresh => {
                //     println!("Time: {:?}, Window refresh callback triggered.", time)
                // }
                // glfw::WindowEvent::Focus(true) => {
                // println!("Time: {:?}, Window focus gained.", time)
                // }
                // glfw::WindowEvent::Focus(false) => println!("Time: {:?}, Window focus lost.", time),
                // glfw::WindowEvent::Iconify(true) => {
                //     println!("Time: {:?}, Window was minimised", time)
                // }
                // glfw::WindowEvent::Iconify(false) => {
                //     println!("Time: {:?}, Window was maximised.", time)
                // }
                // glfw::WindowEvent::FramebufferSize(w, h) => {
                //     println!("Time: {:?}, Framebuffer size: ({:?}, {:?})", time, w, h)
                // }
                // glfw::WindowEvent::Char(character) => {
                //     println!("Time: {:?}, Character: {:?}", time, character)
                // }
                // glfw::WindowEvent::CharModifiers(character, mods) => {
                //     println!("Time: {:?}, Character: {:?}, Modifiers: [{:?}]",
                //              time,
                //              character,
                //              mods)
                // }
                glfw::WindowEvent::MouseButton(btn, action, _) => {
                    // println!("Time: {:?}, Button: {:?}, Action: {:?}, Modifiers: [{:?}]",
                    // time,
                    // glfw::DebugAliases(btn),
                    // action,
                    // mods)
                    match action {
                        Action::Press => mp = Some(btn),
                        Action::Release => mr = Some(btn),
                        _ => (),
                    }
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    //     window.set_title(&format!("Time: {:?}, Cursor position: ({:?}, {:?})",
                    //                               time,
                    //                               xpos,
                    //                               ypos))
                    mpos = (xpos as f64, ypos as f64);
                }
                // glfw::WindowEvent::CursorEnter(true) => {
                //     println!("Time: {:?}, Cursor entered window.", time)
                // }
                // glfw::WindowEvent::CursorEnter(false) => {
                //     println!("Time: {:?}, Cursor left window.", time)
                // }
                // glfw::WindowEvent::Scroll(x, y)                   => window.set_title(&format!("Time: {:?}, Scroll offset: ({:?}, {:?})", time, x, y)),
                glfw::WindowEvent::Key(key, _, _, _) => {
                    // println!("Time: {:?}, Key: {:?}, ScanCode: {:?}, Action: {:?}, Modifiers: [{:?}]", time, key, scancode, action, mods);
                    kp = Some(key);
                    // match (key, action) {
                    // (Key::Escape, Action::Press) => window.set_should_close(true),
                    // (Key::R, Action::Press) => {
                    // Resize should cause the window to "refresh"
                    // let (window_width, window_height) = window.get_size();
                    // window.set_size(window_width + 1, window_height);
                    // window.set_size(window_width, window_height);
                    // }
                    // _ => {}
                    // }
                }
                // glfw::WindowEvent::FileDrop(paths) => {
                // println!("Time: {:?}, Files dropped: {:?}", time, paths)
                // }
                _ => (),
            }
        }
        // self.events_loop.poll_events(|event| match event {
        // glutin::Event::WindowEvent { event, .. } => {
        // match event {
        // glutin::WindowEvent::Closed => panic!("need a smoother way to quit..."),
        // glutin::WindowEvent::KeyboardInput { input, .. }
        // if glutin::ElementState::Pressed == input.state => {
        // match input.virtual_keycode {
        // Some(b) => {
        // kp = Some(b);
        // }
        // _ => (),
        // }
        // }
        // glutin::WindowEvent::MouseInput { state: s, button: b, .. }
        // if glutin::ElementState::Pressed == s => {
        // mp = Some(b);
        // }
        // glutin::WindowEvent::MouseInput { state: s, button: b, .. }
        // if glutin::ElementState::Released == s => {
        // mr = Some(b);
        // }
        // glutin::WindowEvent::MouseMoved { position, .. } => {
        // mpos = position;
        // }
        // _ => (),
        // }
        // }
        // _ => (),
        // });

        self.keypressed = kp;
        self.mousepressed = mp;
        self.mousereleased = mr;
        self.mousepos = mpos;

        self.frame_count += 1;
        
        Ok(())
    }

	/// This will safely close a window and drop the Screen struct associated with it.
	/// Currently unimplemented, so for now, to close a window, you have to quit the
	/// running program.
    pub fn end_drawing(self) {
        match self.display {
            ScreenType::Window(ref d) => (*d).gl_window_mut().set_should_close(true),
            ScreenType::Headless(ref d) => (*d).gl_window_mut().set_should_close(true),
        };
        // unsafe {
        // glfw::ffi::glfwTerminate();
        // }
        // panic!("need a smoother way to end...");
        // FontState.text.Release();
        // FontState.font.Release();
    }
}

// pub fn drawing_window(glfw: &mut glfw::Glfw, window: &mut glfw::Window) {
//     glfw.make_context_current(Some(window));
//     window.show();
// }

pub fn init_shaders(display: &Display, glsl_version: &str) -> Result<Vec<glium::program::Program>, ProcessingErr> {
    let mut shader_bank = Vec::new();

    // basicShapes
    let vsh_bs = "
    #version "
        .to_owned() + &glsl_version +
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

    let fsh_bs = "
    #version "
        .to_owned() + &glsl_version +
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
            vertex_shader: &vsh_bs,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: &fsh_bs,
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: true,
        },
    ).map_err(|e| ProcessingErr::ShaderCompileFail(e))?;
    shader_bank.push(bs_program);

    // texturedShapes
    let vsh_ts = "
    #version "
        .to_owned() + &glsl_version +
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

    let fsh_ts = "
    #version "
        .to_owned() + &glsl_version +
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
            vertex_shader: &vsh_ts,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: &fsh_ts,
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: true,
        },
    ).map_err(|e| ProcessingErr::ShaderCompileFail(e))?;
    shader_bank.push(ts_program);

    // fontDrawing
    let vsh_fd = "
    #version "
        .to_owned() + &glsl_version +
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

    let fsh_fd = "
    #version "
        .to_owned() + &glsl_version +
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
            vertex_shader: &vsh_fd,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: &fsh_fd,
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: true,
        },
    ).map_err(|e| ProcessingErr::ShaderCompileFail(e))?;
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
    let vsh_dfb = "
    #version "
        .to_owned() + &glsl_version +
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

    let fsh_dfb = "
    #version "
        .to_owned() + &glsl_version +
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
            vertex_shader: &vsh_dfb,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: &fsh_dfb,
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: true,
        },
    ).map_err(|e| ProcessingErr::ShaderCompileFail(e))?;
    shader_bank.push(dfb_program);

    Ok(shader_bank)
}
