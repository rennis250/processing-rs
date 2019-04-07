use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use glium;
use glium::uniforms::{UniformsStorage, UniformValue};

use {Screen, ScreenType};
use errors::{ProcessingErr, ErrorReadingIncludeLineInShader};

/// This holds information related to a custom shader that has been loaded
/// by you. It basically just allows `processing-rs` to find the associated program
/// in a Vector of programs that is managed by the Screen struct and also holds the
/// current uniform values associated with that program. To change the uniform values
/// for the program, you need to call shader_info.set().
pub struct ShaderInfo<'a> {
    shader_idx: usize,
    uniforms: UniformsStorage<'a>,
    texture_list: Vec<(String, gl::types::GLuint)>
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
            let m: [[f32; 4]; 4] = $screen.matrices.curr_matrix.into();
            let m = UniformValue::Mat4([[m[0][0], m[0][1], m[0][2], m[0][3]],
                    [m[1][0], m[1][1], m[1][2], m[1][3]],
                    [m[2][0], m[2][1], m[2][2], m[2][3]],
                    [m[3][0], m[3][1], m[3][2], m[3][3]]]);
            uniform!{MVP: m}
        }
    };
    ($screen:ident, $($uniformName:ident: $value:expr),+) => {
        {
            let m: [[f32; 4]; 4] = $screen.matrices.curr_matrix.into();
            let m = UniformValue::Mat4([[m[0][0], m[0][1], m[0][2], m[0][3]],
                    [m[1][0], m[1][1], m[1][2], m[1][3]],
                    [m[2][0], m[2][1], m[2][2], m[2][3]],
                    [m[3][0], m[3][1], m[3][2], m[3][3]]]);
            uniform!{$($uniformName: $value,)+ MVP: m}
        }
    }
}

impl<'a> ShaderInfo<'a> {
	/// Change the currently assigned uniform values for your custom shader. You will
	/// need to redefine all of the uniforms via the `create_uniforms{}` macro, even if
	/// only one uniform changes value. This really shouldn't have any performance
	/// impact on your program, so don't worry. This constraint is imposed by raw
	/// glium, so the situation wouldn't really change if you used that instead.
    pub fn set(&mut self, uni_name: &'a str, value: UniformValue<'a>) {
		self.uniforms = self.uniforms.add(uni_name, value);
	}

	/// Return the shader index that was assigned by OpenGL to your custom shader.
    pub fn get_idx(&self) -> usize {
        self.shader_idx
    }

	/// Return a reference to the uniforms that you assigned to your custom shader.
    pub fn get_uniforms(&self) -> UniformsStorage<'a> {
		self.uniforms.clone()
	}
	
	pub fn get_texture_list(&self) -> Vec<(String, gl::types::GLuint)> {
		self.texture_list.clone()
	}
	
	pub fn add_to_texture_list(&mut self, name: &str, value: gl::types::GLuint) {
		for kv in self.texture_list.iter_mut() {
			if kv.0 == name {
				*kv = (name.to_string(), value);
				return;
			}
		}
		self.texture_list.push((name.to_string(), value));
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
    pub fn load_frag_shader(
        &mut self,
        frag_filename: &str
    ) -> Result<ShaderInfo, ProcessingErr> {
        let fsh = parse_includes(frag_filename)?;
        let mut ff = File::create("full.frag").map_err(|e| ProcessingErr::FullShaderNoCreate(e))?;
        ff.write_all(fsh.as_bytes()).map_err(|e| ProcessingErr::FullShaderNoWrite(e))?;
        ff.flush().map_err(|e| ProcessingErr::FullShaderNoCreate(e))?;

        let vsh = "
    #version "
            .to_owned() + &self.glsl_version +
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

		let m: [[f32; 4]; 4] = self.matrices.curr_matrix.into();
		let m = UniformValue::Mat4([[m[0][0], m[0][1], m[0][2], m[0][3]],
			[m[1][0], m[1][1], m[1][2], m[1][3]],
			[m[2][0], m[2][1], m[2][2], m[2][3]],
			[m[3][0], m[3][1], m[3][2], m[3][3]]]);

        Ok(ShaderInfo {
            shader_idx: self.shader_bank.len() - 1,
            uniforms: UniformsStorage::new("MVP", m),
            texture_list: vec![]
        })
    }

    // pub fn load_shaders(
    //     frag_filename: &str,
    //     vertFilename: &str,
    //     self: &mut State,
    //     GLobjs: &mut GLobjStruct,
    //     shader_bank: &mut HashMap<String, u32>,
    // ) -> ShaderInfo {
    //     let mut shader_info = ShaderInfo::default();
    //     shader_info.ShaderName = rand::thread_rng().gen_ascii_chars().take(16).collect();
    //     shader_info.FragFilename = frag_filename.to_owned();
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
    pub fn shader(&mut self, which_shader: &ShaderInfo<'a>) {
        //shaderType enum - how to handle?...
        self.alternate_shader = which_shader.get_idx();
        self.using_alternate_shader = true;
        self.uniforms = Some(which_shader.get_uniforms());
        self.texture_list = Some(which_shader.get_texture_list());
        self.curr_shader = which_shader.get_idx();
    }

	/// Tell `processing-rs` to stop using any custom shaders and to return to
	/// the default shaders.
    #[inline]
    pub fn reset_shader(&mut self) {
        self.curr_shader = 0;
        self.uniforms = None;
        self.texture_list = None;
        self.using_alternate_shader = false;
    }
}

fn parse_includes(filename: &str) -> Result<String, ProcessingErr> {
    let ff = File::open(filename).map_err(|e| ProcessingErr::ShaderNotFound(e))?;

    let mut total_contents: Vec<String> = Vec::with_capacity(1);
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
            total_contents.push(contents);
        } else {
            total_contents.push(l);
        }
    }

    Ok(total_contents.join("\n"))
}
