use glium;

use Screen;

impl<'a> Screen<'a> {
    pub fn blend_mode(&mut self, mode: &str) {
        if mode == "REPLACE" {
            // glBlendEquation(GL_FUNC_ADD);
            // glBlendFunc(GL_ONE, GL_ZERO);
            self.draw_params.blend = glium::Blend {
                color: glium::BlendingFunction::AlwaysReplace,
                alpha: glium::BlendingFunction::AlwaysReplace,
                constant_value: (1.0, 1.0, 1.0, 1.0),
            };
        } else if mode == "BLEND" {
            // glBlendEquationSeparate(GL_FUNC_ADD, GL_FUNC_ADD);
            // glBlendFuncSeparate(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA, GL_ONE, GL_ONE);
            self.draw_params.blend = glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            };
        } else if mode == "ADD" {
            // glBlendEquationSeparate(GL_FUNC_ADD, GL_FUNC_ADD);
            // glBlendFuncSeparate(GL_SRC_ALPHA, GL_ONE, GL_ONE, GL_ONE);
            self.draw_params.blend = glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::One,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            };
        } else if mode == "SUBTRACT" {
            // glBlendEquationSeparate(GL_FUNC_REVERSE_SUBTRACT, GL_FUNC_ADD);
            // glBlendFuncSeparate(GL_SRC_ALPHA, GL_ONE, GL_ONE, GL_ONE);
            self.draw_params.blend = glium::Blend {
                color: glium::BlendingFunction::ReverseSubtraction {
                    source: glium::LinearBlendingFactor::SourceAlpha,
                    destination: glium::LinearBlendingFactor::One,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            };
        } else if mode == "LIGHTEST" {
            // glBlendEquationSeparate(GL_FUNC_MAX, GL_FUNC_ADD);
            // glBlendFuncSeparate(GL_ONE, GL_ONE, GL_ONE, GL_ONE);
            self.draw_params.blend = glium::Blend {
                // color: glium::BlendingFunction::Addition {
                // source: glium::LinearBlendingFactor::One,
                // destination: glium::LinearBlendingFactor::One,
                // },
                color: glium::BlendingFunction::Max,
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            };
        } else if mode == "DARKEST" {
            // glBlendEquationSeparate(GL_FUNC_MIN, GL_FUNC_ADD);
            // glBlendFuncSeparate(GL_ONE, GL_ONE, GL_ONE, GL_ONE);
            self.draw_params.blend = glium::Blend {
                // color: glium::BlendingFunction::Addition {
                // source: glium::LinearBlendingFactor::One,
                // destination: glium::LinearBlendingFactor::One,
                // },
                color: glium::BlendingFunction::Min,
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            };
        } else if mode == "EXCLUSION" {
            // glBlendEquationSeparate(GL_FUNC_ADD, GL_FUNC_ADD);
            // glBlendFuncSeparate(GL_ONE_MINUS_DST_COLOR, GL_ONE_MINUS_SRC_COLOR, GL_ONE, GL_ONE);
            self.draw_params.blend = glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::OneMinusDestinationColor,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            };
        } else if mode == "MULTIPLY" {
            // glBlendEquationSeparate(GL_FUNC_ADD, GL_FUNC_ADD);
            // glBlendFuncSeparate(GL_ZERO, GL_SRC_COLOR, GL_ONE, GL_ONE);
            self.draw_params.blend = glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::Zero,
                    destination: glium::LinearBlendingFactor::SourceColor,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            };
        } else if mode == "SCREEN" {
            // glBlendEquationSeparate(GL_FUNC_ADD, GL_FUNC_ADD);
            // glBlendFuncSeparate(GL_ONE_MINUS_DST_COLOR, GL_ONE, GL_ONE, GL_ONE);
            self.draw_params.blend = glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::OneMinusDestinationColor,
                    destination: glium::LinearBlendingFactor::One,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            };
        }
    }
}

// createGraphics
