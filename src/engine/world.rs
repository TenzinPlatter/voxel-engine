use std::collections::HashMap;

use glam::{IVec3, Mat4};

use crate::{
    engine::{block::BlockType, voxel::Voxel}, physics::{colliding_with_aabb, PhysicsBody}, player::{Player, DEFAULT_PLAYER_REACH}, render::mesh::Mesh, Resources, State
};

#[derive(Default)]
pub struct World {
    pub voxels: HashMap<IVec3, Voxel>,
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

    /// Adds a voxel at the given position, returning the old value if one existed.
    pub fn set_voxel(&mut self, pos: IVec3) -> Option<Voxel> {
        let random_bit = rand::random::<u8>() % 2;
        let block_type = match random_bit {
            0 => BlockType::Dirt,
            1 => BlockType::Stone,
            _ => panic!("how"),
        };

        self.voxels.insert(pos, Voxel::new(pos, block_type))
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

    pub fn set_looking_at_vox(&self, state: &mut State, player: &Player) {
        state.looking_at_vox_pos = self.get_looking_at_vox_pos(player);
    }

    /// Return the voxel that the player is looking at within the players reach, if there is one
    fn get_looking_at_vox_pos(&self, player: &Player) -> Option<IVec3> {
        let ray = |n: f32| player.camera.position + n * player.camera.front;

        for i in 0..(DEFAULT_PLAYER_REACH as i32) {
            let pos = ray(i as f32).as_ivec3();
            let vox = self.voxels.get(&pos);
            if vox.is_some() {
                return Some(pos);
            }
        }

        None
    }
}
