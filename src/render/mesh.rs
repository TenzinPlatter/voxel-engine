use bytemuck::cast_slice;
use gl33::{GL_STATIC_DRAW, GL_TRIANGLES, global_loader::*};
use glam::Mat4;

use crate::render::{buffer::{buffer_data, Buffer, BufferType, VertexArray}, shader::{ShaderProgram, ShaderUniformType}, texture::Texture, vertex::{Vertex, VertexTex}};

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

    pub fn draw(&self, shader_program: &ShaderProgram) {
        glBindVertexArray(self.vao.0);
        Mat4::set_uniform(shader_program, "model", Mat4::IDENTITY);
        unsafe {
            glDrawArrays(GL_TRIANGLES, 0, self.nverticies as i32);
        }
    }
}
