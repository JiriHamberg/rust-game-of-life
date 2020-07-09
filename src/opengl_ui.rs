#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;
use self::gl::types::*;

use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

use super::shader::Shader;

use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Rad, Vector3};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub struct Canvas {
    pub points: Vec<(i32, i32)>,
}

impl Canvas {
    #[allow(non_snake_case)]
    pub fn run(&self) {
        // glfw: initialize and configure
        // ------------------------------
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation
        // --------------------
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
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let (ourShader, VBO, VAO) = unsafe {
            // configure global opengl state
            // -----------------------------
            gl::Enable(gl::DEPTH_TEST);

            // build and compile our shader program
            // ------------------------------------
            let ourShader = Shader::new("src/shaders/rect.vs", "src/shaders/rect.fs");

            let vertices: [f32; 32] = [
                // positions       // colors        // texture coords
                0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
                0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
                -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
                -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
            ];
            let indices = [
                0, 1, 3, // first Triangle
                1, 2, 3, // second Triangle
            ];

            let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
            gl::GenBuffers(1, &mut EBO);

            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            // color attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
            // texture coord attribute
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (6 * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            (ourShader, VBO, VAO)
        };

        // render loop
        // -----------
        while !window.should_close() {
            // events
            // -----
            Canvas::process_events(&mut window, &events);

            // render
            // ------
            unsafe {
                gl::ClearColor(1.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // activate shader
                ourShader.useProgram();

                // create transformations
                // NOTE: cgmath requires axis vectors to be normalized!
                let model: Matrix4<f32> = Matrix4::from_axis_angle(
                    vec3(0.5, 1.0, 0.0).normalize(),
                    Rad(glfw.get_time() as f32),
                );
                let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -100.));
                let projection: Matrix4<f32> =
                    perspective(Deg(45.0), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
                // retrieve the matrix uniform locations
                let modelLoc = gl::GetUniformLocation(ourShader.ID, c_str!("model").as_ptr());
                let viewLoc = gl::GetUniformLocation(ourShader.ID, c_str!("view").as_ptr());
                // pass them to the shaders (3 different ways)
                gl::UniformMatrix4fv(modelLoc, 1, gl::FALSE, model.as_ptr());
                gl::UniformMatrix4fv(viewLoc, 1, gl::FALSE, &view[0][0]);
                // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes it's often best practice to set it outside the main loop only once.
                ourShader.setMat4(c_str!("projection"), &projection);

                let rectPositions: Vec<Vector3<f32>> = self
                    .points
                    .iter()
                    .map(|(x, y)| vec3(*x as f32, *y as f32, 0.0))
                    .collect();

                // render boxes
                gl::BindVertexArray(VAO);
                for position in rectPositions {
                    // calculate the model matrix for each object and pass it to shader before drawing
                    let model: Matrix4<f32> = Matrix4::from_translation(position);
                    ourShader.setMat4(c_str!("model"), &model);

                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                }
            }

            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
            // -------------------------------------------------------------------------------
            window.swap_buffers();
            glfw.poll_events();
        }

        // optional: de-allocate all resources once they've outlived their purpose:
        // ------------------------------------------------------------------------
        unsafe {
            gl::DeleteVertexArrays(1, &VAO);
            gl::DeleteBuffers(1, &VBO);
        }
    }

    // NOTE: not the same version as in common.rs!
    fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
