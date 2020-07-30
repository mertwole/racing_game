use std::sync::mpsc::Receiver;

use glfw::{Context, Action};
pub use glfw::{Key};

use crate::engine::input::EventType;

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

    pub fn get_events(&mut self) -> Vec<(Key, EventType)>{
        let mut events : Vec<(Key, EventType)> = Vec::new();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                
                glfw::WindowEvent::Key(key, _, Action::Press, _) => events.push((key, EventType::Pressed)),
                glfw::WindowEvent::Key(key, _, Action::Release, _) => events.push((key, EventType::Released)),

                _ => {}
            }
        }

        self.glfw.poll_events();

        events
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn should_close(&self) -> bool{
        self.window.should_close()
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }

    pub fn set_time(&mut self, time : f64) {
        self.glfw.set_time(time);
    }
}