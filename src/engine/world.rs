use std::collections::HashSet;

use glam::IVec3;

use crate::render::{get_voxel_verticies, mesh::Mesh, shader::ShaderProgram};

pub struct World {
    pub voxel_positions: HashSet<IVec3>,
    pub mesh: Option<Mesh>,
}

impl World {
    pub fn new(vertex_shader: &str, fragment_shader: &str) -> Self {
        Self {
            voxel_positions: HashSet::new(),
            mesh: None,
            shader_program,
        }
    }

    pub fn draw(&self) {
        if let Some(mesh) = &self.mesh {
            mesh.draw(&self.shader_program);
        }
    }

    pub fn rebuild_mesh(&mut self) {
        // TODO: presize this to correct size
        let mut verticies = vec![];

        for v in &self.voxel_positions {
            verticies.extend(get_voxel_verticies(&v));
        }

        self.mesh = Some(Mesh::new(&verticies));
    }
}
