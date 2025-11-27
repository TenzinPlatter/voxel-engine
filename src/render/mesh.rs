use std::ptr;

use bytemuck::cast_slice;
use gl33::{GL_STATIC_DRAW, GL_TRIANGLES, GL_UNSIGNED_INT, global_loader::*};

use crate::render::{buffer::{buffer_data, Buffer, BufferType, VertexArray}, vertex::VertexTex};

pub struct Mesh {
    vao: VertexArray,
    vbo: Buffer,
    nverticies: u32,
}

impl Mesh {
    pub fn new(verticies: &[VertexTex]) -> Self {
        let vao = VertexArray::new().expect("Failed to create VAO");
        vao.bind();
        let vbo = Buffer::new().expect("Failed to create VBO");
        vbo.bind(BufferType::Array);
        buffer_data(BufferType::Array, cast_slice(verticies), GL_STATIC_DRAW);

        Self {
            vao,
            vbo,
            nverticies: verticies.len() as u32,
        }
    }

    pub fn draw(&self) {
        glBindVertexArray(self.vao.0);
        unsafe {
            glDrawElements(GL_TRIANGLES, self.nverticies as i32, GL_UNSIGNED_INT, ptr::null());
        }
    }
}
