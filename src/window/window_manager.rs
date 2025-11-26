use crate::{error_log, trace_log};
use std::os::raw::c_void;
use crate::window::imgui_manager::imgui_manager_mod;
use glfw::{Action, Context, Key, WindowEvent};
use std::time::{Duration, Instant};
use gl::types::GLuint;
use crate::draw::{DrawManager, Shader, Texture};
use crate::editor::EditorState;

pub fn create_window(window_title: &String, window_width: u32, window_height: u32) {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap_or_else(|e| {
        error_log!("Failed to initialize glfw context: {}", e);
        panic!("glfw failed!");
    });

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(window_width, window_height, window_title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_char_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);
    window.set_resizable(true);

    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map(|f| f as *const c_void)
            .unwrap_or(std::ptr::null())
    });

    unsafe {
        gl::Viewport(0, 0, window_width as i32, window_height as i32);
        gl::ClearColor(0.1, 0.12, 0.15, 1.0);
    }


    trace_log!("Initializing ImGui!");
    let mut imgui_manager = imgui_manager_mod::ImGuiWindow::new(&mut window);

    let mut last_frame = Instant::now();

    let mut editor_state: EditorState = EditorState::new();

    //Prepare drawing
    let vertices: [f32; 32] = [
        // positions          // colors           // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
    ];

    let indices: [u32;6] = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];

    let mut shader = Shader::new("shaders/vs.glsl", "shaders/fs.glsl").unwrap_or_else(|err| {
        error_log!("Error loading shaders:{}", err);
        panic!("Failed to load shaders");
    });

    let program:GLuint = shader.build_shader();


    let mut texture_1:Texture=Texture::new(
        gl::REPEAT as i32,
        gl::REPEAT as i32,
        gl::REPEAT as i32,
        gl::LINEAR as i32,
        gl::LINEAR as i32,
        "textures/wall.png").unwrap_or_else(|err| {
        error_log!("Error loading textures:{}", err);
        panic!("Failed to load texture!");
    });

    let id_1 =texture_1.create_texture();

    let mut texture_2:Texture=Texture::new(
        gl::REPEAT as i32,
        gl::REPEAT as i32,
        gl::REPEAT as i32,
        gl::LINEAR as i32,
        gl::LINEAR as i32,
        "textures/awesomeface.png").unwrap_or_else(|err| {
        error_log!("Error loading textures:{}", err);
        panic!("Failed to load texture!");
    });

    let id_2 =texture_2.create_texture();

    let textures:Vec<(u32, &str)> = vec![(id_1,"texture1"),(id_2,"texture_2")];

    let mut draw_manager = DrawManager::new();


    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            imgui_manager.handle_event(&event);

            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                WindowEvent::FramebufferSize(w,h) => {
                    unsafe {
                        gl::Viewport(0, 0, w, h);
                    }
                }
                _ => {}
            }
        }

        // // The rest of the game loop goes here...
        // --- DRAW TRIANGLE --- //
        //clear viewport and update size
        let (w,h) = window.get_framebuffer_size();
        unsafe {
            gl::ClearColor(0.1, 0.12, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        draw_manager.draw(&vertices, &indices, &mut shader, &textures, true);




        //////////////////////////
        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;



        imgui_manager.init_frame(&window, delta_s, &mut editor_state);

        window.swap_buffers();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
}







