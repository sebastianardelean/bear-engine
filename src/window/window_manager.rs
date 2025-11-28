use crate::draw::{
    RenderManager, RotationAxis, SCALE_X, SCALE_Y, SCALE_Z, Shader, Shape, Texture, bind_texture,
    get_identity, rotate, scale, translate,
};
use crate::editor::EditorState;
use crate::window::imgui_manager::imgui_manager_mod;
use crate::{error_log, trace_log};
use gl::FALSE;
use gl::types::GLuint;
use glam::{Mat4, Vec3};
use glfw::{Action, Context, Key, WindowEvent};
use std::os::raw::c_void;
use std::time::{Duration, Instant};

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

    let mut last_frame = Instant::now();

    let mut editor_state: EditorState = EditorState::new();

    //Prepare drawing of the first shape

    trace_log!("Preparing the shaders!\n");
    let mut shader = Shader::new("shaders/coordinates_vs.glsl", "shaders/coordinates_fs.glsl")
        .unwrap_or_else(|err| {
            error_log!("Error loading shaders:{}", err);
            panic!("Failed to load shaders");
        });

    let _shader_program_id: u32 = shader.build_shader();

    let vertices: [f32; 180] = [
        -0.5, -0.5, -0.5,  0.0, 0.0,
        0.5, -0.5, -0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5,  0.5,  0.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
        0.5, -0.5, -0.5,  1.0, 1.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
        0.5,  0.5, -0.5,  1.0, 1.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
        0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0
    ];

    let cube_positions: [Vec3; 10] = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 5.0, -15.0),
        Vec3::new(-1.5, -2.2, -2.5),
        Vec3::new(-3.8, -2.0, -12.3),
        Vec3::new(2.4, -0.4, -3.5),
        Vec3::new(-1.7, 3.0, -7.5),
        Vec3::new(1.3, -2.0, -2.5),
        Vec3::new(1.5, 2.0, -2.5),
        Vec3::new(1.5, 0.2, -1.5),
        Vec3::new(-1.3, 1.0, -1.5),
    ];

    trace_log!("Preparing GPU Buffers\n");
    let shape: Shape = Shape::new(Vec::from(vertices), None);

    trace_log!("Preparing the textures\n");
    let mut texture_1: Texture = Texture::new(
        gl::REPEAT as i32,
        gl::REPEAT as i32,
        gl::REPEAT as i32,
        gl::LINEAR as i32,
        gl::LINEAR as i32,
        "textures/container.png",
    )
    .unwrap_or_else(|err| {
        error_log!("Error loading textures:{}", err);
        panic!("Failed to load texture!");
    });

    let mut texture_2: Texture = Texture::new(
        gl::REPEAT as i32,
        gl::REPEAT as i32,
        gl::REPEAT as i32,
        gl::LINEAR as i32,
        gl::LINEAR as i32,
        "textures/awesomeface.png",
    )
    .unwrap_or_else(|err| {
        error_log!("Error loading textures:{}", err);
        panic!("Failed to load texture!");
    });

    let id_1 = texture_1.create_texture();
    let id_2 = texture_2.create_texture();

    let textures: Vec<(u32, &str)> = vec![(id_1, "texture1"), (id_2, "texture2")];

    trace_log!("Preparing the renderer");

    let mut render_manager = RenderManager::new();
    render_manager.prepare(&mut shader, &textures);

    render_manager.queue_shapes(shape);

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            imgui_manager.handle_event(&event);

            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                WindowEvent::FramebufferSize(w, h) => unsafe {
                    gl::Viewport(0, 0, w, h);
                },
                _ => {}
            }
        }

        // // The rest of the game loop goes here...
        unsafe {
            gl::ClearColor(0.1, 0.12, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render_manager.apply_texture(&textures);

        let mut view_matrix = get_identity();
        let mut projection_matrix = get_identity();

        projection_matrix = Mat4::perspective_rh(
            45f32.to_radians(),
            window_width as f32 / window_height as f32,
            0.1,
            100.0,
        );
        view_matrix = view_matrix * translate(Vec3::new(0.0, 0.0, -3.0));

        shader.set_uniform_matrix_4(String::from("projection"), projection_matrix);
        shader.set_uniform_matrix_4(String::from("view"), view_matrix);

        for (i, position) in cube_positions.iter().enumerate() {
            let mut model = Mat4::IDENTITY;
            model = model * translate(*position);
            let angle: f32 = 20.0 * i as f32;
            let axis = Vec3::new(1.0, 0.3, 0.5).normalize();
            let rotation = Mat4::from_axis_angle(axis, angle.to_radians());
            model = model * rotation;
            shader.set_uniform_matrix_4(String::from("model"), model);
            render_manager.draw(&mut shader);
        }

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
