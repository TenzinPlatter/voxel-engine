use glam::{IVec3, Vec2, Vec3};

use crate::{
    physics::PhysicsBody,
    render::vertex::VertexTex,
};

pub struct Voxel {
    position: Vec3,
    velocity: Vec3,
}

impl Voxel {
    pub fn new(position: IVec3) -> Self {
        Voxel {
            position: position.as_vec3(),
            velocity: Vec3::ZERO,
        }
    }

    pub fn get_verticies(&self) -> [VertexTex; 36] {
        let mut vertices = [
            VertexTex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0, 0.0)),
            VertexTex::new(Vec3::new(0.5, -0.5, -0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, -0.5), Vec2::new(1.0, 1.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, -0.5), Vec2::new(1.0, 1.0)),
            VertexTex::new(Vec3::new(-0.5, 0.5, -0.5), Vec2::new(0.0, 1.0)),
            VertexTex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0, 0.0)),
            VertexTex::new(Vec3::new(-0.5, -0.5, 0.5), Vec2::new(0.0, 0.0)),
            VertexTex::new(Vec3::new(0.5, -0.5, 0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, 0.5), Vec2::new(1.0, 1.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, 0.5), Vec2::new(1.0, 1.0)),
            VertexTex::new(Vec3::new(-0.5, 0.5, 0.5), Vec2::new(0.0, 1.0)),
            VertexTex::new(Vec3::new(-0.5, -0.5, 0.5), Vec2::new(0.0, 0.0)),
            VertexTex::new(Vec3::new(-0.5, 0.5, 0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(-0.5, 0.5, -0.5), Vec2::new(1.0, 1.0)),
            VertexTex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0, 1.0)),
            VertexTex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0, 1.0)),
            VertexTex::new(Vec3::new(-0.5, -0.5, 0.5), Vec2::new(0.0, 0.0)),
            VertexTex::new(Vec3::new(-0.5, 0.5, 0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, 0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, -0.5), Vec2::new(1.0, 1.0)),
            VertexTex::new(Vec3::new(0.5, -0.5, -0.5), Vec2::new(0.0, 1.0)),
            VertexTex::new(Vec3::new(0.5, -0.5, -0.5), Vec2::new(0.0, 1.0)),
            VertexTex::new(Vec3::new(0.5, -0.5, 0.5), Vec2::new(0.0, 0.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, 0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0, 1.0)),
            VertexTex::new(Vec3::new(0.5, -0.5, -0.5), Vec2::new(1.0, 1.0)),
            VertexTex::new(Vec3::new(0.5, -0.5, 0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(0.5, -0.5, 0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(-0.5, -0.5, 0.5), Vec2::new(0.0, 0.0)),
            VertexTex::new(Vec3::new(-0.5, -0.5, -0.5), Vec2::new(0.0, 1.0)),
            VertexTex::new(Vec3::new(-0.5, 0.5, -0.5), Vec2::new(0.0, 1.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, -0.5), Vec2::new(1.0, 1.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, 0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(0.5, 0.5, 0.5), Vec2::new(1.0, 0.0)),
            VertexTex::new(Vec3::new(-0.5, 0.5, 0.5), Vec2::new(0.0, 0.0)),
            VertexTex::new(Vec3::new(-0.5, 0.5, -0.5), Vec2::new(0.0, 1.)),
        ];

        for v in vertices.iter_mut() {
            v.position += self.position;
        }

        vertices
    }
}

impl PhysicsBody for Voxel {
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
