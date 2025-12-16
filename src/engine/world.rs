use std::collections::HashMap;

use glam::{IVec3, Mat4};

use crate::{
    Resources,
    engine::{block::BlockType, voxel::Voxel},
    physics::{PhysicsBody, colliding_with_aabb},
    player::{DEFAULT_PLAYER_REACH, Player},
    render::mesh::Mesh,
};

#[derive(Default)]
pub struct World {
    voxels: HashMap<IVec3, Voxel>,
    pub mesh: Option<Mesh>,
}

impl World {
    /// Rebuilds the world's mesh from all voxels, optionally using a new texture.
    pub fn rebuild_mesh(&mut self, resources: &Resources) {
        // TODO: presize this to correct size
        let mut verticies = vec![];

        for vox in self.voxels.values() {
            verticies.extend(vox.get_verticies(resources));
        }

        self.mesh = Some(Mesh::new(&verticies, Mat4::IDENTITY, resources.atlas.texture));
    }

    /// Gets the voxel at the given position, returning None if it doesn't exist.
    pub fn get_voxel(&self, pos: &IVec3) -> Option<&Voxel> {
        self.voxels.get(pos)
    }

    /// Adds a voxel at the given position, returning the old value if one existed.
    pub fn set_voxel(&mut self, pos: IVec3) -> Option<Voxel> {
        self.voxels.insert(pos, Voxel::new(pos, BlockType::Dirt))
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

    pub fn set_looking_at_vox(&mut self, player: &Player) {
        let looking_at_vox = self.get_looking_at_vox(player);

        if let Some(vox) = looking_at_vox {}
    }

    /// Return the voxel that the player is looking at within the players reach, if there is one
    fn get_looking_at_vox(&self, player: &Player) -> Option<&Voxel> {
        let ray = |n: f32| player.camera.position + n * player.camera.front;

        for i in 0..(DEFAULT_PLAYER_REACH as i32) {
            let vox = self.voxels.get(&ray(i as f32).as_ivec3());
            if vox.is_some() {
                return vox;
            }
        }

        None
    }
}
