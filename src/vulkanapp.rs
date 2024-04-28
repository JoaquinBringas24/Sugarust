
extern crate glfw;
use ash;
use glm;
use glfw::{fail_on_errors, Context, Glfw, WindowEvent};
pub struct VulkanApp {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl VulkanApp {
    // Main function 
    pub fn run(self) {
        let (window, events, glfw) = VulkanApp::init_window(self.width, self.height, &self.title);
        VulkanApp::main_loop(window, events, glfw);
    }

    /// fn init_vulkan() {
    ///    todo!()l
    /// }
    // Loop events gotten from the init method
    fn main_loop(mut window:glfw::PWindow, events: glfw::GlfwReceiver<(f64, WindowEvent)>, mut glfw: Glfw) {
        window.make_current();
        window.set_key_polling(true);
        
        while  !window.should_close() {
            window.swap_buffers();

            glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
            
        }
    }

    // Create a window 
    fn init_window(width: u32, height: u32, title: &str) -> (glfw::PWindow, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>, Glfw) {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        
        let (window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        return  (window, events, glfw)
    }

}