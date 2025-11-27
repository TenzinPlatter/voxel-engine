use std::collections::HashSet;

use glam::IVec3;

use crate::render::mesh::Mesh;

#[derive(Default)]
pub struct World {
    pub voxels: HashSet<IVec3>,
    pub mesh: Option<Mesh>,
}
