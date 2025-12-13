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
    /// Rebuilds the world's mesh from all voxels, optionally using a new texture.
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

    /// Gets the voxel at the given position, returning None if it doesn't exist.
    pub fn get_voxel(&self, pos: &IVec3) -> Option<&Voxel> {
        self.voxels.get(pos)
    }

    /// Adds a voxel at the given position, returning the old value if one existed.
    pub fn set_voxel(&mut self, pos: IVec3) -> Option<Voxel> {
        self.voxels.insert(pos, Voxel::new(pos))
    }

    /// Removes the voxel at the given position, returning it if it existed.
    pub fn remove_voxel(&mut self, pos: &IVec3) -> Option<Voxel> {
        self.voxels.remove(pos)
    }

    /// Checks if the given physics body is colliding with any voxel in the world.
    pub fn is_colliding(&self, other: &PhysicsBody) -> bool {
        // TODO: extrusion or something so we cant phase through floor
        // TODO: optimize to not check every square
        self.voxels.values().any(|v| colliding_with_aabb(&v.body, other))
    }
}
