extern crate gl;
use self::gl::types::*;

use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::Matrix4;

use super::shader::Shader;

pub const RECTANGLE_SIZE: f32 = 1.0;

pub struct RectangleProgram {
    shader: Shader,
    vao: u32,
    vbo: u32,
    ebo: u32,
}

impl RectangleProgram {
    pub unsafe fn new() -> RectangleProgram {
        let shader = Shader::new("src/shaders/rect.vs", "src/shaders/rect.fs");

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

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
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

        RectangleProgram {
            shader: shader,
            vbo: vbo,
            vao: vao,
            ebo: ebo,
        }
    }

    pub unsafe fn use_program(&self) {
        gl::Enable(gl::DEPTH_TEST);
        self.shader.useProgram();
        gl::BindVertexArray(self.vao);
    }

    pub unsafe fn set_view(&self, view: &Matrix4<f32>) {
        self.shader.setMat4(c_str!("view"), view);
    }

    pub unsafe fn set_model(&self, model: &Matrix4<f32>) {
        self.shader.setMat4(c_str!("model"), model);
    }

    pub unsafe fn set_projection(&self, projection: &Matrix4<f32>) {
        self.shader.setMat4(c_str!("projection"), projection);
    }

    pub unsafe fn draw_rectangle(&self) {
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    }
}

impl Drop for RectangleProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
