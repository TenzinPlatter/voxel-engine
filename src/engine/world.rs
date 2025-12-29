use glam::{IVec3, Mat4};

use crate::{
    GameResources,
    engine::{block::BlockType, voxel::Voxel},
    physics::{PhysicsBody, colliding_with_aabb},
    render::mesh::Mesh,
    utils::tracked_map::TrackedHashMap,
};

#[derive(Default)]
pub struct World {
    pub voxels: TrackedHashMap<IVec3, Voxel>,
    pub mesh: Option<Mesh>,
}

impl World {
    /// Rebuilds the world's mesh from all voxels, optionally using a new texture.
    pub fn rebuild_mesh(&mut self, resources: &GameResources) {
        // TODO: presize this to correct size
        let mut verticies = vec![];

        // voxels positions are top x, z corner
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
}
