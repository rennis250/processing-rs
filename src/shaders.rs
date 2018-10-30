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

    pub fn load_shaders<U: Uniforms>(
        &mut self,
        vertFilename: &str,
        fragFilename: &str,
        uniforms: U,
    ) -> ShaderInfo<U> {
        let vsh = parse_includes(vertFilename);
        let mut vf = File::create("full.vert").unwrap();
        vf.write_all(vsh.as_bytes()).unwrap();
        vf.flush();

        let fsh = parse_includes(fragFilename);
        let mut ff = File::create("full.frag").unwrap();
        ff.write_all(fsh.as_bytes()).unwrap();
        ff.flush();

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
