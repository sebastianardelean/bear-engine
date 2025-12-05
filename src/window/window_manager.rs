use crate::draw::CameraMovement::{Backward, Forward, Left, Right};
use crate::draw::{Camera,  PITCH, SCALE_X, SCALE_Y, SCALE_Z, Shader,  Texture, YAW, scale, translate, rotate, Light, Model};
use crate::editor::EditorState;
use crate::window::imgui_manager::imgui_manager_mod;
use crate::{error_log, trace_log};
use glam::{Mat4, Vec3};
use glfw::{Action, Context, Key, WindowEvent};
use std::os::raw::c_void;
use std::time::Duration;

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
        .create_window(
            window_width,
            window_height,
            window_title,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_char_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);
    window.set_resizable(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map(|f| f as *const c_void)
            .unwrap_or(std::ptr::null())
    });

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Viewport(0, 0, window_width as i32, window_height as i32);
        gl::ClearColor(0.1, 0.12, 0.15, 1.0);
    }

    trace_log!("Initializing ImGui!");
    let mut imgui_manager = imgui_manager_mod::ImGuiWindow::new(&mut window);

    let mut editor_state: EditorState = EditorState::new();

    //Prepare drawing of the first shape

    trace_log!("Preparing the shaders!\n");
    let mut shader:Shader = Shader::new(
        "shaders/model/model_vs.glsl",
        "shaders/model/model_fs.glsl"
    ).unwrap_or_else(|err| {
        error_log!("Error loading object shaders:{}", err);
        panic!("Failed to load object shaders");
    });

    let _shader:u32 =shader.build_shader();

    trace_log!("Preparing the model!\n");
    let mut model:Model = Model::new(
      "resources/backpack/backpack.obj", false
    );
    model.load_model();


    trace_log!("Preparing the camera!\n");
    let mut camera: Camera = Camera::new(
        Vec3::from([0.0, 0.0, 3.0]),
        Vec3::from([0.0, 1.0, 0.0]),
        YAW,
        PITCH,
    );





    trace_log!("Preparing the renderer");


    let mut last_x: f32 = window_width as f32 / 2.0;
    let mut last_y: f32 = window_height as f32 / 2.0;

    let mut first_mouse: bool = true;

    let mut last_frame: f32 = 0.0;





    while !window.should_close() {
        glfw.poll_events();

        let current_frame = glfw.get_time() as f32;
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;

        for (_, event) in glfw::flush_messages(&events) {
            imgui_manager.handle_event(&event);

            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                WindowEvent::Key(Key::W, _, Action::Press| Action::Repeat, _) => {
                    camera.process_keyboard(Forward, delta_time);
                }
                WindowEvent::Key(Key::A, _, Action::Press | Action::Repeat, _) => {
                    camera.process_keyboard(Left, delta_time);
                }
                WindowEvent::Key(Key::S, _, Action::Press| Action::Repeat, _) => {
                    camera.process_keyboard(Backward, delta_time);
                }
                WindowEvent::Key(Key::D, _, Action::Press| Action::Repeat, _) => {
                    camera.process_keyboard(Right, delta_time);
                }
                WindowEvent::FramebufferSize(w, h) => unsafe {
                    gl::Viewport(0, 0, w, h);
                },
                WindowEvent::CursorPos(x, y) => {
                    //handle mouse
                    let x_pos: f32 = x as f32;
                    let y_pos: f32 = y as f32;
                    if first_mouse {
                        last_x = x_pos;
                        last_y = y_pos;
                        first_mouse = false;
                    }

                    let x_offset: f32 = x_pos - last_x;
                    let y_offset: f32 = last_y - y_pos;
                    last_x = x_pos;
                    last_y = y_pos;
                    camera.process_mouse(x_offset, y_offset, true);
                }
                WindowEvent::Scroll(_x_offset, y_offset) => {
                    //handle scroll on y offset
                    camera.process_mouse_scroll(y_offset as f32);
                }

                _ => {}
            }
        }

        // // The rest of the game loop goes here...
        unsafe {
            gl::ClearColor(0.05, 0.05, 0.05, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader.apply_shader();

        //view/projection transformation
        let projection_matrix: Mat4 = Mat4::perspective_rh(
            camera.get_zoom().to_radians(),
            window_width as f32 / window_height as f32,
            0.1,
            100.0,
        );
        let view_matrix: Mat4 = camera.view_matrix();

        shader.set_uniform_matrix_4(String::from("projection"), projection_matrix);
        shader.set_uniform_matrix_4(String::from("view"), view_matrix);


        let mut model_mat:Mat4 = Mat4::IDENTITY;
        model_mat = model_mat * translate(Vec3::new(0.0, 0.0, 0.0))*scale(0.5,SCALE_X|SCALE_Y|SCALE_Z);
        shader.set_uniform_matrix_4(String::from("model"), model_mat);
        model.draw(&mut shader);

        //////////////////////////

        imgui_manager.init_frame(&window, delta_time, &mut editor_state);

        window.swap_buffers();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
