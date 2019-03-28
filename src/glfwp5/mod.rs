#[cfg(feature = "glfw")]

// This module is here for those that need to use glfw instead of glutin as a backend.
// You should generally stick to the glutin backend, as tests have shown it to be
// faster and it is written in pure Rust, but if you need glfw, then request the 
// glfw feature in your Cargo.toml and make sure to run Screen::init() before
// calling Screen::new(). Please check the documentation of Screen::init() and
// Screen::new() in this module. Otherwise, usage of `processing-rs` is equivalent
// for glfw and glutin, so you don't need to really pay any other attention to the
// difference.

pub mod backend;
pub mod screen;
pub mod environment;
pub mod input;
pub mod constants;
