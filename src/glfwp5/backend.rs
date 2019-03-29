#[cfg(feature = "glfw")]

use std::cell::{RefCell, Cell, Ref, RefMut};
use std::rc::Rc;
use std::os::raw::c_void;
use std::ops::Deref;

use glfw;
use glfw::Context;
use glium;
use glium::Frame;
use glium::backend::{Backend, Facade};
use errors::ProcessingErr;

// in order to create our context, we will need to provide an object which implements
// the `Backend` trait
pub struct GLFWBackend(Rc<RefCell<glfw::Window>>);

#[derive(Clone)]
pub struct Display {
    // contains everything related to the current context and its state
    context: Rc<glium::backend::Context>,
    // The glutin Window alongside its associated GL Context.
    gl_window: Rc<RefCell<glfw::Window>>,
    // Used to check whether the framebuffer dimensions have changed between frames. If they have,
    // the glutin context must be resized accordingly.
    last_framebuffer_dimensions: Cell<(u32, u32)>,
}

impl Display {
    /// Create a new glium `Display` from the given context and window builders.
    pub fn new(gl_window: glfw::Window) -> Result<Self, ProcessingErr> {
        let gl_window = Rc::new(RefCell::new(gl_window));
        let glfw_backend = GLFWBackend(gl_window.clone());
        let framebuffer_dimensions = glfw_backend.get_framebuffer_dimensions();
        let context =
            unsafe { glium::backend::Context::new(glfw_backend, true, Default::default()) };
        Ok(Display {
            gl_window: gl_window,
            context: context.map_err(|e| ProcessingErr::ContextNoCreate(e))?,
            last_framebuffer_dimensions: Cell::new(framebuffer_dimensions),
        })
    }

    // /// Rebuilds the Display's `GlWindow` with the given window and context builders.
    // ///
    // /// This method ensures that the new `GlWindow`'s `Context` will share the display lists of the
    // /// original `GlWindow`'s `Context`.
    // pub fn rebuild(&self,
    //                window_builder: glutin::WindowBuilder,
    //                context_builder: glutin::ContextBuilder,
    //                events_loop: &glutin::EventsLoop)
    //                -> Result<(), DisplayCreationError> {
    //     // Share the display lists of the existing context.
    //     let new_gl_window = {
    //         let gl_window = self.gl_window.borrow();
    //         let context_builder = context_builder.with_shared_lists(gl_window.context());
    //         try!(glutin::GlWindow::new(window_builder, context_builder, events_loop))
    //     };

    //     // Replace the stored GlWindow with the new one.
    //     let mut gl_window = self.gl_window.borrow_mut();
    //     std::mem::replace(&mut (*gl_window), new_gl_window);

    //     // Rebuild the Context.
    //     let backend = GlutinBackend(self.gl_window.clone());
    //     try!(unsafe { self.context.rebuild(backend) });

    //     Ok(())
    // }

    /// Borrow the inner glutin GlWindow.
    #[inline]
    pub fn gl_window(&self) -> Ref<glfw::Window> {
        self.gl_window.borrow()
    }

    /// Borrow the inner glutin GlWindow.
    #[inline]
    pub fn gl_window_mut(&self) -> RefMut<glfw::Window> {
        self.gl_window.borrow_mut()
    }

    /// Start drawing on the backbuffer.
    ///
    /// This function returns a `Frame`, which can be used to draw on it. When the `Frame` is
    /// destroyed, the buffers are swapped.
    ///
    /// Note that destroying a `Frame` is immediate, even if vsync is enabled.
    ///
    /// If the framebuffer dimensions have changed since the last call to `draw`, the inner glutin
    /// context will be resized accordingly before returning the `Frame`.
    #[inline]
    pub fn draw(&self) -> Frame {
        let (w, h) = self.context.get_framebuffer_dimensions();

        // If the size of the framebuffer has changed, resize the context.
        if self.last_framebuffer_dimensions.get() != (w, h) {
            self.last_framebuffer_dimensions.set((w, h));
            self.gl_window.borrow_mut().set_size(w as i32, h as i32);
        }

        Frame::new(self.context.clone(), (w, h))
    }
}

impl Deref for Display {
    type Target = glium::backend::Context;
    #[inline]
    fn deref(&self) -> &glium::backend::Context {
        &self.context
    }
}

impl Facade for Display {
    #[inline]
    fn get_context(&self) -> &Rc<glium::backend::Context> {
        &self.context
    }
}

impl Deref for GLFWBackend {
    type Target = Rc<RefCell<glfw::Window>>;
    #[inline]
    fn deref(&self) -> &Rc<RefCell<glfw::Window>> {
        &self.0
    }
}

unsafe impl glium::backend::Backend for GLFWBackend {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        Ok(self.0.borrow_mut().swap_buffers())
    }

    // this function is called only after the OpenGL context has been made current
    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        self.0.borrow_mut().get_proc_address(symbol) as *const _
    }

    // this function is used to adjust the viewport when the user wants to draw or blit on
    // the whole window
    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        // we default to a dummy value is the window no longer exists
        let (fbw, fbh) = self.0.borrow().get_framebuffer_size();
        (fbw as u32, fbh as u32)
    }

    fn is_current(&self) -> bool {
        // if you are using a library that doesn't provide an equivalent to `is_current`, you
        // can just put `unimplemented!` and pass `false` when you create
        // the `Context` (see below)
        self.0.borrow().is_current()
    }

    unsafe fn make_current(&self) {
        self.0.borrow_mut().make_current();
    }
}
