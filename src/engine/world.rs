use glam::{IVec3, Mat4};

use crate::{
    GameResources,
    engine::{block::BlockType, voxel::Voxel},
    physics::{PhysicsBody, colliding_with, hit_info::HitInfo},
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
        let mut verticies = vec![];

        // voxels positions are top x, z corner
        for vox in self.voxels.values() {
            verticies.extend(vox.get_verticies(resources));
        }

        self.mesh = Some(Mesh::new(
            &verticies,
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
    pub fn is_colliding(&self, other: &PhysicsBody) -> Option<HitInfo> {
        // TODO: optimize to not check every square
        let collisions = self
            .voxels
            .values()
            .filter(|v| colliding_with(&v.body, other));

        let closest = collisions.min_by(|a, b| {
            let da = a.body.position.distance_squared(other.position);
            let db = b.body.position.distance_squared(other.position);
            da.partial_cmp(&db).unwrap()
        });

        if let Some(closest) = closest
        {
            let face = other.get_collision_face(&closest.body);
            Some(HitInfo::new(closest.body.position.as_ivec3(), face))
        } else {
            None
        }
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
