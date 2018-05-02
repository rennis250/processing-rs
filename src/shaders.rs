use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use glium;
use glium::uniforms::Uniforms;
//use rand;
//use rand::Rng;

use {Screen, ScreenType};

#[derive(Clone, Debug)]
pub struct ShaderInfo<U: Uniforms> {
    ShaderIdx: usize,
    uniforms: Option<U>,
}

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
    pub fn new(idx: usize, uniforms: Option<U>) -> Self {
        ShaderInfo {
            ShaderIdx: idx,
            uniforms: uniforms,
        }
    }

    pub fn set(&mut self, uniforms: U) {
        self.uniforms = Some(uniforms);
    }

    pub fn get_idx(&self) -> usize {
        self.ShaderIdx
    }

    pub fn get_uniforms(&self) -> &U {
        self.uniforms.as_ref().unwrap()
    }
}

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

    pub fn load_frag_shader<U: Uniforms>(
        &mut self,
        fragFilename: &str,
        uniforms: U,
    ) -> ShaderInfo<U> {
        let fsh = parse_includes(fragFilename);
        let mut ff = File::create("full.frag").unwrap();
        ff.write_all(fsh.as_bytes()).unwrap();
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
                glium::Program::new(
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
                ).unwrap()
            }
            ScreenType::Headless(ref d) => {
                glium::Program::new(
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
                ).unwrap()
            }
        };
        self.shader_bank.push(program);

        ShaderInfo {
            ShaderIdx: self.shader_bank.len() - 1,
            uniforms: Some(uniforms),
        }
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

    #[inline]
    pub fn shader<U: Uniforms>(&mut self, whichShader: &ShaderInfo<U>) {
        //shaderType enum - how to handle?...
        self.AlternateShader = whichShader.ShaderIdx;
        self.UsingAlternateShader = true;
        self.CurrShader = whichShader.ShaderIdx;
    }

    #[inline]
    pub fn reset_shader(&mut self) {
        self.CurrShader = 0;
        self.UsingAlternateShader = false;
    }
}

fn parse_includes(filename: &str) -> String {
    let ff = File::open(filename).unwrap();

    let mut totalContents: Vec<String> = Vec::with_capacity(1);
    for line in BufReader::new(ff).lines() {
        let l = line.unwrap();
        if l.starts_with("#include") {
            let mut lparts = l.split('<');
            let ifname = lparts.nth(1).unwrap();
            let ln = ifname.len();
            let mut dat = File::open(&ifname[0..ln - 1]).unwrap();
            let mut contents = String::new();
            dat.read_to_string(&mut contents).expect(
                "something went wrong reading the file",
            );
            totalContents.push(contents);
        } else {
            totalContents.push(l);
        }
    }

    totalContents.join("\n")
}
