use glam::Vec3;

pub const PHYSICS_DT: f32 = 1. / 120.;

#[derive(Clone, Debug)]
pub struct PhysicsBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub size: Vec3,
    pub accumulator: f32,
}

impl PhysicsBody {
    pub fn new(position: Vec3, size: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            size,
            accumulator: 0.,
        }
    }
}

/// Check whether two physics bodies are colliding using AABB collision detection
/// TODO: when using this do a binary search from start time to end time to see how far the object
/// can be moved before colliding, ~5 steps is probably good
pub fn colliding_with(a: &PhysicsBody, b: &PhysicsBody) -> bool {
    a.position.x < b.position.x + b.size.x
        && a.position.x + a.size.x > b.position.x
        && a.position.y < b.position.y + b.size.y
        && a.position.y + a.size.y > b.position.y
        && a.position.z < b.position.z + b.size.z
        && a.position.z + a.size.z > b.position.z
}
