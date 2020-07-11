#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;

use std::sync::mpsc::Receiver;

use super::rectangle_program::RectangleProgram;

use cgmath::{perspective, vec3, Deg, Matrix4, Vector3};

use std::time::{SystemTime, UNIX_EPOCH};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 800;

const UPDATE_FREQ_MILLIS: u16 = 40;

pub struct Canvas {
    pub point_receiver: Receiver<Vec<(i32, i32)>>,
}

impl Canvas {
    #[allow(non_snake_case)]
    pub fn run(&self) {
        // glfw: initialize and configure
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation
        let (mut window, events) = glfw
            .create_window(
                SCR_WIDTH,
                SCR_HEIGHT,
                "Game of life",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            let rectangle_program = RectangleProgram::new();

            let mut last_update = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            // render loop
            while !window.should_close() {
                // events
                Canvas::process_events(&mut window, &events);

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                let maybe_points = if now - last_update > UPDATE_FREQ_MILLIS as u128 {
                    last_update = now;
                    self.point_receiver.try_recv().ok()
                } else {
                    None
                };

                // render
                maybe_points.map(|points| {
                    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    rectangle_program.use_program();

                    let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -250.));
                    let projection: Matrix4<f32> =
                        perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 400.0);
                    rectangle_program.set_projection(&projection);
                    rectangle_program.set_view(&view);

                    let rectPositions: Vec<Vector3<f32>> = points
                        .iter()
                        .map(|(x, y)| vec3(*x as f32, *y as f32, 0.0))
                        .collect();

                    for position in rectPositions {
                        // calculate the model matrix for each object and pass it to shader before drawing
                        let model: Matrix4<f32> =
                            Matrix4::from_translation(position + vec3(-100.0, -100.0, 0.0));
                        rectangle_program.set_model(&model);
                        rectangle_program.draw_rectangle();
                    }

                    window.swap_buffers();
                });
                glfw.poll_events();
            }
        }
    }

    fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height)
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
