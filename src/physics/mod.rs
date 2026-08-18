use std::sync::LazyLock;

use glam::Vec3;
use uom::si::{
    f32::{Acceleration, Time},
    time,
};

use crate::physics::hit_info::HitFace;

pub mod dda;
pub mod dda_other;
pub mod hit_info;

pub static PHYSICS_DT: LazyLock<Time> = LazyLock::new(|| Time::new::<time::second>(1.0 / 60.0));
pub static GRAVITY: LazyLock<Acceleration> =
    LazyLock::new(|| Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(-9.81));

#[derive(Clone, Debug)]
pub struct PhysicsBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub size: Vec3,
    pub accumulator: Time,
}

impl PhysicsBody {
    /// Creates a new physics body at the given position and size.
    pub fn new(position: Vec3, size: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            size,
            accumulator: Time::new::<time::second>(0.0),
        }
    }

    /// Determines which face of this physics body is colliding with another.
    /// NOTE: This function assumes that a collision is already occurring.
    pub fn get_collision_face(&self, other: &PhysicsBody) -> HitFace {
        let dx_min = (other.position.x + other.size.x) - self.position.x;
        let dx_max = (self.position.x + self.size.x) - other.position.x;
        let dy_min = (other.position.y + other.size.y) - self.position.y;
        let dy_max = (self.position.y + self.size.y) - other.position.y;
        let dz_min = (other.position.z + other.size.z) - self.position.z;
        let dz_max = (self.position.z + self.size.z) - other.position.z;

        let min_overlap = dx_min
            .min(dx_max)
            .min(dy_min)
            .min(dy_max)
            .min(dz_min)
            .min(dz_max);

        if min_overlap == dx_min {
            HitFace::NegX
        } else if min_overlap == dx_max {
            HitFace::PosX
        } else if min_overlap == dy_min {
            HitFace::PosY
        } else if min_overlap == dy_max {
            HitFace::NegY
        } else if min_overlap == dz_min {
            HitFace::NegZ
        } else {
            HitFace::PosZ
        }
    }

    pub fn extrude(&self, extrusion: Vec3) -> Self {
        let amount = extrusion.length();
        if amount < 0.0 {
            panic!("Extrusion amount must be non-negative");
        }

        let mut new_size = self.size;
        let mut new_position = self.position;

        if extrusion.x > 0.0 {
            new_size.x += amount;
        } else if extrusion.x < 0.0 {
            new_size.x += amount;
            new_position.x -= amount;
        }

        if extrusion.y > 0.0 {
            new_size.y += amount;
        } else if extrusion.y < 0.0 {
            new_size.y += amount;
            new_position.y -= amount;
        }

        if extrusion.z > 0.0 {
            new_size.z += amount;
        } else if extrusion.z < 0.0 {
            new_size.z += amount;
            new_position.z -= amount;
        }

        PhysicsBody {
            position: new_position,
            velocity: Vec3::ZERO,
            size: new_size,
            accumulator: Time::new::<time::second>(0.),
        }
    }
}

pub fn colliding_with_voxel_from_pos(p: &PhysicsBody, vox: Vec3) -> bool {
    let vox_body = PhysicsBody::new(vox, Vec3::ONE);
    colliding_with(p, &vox_body)
}

/// Checks whether two physics bodies are colliding using AABB collision detection.
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
