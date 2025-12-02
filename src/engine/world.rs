use std::collections::HashSet;

use glam::IVec3;

use crate::render::{mesh::Mesh, shader::ShaderProgram};

pub struct World {
    pub voxels: HashSet<IVec3>,
    pub shader_program: ShaderProgram,
    pub mesh: Option<Mesh>,
}

impl World {
    pub fn new(vertex_shader: &str, fragment_shader: &str) -> Self {
        let shader_program = ShaderProgram::from_vert_frag(vertex_shader, fragment_shader)
                .expect("Failed to create shader program for world");

        shader_program.use_program();

        Self {
            voxels: HashSet::new(),
            mesh: None,
            shader_program,
        }
    }

    pub fn draw(&self) {
        if let Some(mesh) = &self.mesh {
            mesh.draw(&self.shader_program);
        }
    }
}
