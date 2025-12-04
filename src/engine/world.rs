use std::collections::HashSet;

use glam::{IVec3, Mat4};

use crate::render::{
    get_voxel_verticies,
    mesh::{self, Mesh},
    texture::Texture,
};

#[derive(Default)]
pub struct World {
    voxel_positions: HashSet<IVec3>,
    pub mesh: Option<Mesh>,
}

impl World {
    pub fn rebuild_mesh(&mut self, texture: Option<Texture>) {
        // TODO: presize this to correct size
        let mut verticies = vec![];

        for v in &self.voxel_positions {
            verticies.extend(get_voxel_verticies(v));
        }

        let tex = if let Some(tex) = texture {
            tex
        } else if let Some(mesh) = self.mesh.take() {
            mesh.texture
        } else {
            // TODO: better way to handle this?
            Texture::new().unwrap()
        };

        self.mesh = Some(Mesh::new(&verticies, Mat4::IDENTITY, tex));
    }

    /// Returns true if a voxel was added, false if it was removed
    pub fn set_voxel(&mut self, pos: IVec3) -> bool {
        self.voxel_positions.insert(pos)
    }

    /// Returns whether the value existed and was removed
    pub fn remove_voxel(&mut self, pos: &IVec3) -> bool {
        self.voxel_positions.remove(pos)
    }
}
