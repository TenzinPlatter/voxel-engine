use glam::Vec3;

use crate::{physics::PhysicsBody, render::camera::Camera};

pub struct Player {
    position: Vec3,
    velocity: Vec3,
    pub camera: Camera,
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            camera: Camera::new(position),
        }
    }
}

impl PhysicsBody for Player {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn size(&self) -> Vec3 {
        Vec3::new(1., 2., 1.)
    }

    fn translate(&mut self, delta: Vec3) {
       self.position += delta;
    }
}
