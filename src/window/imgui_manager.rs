pub mod imgui_manager_mod {
    use crate::trace_log;

    use glfw::{Action, Window, WindowEvent};
    use imgui::{Condition, Ui};

    use std::os::raw::c_void;
    
    
    use imgui_opengl_renderer::Renderer;
    use crate::editor::EditorState;

    #[derive()]
    pub struct ImGuiWindow {
        imgui_context: imgui::Context,

        renderer: Renderer,

    }


    impl ImGuiWindow {
        pub fn new(window: &mut Window) -> ImGuiWindow {
            let mut imgui = imgui::Context::create();
            imgui.set_ini_filename(None);

            let renderer = Renderer::new(&mut imgui, |s| {
                window
                    .get_proc_address(s)
                    .map(|f| f as *const c_void)
                    .unwrap_or(std::ptr::null())
            });

            ImGuiWindow {
                imgui_context: imgui,
                renderer,
            }
        }

        pub fn handle_event(&mut self, event: &WindowEvent) {
            let io = self.imgui_context.io_mut();

            match event {
                WindowEvent::Key(key, _, action, _) => {
                    let pressed = *action != Action::Release;
                    io.keys_down[*key as usize] = pressed;
                }
                WindowEvent::Char(c) => {
                    io.add_input_character(*c);
                }
                WindowEvent::CursorPos(x, y) => {
                    io.mouse_pos = [*x as f32, *y as f32];
                }
                WindowEvent::MouseButton(button, action, _) => {
                    let pressed = *action != Action::Release;
                    match button {
                        glfw::MouseButtonLeft => io.mouse_down[0] = pressed,
                        glfw::MouseButtonRight => io.mouse_down[1] = pressed,
                        glfw::MouseButtonMiddle => io.mouse_down[2] = pressed,
                        _ => {}
                    }
                }
                glfw::WindowEvent::Scroll(x, y) => {
                    io.mouse_wheel_h += *x as f32;
                    io.mouse_wheel += *y as f32;
                }
                _ => {}
            }
        }

        pub fn init_frame(&mut self, window: &Window, delta_s: f32, editor_state: &mut EditorState) {
            let (width, height) = window.get_framebuffer_size();
            self.imgui_context.io_mut().display_size = [width as f32, height as f32];

            self.imgui_context.io_mut().delta_time = delta_s;
            let ui = self.imgui_context.frame();

            Self::show_menu_bar(ui);
            Self::show_properties_window(ui);
            Self::show_toolbox_window(ui, editor_state);
            //
            // ui.show_about_window(&mut true);
            // ui.show_demo_window(&mut true);
            // ui.show_metrics_window(&mut true);

            self.renderer.render(&mut self.imgui_context);
        }

        fn show_properties_window(ui: &mut Ui) {
            ui.window("Properties")
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text("Hello, Properties!");
                });
        }

        fn show_toolbox_window(ui: &mut Ui, state: &mut EditorState) {

            ui.window("Toolbox")

                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(|| {
                    ui.text("Hello, Toolbox!");
                    if ui.button("Quantum Register") {
                        trace_log!("Quantum Register pressed");
                        ui.open_popup("modal1");
                    }

                    ui.popup("modal1", || {

                         ui.input_int("Number of qubits", &mut state.qcircuit_parms.number_of_qubits).build();

                        if ui.button("OK") {
                             if state.qcircuit_parms.number_of_qubits > 0 {
                                 ui.close_current_popup();
                            }
                        }
                        ui.same_line();
                        if ui.button("Cancel") {
                            ui.close_current_popup(); // closes popup
                        }
                    });




                });

        }

        fn show_menu_bar(ui: &mut Ui) {
            ui.main_menu_bar(|| {
                ui.menu("File", || {
                    if ui.menu_item_config("Quit").build() {
                        std::process::exit(0);
                    }
                });
                ui.menu("Help", || {
                    if ui.menu_item_config("About").build() {
                        println!("This is a simple ImGui example!");
                    }
                });
            });
        }
    }
}
