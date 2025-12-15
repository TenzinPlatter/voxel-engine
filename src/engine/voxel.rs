use glam::{IVec3, Vec2, Vec3};

use crate::{
    Resources,
    engine::block::{BlockType, texture_coordinates_from_block_id},
    physics::PhysicsBody,
    render::vertex::VertexTex,
};

pub struct Voxel {
    pub body: PhysicsBody,
    pub block_type: BlockType,
}

impl Voxel {
    /// Creates a new voxel at the given integer position.
    pub fn new(position: IVec3, block_type: BlockType) -> Self {
        Voxel {
            body: PhysicsBody::new(position.as_vec3(), Vec3::ONE),
            block_type,
        }
    }

    /// Returns the 36 vertices that make up this voxel's cube geometry.
    /// Generates 6 faces (2 triangles each = 6 vertices per face).
    pub fn get_verticies(&self, resources: &Resources) -> [VertexTex; 36] {
        let pos = self.body.position;

        // Define the 6 cube faces with their 4 corners in counter-clockwise order
        // Each face: [bottom_left, bottom_right, top_right, top_left]
        let faces = [
            // Back face (Z-)
            [
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
            ],
            // Front face (Z+)
            [
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
            ],
            // Left face (X-)
            [
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, -0.5, 0.5),
            ],
            // Right face (X+)
            [
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, 0.5),
            ],
            // Bottom face (Y-)
            [
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(-0.5, -0.5, 0.5),
            ],
            // Top face (Y+)
            [
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
            ],
        ];

        // UV coordinates for each corner of a face
        let uvs = dbg!(texture_coordinates_from_block_id(resources, self.block_type).as_uv_corners());

        let mut vertices = [VertexTex::new(Vec3::ZERO, Vec2::ZERO); 36];
        let mut vertex_index = 0;

        // Generate two triangles per face
        for face in &faces {
            // Triangle 1: [0, 1, 2]
            vertices[vertex_index] = VertexTex::new(face[0] + pos, uvs[0]);
            vertices[vertex_index + 1] = VertexTex::new(face[1] + pos, uvs[1]);
            vertices[vertex_index + 2] = VertexTex::new(face[2] + pos, uvs[2]);

            // Triangle 2: [2, 3, 0]
            vertices[vertex_index + 3] = VertexTex::new(face[2] + pos, uvs[2]);
            vertices[vertex_index + 4] = VertexTex::new(face[3] + pos, uvs[3]);
            vertices[vertex_index + 5] = VertexTex::new(face[0] + pos, uvs[0]);

            vertex_index += 6;
        }

        vertices
    }
}
