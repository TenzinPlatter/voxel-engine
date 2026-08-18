use glam::{IVec3, Mat4};

use crate::{
    GameResources,
    engine::{block::BlockType, voxel::Voxel},
    physics::{PhysicsBody, colliding_with},
    render::mesh::Mesh,
    utils::tracked_map::TrackedHashMap,
};

pub struct World {
    pub voxels: TrackedHashMap<IVec3, Voxel>,
    pub mesh: Option<Mesh>,
}

impl World {
    /// Rebuilds the world's mesh from all voxels, optionally using a new texture.
    pub fn rebuild_mesh(&mut self, resources: &GameResources) {
        // TODO: presize this to correct size
        let mut vertcies = vec![];

        // voxels positions are top x, z corner
        for vox in self.voxels.values() {
            vertcies.extend(vox.get_vertices(resources));
        }

        self.mesh = Some(Mesh::new(
            &vertcies,
            Mat4::IDENTITY,
            resources.atlas.texture,
        ));
    }

    /// Adds a voxel at the given position, returning the old value if one existed.
    pub fn set_voxel(&mut self, pos: IVec3, block_type: BlockType) -> Option<Voxel> {
        self.voxels.insert(pos, Voxel::new(pos, block_type))
    }

    /// Removes the voxel at the given position, returning it if it existed.
    pub fn remove_voxel(&mut self, pos: &IVec3) -> Option<Voxel> {
        self.voxels.remove(pos)
    }

    /// Checks if the given physics body is colliding with any voxel in the world.
    pub fn is_colliding(&self, other: &PhysicsBody) -> bool {
        // TODO: extrusion or something so we cant phase through voxels when moving quickly
        // TODO: optimize to not check every square
        self.voxels
            .values()
            .any(|v| colliding_with(&v.body, other))
    }
}

impl Default for World {
    fn default() -> Self {
        let mut res = Self {
            voxels: Default::default(),
            mesh: Default::default(),
        };

        for z in -32..32 {
            for x in -32..32 {
                res.set_voxel(IVec3::new(x, 0, z), BlockType::Dirt);
            }
        }
        res.set_voxel(IVec3::new(0, 1, 0), BlockType::Dirt);
        res
    }
}
