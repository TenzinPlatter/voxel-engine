use std::collections::HashMap;

use glam::{IVec3, Mat4};

use crate::{
    engine::voxel::Voxel,
    render::{mesh::Mesh, texture::Texture},
};

#[derive(Default)]
pub struct World {
    voxel_positions: HashMap<IVec3, Voxel>,
    pub mesh: Option<Mesh>,
}

impl World {
    pub fn rebuild_mesh(&mut self, texture: Option<Texture>) {
        // TODO: presize this to correct size
        let mut verticies = vec![];

        for vox in self.voxel_positions.values() {
            verticies.extend(vox.get_verticies());
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

    /// get the voxel at position `pos`. If it exists return Some(&Voxel), else None
    pub fn get_voxel(&self, pos: &IVec3) -> Option<&Voxel> {
        self.voxel_positions.get(pos)
    }

    /// Returns None if a voxel was added, Some(Voxel) if the value already existed
    pub fn set_voxel(&mut self, pos: IVec3) -> Option<Voxel> {
        self.voxel_positions.insert(pos, Voxel::new(pos))
    }

    /// Returns the value that was removed if it existed in the map, else None
    pub fn remove_voxel(&mut self, pos: &IVec3) -> Option<Voxel> {
        self.voxel_positions.remove(pos)
    }
}
