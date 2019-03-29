use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use glium;
use glium::uniforms::Uniforms;
//use rand;
//use rand::Rng;

use {Screen, ScreenType};
use errors::{ProcessingErr, ErrorReadingIncludeLineInShader};

/// This holds information related to a custom shader that has been loaded
/// by you. It basically just allows `processing-rs` to find the associated program
/// in a Vector of programs that is managed by the Screen struct and also holds the
/// current uniform values associated with that program. To change the uniform values
/// for the program, you need to call shader_info.set().
#[derive(Clone, Debug)]
pub struct ShaderInfo<U: Uniforms> {
    ShaderIdx: usize,
    uniforms: Option<U>,
}

/// This macro rolls your custom uniforms for your custom shader into the uniform
/// format expected by glium. It makes things more convienent, because it will add the
/// current MVP transformation matrix that is shared across all shaders for consistent
/// rendering to the screen. If your shader will be rendering 2-dimensional or
/// 3-dimensional shapes to the screen, you will probably want to make use of this
/// matrix, which will be available as `uniform mat4 MVP` in your custom shader.
#[macro_export]
macro_rules! create_uniforms {
    ($screen:ident) => {
        {
            let m: [[f32; 4]; 4] = $screen.matrices.currMatrix.into();
            let m = [[m[0][0], m[0][1], m[0][2], m[0][3]],
                    [m[1][0], m[1][1], m[1][2], m[1][3]],
                    [m[2][0], m[2][1], m[2][2], m[2][3]],
                    [m[3][0], m[3][1], m[3][2], m[3][3]]];
            uniform!{MVP: m}
        }
    };
    ($screen:ident, $($uniformName:ident: $value:expr),+) => {
        {
            let m: [[f32; 4]; 4] = $screen.matrices.currMatrix.into();
            let m = [[m[0][0], m[0][1], m[0][2], m[0][3]],
                    [m[1][0], m[1][1], m[1][2], m[1][3]],
                    [m[2][0], m[2][1], m[2][2], m[2][3]],
                    [m[3][0], m[3][1], m[3][2], m[3][3]]];
            uniform!{$($uniformName: $value,)+ MVP: m}
        }
    }
}

impl<U: Uniforms> ShaderInfo<U> {
	/// Create a new ShaderInfo struct that contains your uniforms and the shader
	/// index that was returned by OpenGL. You should basically not need to use this
	/// as a normal user, since screen.load_frag_shader() and screen.load_shaders()
	/// use it internally and handle the details for you. The uniforms will need to
	/// have been created by the macro `create_uniforms{}` provided by this crate.
	/// Please see its documentation and use it whenever you need to pass uniforms
	/// to shaders in `processing-rs`.
    pub fn new(idx: usize, uniforms: Option<U>) -> Self {
        ShaderInfo {
            ShaderIdx: idx,
            uniforms: uniforms,
        }
    }

	/// Change the currently assigned uniform values for your custom shader. You will
	/// need to redefine all of the uniforms via the `create_uniforms{}` macro, even if
	/// only one uniform changes value. This really shouldn't have any performance
	/// impact on your program, so don't worry. This constraint is imposed by raw
	/// glium, so the situation wouldn't really change if you used that instead.
    pub fn set(&mut self, uniforms: U) {
        self.uniforms = Some(uniforms);
    }

	/// Return the shader index that was assigned by OpenGL to your custom shader.
    pub fn get_idx(&self) -> usize {
        self.ShaderIdx
    }

	/// Return a reference to the uniforms that you assigned to your custom shader.
    pub fn get_uniforms(&self) -> &U { // currently, there is no way for uniforms to be
    									// None, so I feel safe with this as is for now...
        self.uniforms.as_ref().unwrap()
    }
}

/// Processing has three ways to feed data through a shader: as points, as lines,
/// and as triangles. The distinction is not as necessary here and may go away in time.
/// Regardless, this hasn't been actually implemented yet, but if you want more info,
/// check the official Processing reference for now.
enum DrawType {
    POINT,
    LINE,
    TRIANGLE,
}

impl<'a> Screen<'a> {
    // pub fn shader(&mut self, shader_name: &str) {
    // gl::Uniform3f(gl::GetUniformLocation(shader_bank["fontDrawing"], "textColor"), GLfloat(state.fillCol[1].r), GLfloat(state.fillCol[1].g), GLfloat(state.fillCol[1].b))
    // unsafe {
    // gl::Uniform3f(
    // gl::GetUniformLocation(
    // *shader_bank.get("fontDrawing").unwrap(),
    // CString::new("textColor").unwrap().as_ptr(),
    // ),
    // 0.0,
    // 0.0,
    // 0.0,
    // );
    // }
    // } else if shader_name == "drawFramebuffer" {
    // self.CurrShader = "drawFramebuffer".to_owned();
    // }
    // }

    // fn remove_all_shaders(shader_bank: &HashMap<String, u32>) {
    //     for (_, &p) in shader_bank {
    //         unsafe {
    //             gl::UseProgram(p);
    //         }
    //         let mut c = 0;
    //         let mut asa = vec![0u32; 1];
    //         unsafe {
    //             gl::GetAttachedShaders(p, 10, &mut c, mem::transmute(&asa[0]));
    //             for s in asa {
    //                 gl::DetachShader(p, s);
    //                 gl::DeleteShader(s);
    //             }
    //             gl::UseProgram(0);
    //             gl::DeleteProgram(p);
    //         }
    //     }
    // }

	/// Load your custom fragment shader and the initial values of the uniforms that
	/// go with it. You will get a ShaderInfo struct in return, through which you
	/// can update the uniform values and which can be used to tell `processing-rs` to
	/// use your custom shader, instead of the standard ones it provides. The ShaderInfo
	/// struct that you receive should be the input to screen.shader(), which allows
	/// you to use the custom shader.
	///
	/// This function provides an additional convienence, in that if your shader
	/// contains a line like `#include <auxiliary.frag>` it will process that and
	/// load the named file into that exact location of your fragment shader. This
	/// allows you to keep things a bit more modular and managable. Please note though,
	/// it will do this no matter where you put the line, so remain aware. You probably
	/// only want to use this convienence at outermost scope, but the choice is yours.
    pub fn load_frag_shader<U: Uniforms>(
        &mut self,
        fragFilename: &str,
        uniforms: U,
    ) -> Result<ShaderInfo<U>, ProcessingErr> {
        let fsh = parse_includes(fragFilename)?;
        let mut ff = File::create("full.frag").map_err(|e| ProcessingErr::FullShaderNoCreate(e))?;
        ff.write_all(fsh.as_bytes()).map_err(|e| ProcessingErr::FullShaderNoWrite(e))?;
        ff.flush();

        let vsh = "
    #version "
            .to_owned() + &self.GlslVersion +
            "

    in vec3 position;
    in vec4 color;

    out vec4 vColor;
    out vec3 Position;

    uniform \
                   mat4 MVP;

    void main() {
        vColor = color;
        Position = position;
        gl_Position = MVP \
                   * vec4(position, 1.0);
    }
    ";

        let program = match self.display {
            ScreenType::Window(ref d) => {
                match glium::Program::new(
                    d,
                    glium::program::ProgramCreationInput::SourceCode {
                        vertex_shader: &vsh,
                        tessellation_control_shader: None,
                        tessellation_evaluation_shader: None,
                        geometry_shader: None,
                        fragment_shader: &fsh,
                        transform_feedback_varyings: None,
                        outputs_srgb: true,
                        uses_point_size: true,
                    },
                ) {
                	Ok(res) => res,
                	Err(e) => return Err(ProcessingErr::ShaderCompileFail(e))
                }
            }
            ScreenType::Headless(ref d) => {
                match glium::Program::new(
                    d,
                    glium::program::ProgramCreationInput::SourceCode {
                        vertex_shader: &vsh,
                        tessellation_control_shader: None,
                        tessellation_evaluation_shader: None,
                        geometry_shader: None,
                        fragment_shader: &fsh,
                        transform_feedback_varyings: None,
                        outputs_srgb: true,
                        uses_point_size: true,
                    },
                ) {
                	Ok(res) => res,
                	Err(e) => return Err(ProcessingErr::ShaderCompileFail(e))
                }
            }
        };
        self.shader_bank.push(program);

        Ok(ShaderInfo {
            ShaderIdx: self.shader_bank.len() - 1,
            uniforms: Some(uniforms),
        })
    }

    // pub fn load_shaders(
    //     fragFilename: &str,
    //     vertFilename: &str,
    //     self: &mut State,
    //     GLobjs: &mut GLobjStruct,
    //     shader_bank: &mut HashMap<String, u32>,
    // ) -> ShaderInfo {
    //     let mut shader_info = ShaderInfo::default();
    //     shader_info.ShaderName = rand::thread_rng().gen_ascii_chars().take(16).collect();
    //     shader_info.FragFilename = fragFilename.to_owned();
    //     shader_info.VertFilename = vertFilename.to_owned();
    //
    //     let fsh = parse_frag_includes(&shader_info);
    //     let vsh = parse_vert_includes(&shader_info);
    //
    //     let program = new_program(&vsh, &fsh);
    //     shader_bank.insert(shader_info.ShaderName.clone(), program);
    //     shader_info.Program = program;
    //     unsafe {
    //         gl::UseProgram(program);
    //
    //         let mut x = 0;
    //         gl::GenVertexArrays(1, &mut x);
    //         GLobjs.VAOs.push(x);
    //         gl::BindVertexArray(*GLobjs.VAOs.last().unwrap());
    //         shader_info.VaoInd = GLobjs.VAOs.len() as i32 - 1;
    //         gl::GenBuffers(1, &mut x);
    //         GLobjs.EBOs.push(x);
    //         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *GLobjs.EBOs.last().unwrap());
    //         shader_info.EboInd = GLobjs.EBOs.len() as i32 - 1;
    //
    //         gl::GenBuffers(1, &mut x);
    //         GLobjs.POSVBOs.push(x);
    //         gl::BindBuffer(gl::ARRAY_BUFFER, *GLobjs.POSVBOs.last().unwrap());
    //         shader_info.PosInd = GLobjs.POSVBOs.len() as i32 - 1;
    //         let positionAttribute =
    //             gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr()) as GLuint;
    //         gl::EnableVertexAttribArray(positionAttribute);
    //         gl::VertexAttribPointer(
    //             positionAttribute,
    //             3,
    //             gl::FLOAT,
    //             gl::FALSE as GLboolean,
    //             3 * 4,
    //             ptr::null(),
    //         );
    //
    //         gl::GenBuffers(1, &mut x);
    //         GLobjs.COLVBOs.push(x);
    //         gl::BindBuffer(gl::ARRAY_BUFFER, *GLobjs.COLVBOs.last().unwrap());
    //         shader_info.ColInd = GLobjs.COLVBOs.len() as i32 - 1;
    //         let colorAttribute =
    //             gl::GetAttribLocation(program, CString::new("color").unwrap().as_ptr()) as GLuint;
    //         gl::EnableVertexAttribArray(colorAttribute);
    //         gl::VertexAttribPointer(
    //             colorAttribute,
    //             4,
    //             gl::FLOAT,
    //             gl::FALSE as GLboolean,
    //             4 * 4,
    //             ptr::null(),
    //         );
    //     }
    //     shader("basicShapes", self, GLobjs, shader_bank);
    //
    //     shader_info
    // }

	/// Tell `processing-rs` to use your custom shader instead of one of the standards
	/// it provides. This only accepts ShaderInfo structs, which are output by
	/// screen.load_frag_shader() and screen.load_shaders().
    #[inline]
    pub fn shader<U: Uniforms>(&mut self, whichShader: &ShaderInfo<U>) {
        //shaderType enum - how to handle?...
        self.AlternateShader = whichShader.ShaderIdx;
        self.UsingAlternateShader = true;
        self.CurrShader = whichShader.ShaderIdx;
    }

	/// Tell `processing-rs` to stop using any custom shaders and to return to
	/// the default shaders.
    #[inline]
    pub fn reset_shader(&mut self) {
        self.CurrShader = 0;
        self.UsingAlternateShader = false;
    }
}

fn parse_includes(filename: &str) -> Result<String, ProcessingErr> {
    let ff = File::open(filename).map_err(|e| ProcessingErr::ShaderNotFound(e))?;

    let mut totalContents: Vec<String> = Vec::with_capacity(1);
    let mut line_num = 0;
    for line in BufReader::new(ff).lines() {
    	line_num += 1;
        let l = line.map_err(|e| ProcessingErr::ErrorReadingShader(line_num, e))?;
        if l.starts_with("#include") {
            let mut lparts = l.split('<');
            let ifname = lparts.nth(1).ok_or(
            	ProcessingErr::ErrorReadingShader(line_num,
            		std::io::Error::new(std::io::ErrorKind::InvalidInput,
            			ErrorReadingIncludeLineInShader::new(
            				"It is possible that the name for the include file is missing."
            			)
            		)
            	)
            )?;
            let ln = ifname.len();
            let mut dat = File::open(&ifname[0..ln - 1]).map_err(|e| ProcessingErr::IncludeNotFound(e))?;
            let mut contents = String::new();
            dat.read_to_string(&mut contents).map_err(|e| ProcessingErr::ErrorReadingInclude(e))?;
            totalContents.push(contents);
        } else {
            totalContents.push(l);
        }
    }

    Ok(totalContents.join("\n"))
}
