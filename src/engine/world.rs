use std::collections::HashMap;

use glam::{IVec3, Mat4};

use crate::{
    engine::voxel::Voxel,
    physics::{PhysicsBody, colliding_with_aabb},
    render::{mesh::Mesh, texture::Texture},
};

#[derive(Default)]
pub struct World {
    voxels: HashMap<IVec3, Voxel>,
    pub mesh: Option<Mesh>,
}

impl World {
    pub fn rebuild_mesh(&mut self, texture: Option<Texture>) {
        // TODO: presize this to correct size
        let mut verticies = vec![];

        for vox in self.voxels.values() {
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
        self.voxels.get(pos)
    }

    /// Returns None if a voxel was added, Some(Voxel) if the value already existed
    pub fn set_voxel(&mut self, pos: IVec3) -> Option<Voxel> {
        self.voxels.insert(pos, Voxel::new(pos))
    }

    /// Returns the value that was removed if it existed in the map, else None
    pub fn remove_voxel(&mut self, pos: &IVec3) -> Option<Voxel> {
        self.voxels.remove(pos)
    }

    pub fn is_colliding(&self, other: &PhysicsBody) -> bool {
        self.voxels.values().any(|v| colliding_with_aabb(&v.body, other))
    }
}
