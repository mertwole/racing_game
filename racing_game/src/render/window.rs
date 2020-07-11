use glfw::{Context, Key, Action};
use std::sync::mpsc::Receiver;

pub struct WindowParameters {
    pub width : u32,
    pub height : u32,
    pub title : String
}

pub struct Window {
    window : glfw::Window,
    glfw : glfw::Glfw,
    events : Receiver<(f64, glfw::WindowEvent)>
}

impl Window{    
    pub fn open(parameters : WindowParameters) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    
        let (mut window, events) = glfw.create_window(parameters.width, parameters.height, &parameters.title, glfw::WindowMode::Windowed).expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
    
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window { window, glfw, events }
    }

    pub fn update_events(&mut self){
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.window.set_should_close(true),
                _ => {}
            }
        }

        self.glfw.poll_events();
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn should_close(&self) -> bool{
        self.window.should_close()
    }
}