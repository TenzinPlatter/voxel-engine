use glam::Vec3;

use crate::physics::PhysicsBody;

pub struct Voxel {
    position: Vec3,
    velocity: Vec3,
}

impl PhysicsBody for Voxel {
    fn colliding_with(&self, other: &dyn PhysicsBody) -> bool {
        let (left, right): (&dyn PhysicsBody, &dyn PhysicsBody) = if self.position().x < other.position().x {
            (self, other)
        } else {
            (other, self)
        };

        // AABB collision detection
        let left_pos = left.position();
        let left_size = left.size();
        let right_pos = right.position();
        let right_size = right.size();

        left_pos.x + left_size.x > right_pos.x &&
        left_pos.x < right_pos.x + right_size.x &&
        left_pos.y + left_size.y > right_pos.y &&
        left_pos.y < right_pos.y + right_size.y &&
        left_pos.z + left_size.z > right_pos.z &&
        left_pos.z < right_pos.z + right_size.z
    }

    fn position(&self) -> Vec3 {
        self.position
    }

    fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }

    fn size(&self) -> Vec3 {
        Vec3::ONE
    }
}
