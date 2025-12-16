pub mod buffer;
pub mod camera;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod vertex;
pub mod renderer;
pub mod atlas;

use bytemuck::cast_slice;
use gl33::{global_loader::*, *};
use glam::{IVec3, Mat4, Vec2, Vec3};

use crate::render::{
    buffer::buffer_data,
    shader::{ShaderProgram, ShaderUniformType},
    vertex::VertexTex,
};

/// The polygon display modes you can set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    /// Just show the points.
    Point = GL_POINT.0 as isize,
    /// Just show the lines.
    Line = GL_LINE.0 as isize,
    /// Fill in the polygons.
    Fill = GL_FILL.0 as isize,
}

/// Sets the font and back polygon mode to the mode given.
pub fn polygon_mode(mode: PolygonMode) {
    unsafe { glPolygonMode(GL_FRONT_AND_BACK, GLenum(mode as u32)) };
}

/// Sets the clear color for the framebuffer.
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        glClearColor(r, g, b, a);
    }
}

/// Renders a unit cube at the given position with optional rotation.
pub fn draw_voxel_at(shader_program: &ShaderProgram, pos: &IVec3, rotation_mat: Option<Mat4>) {
    let vertices = [
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

    // TODO: nasty very slow
    buffer_data(buffer::BufferType::Array, cast_slice(&vertices), GL_STATIC_DRAW);

    let mut model = Mat4::IDENTITY;
    model *= Mat4::from_translation(pos.as_vec3());
    if let Some(rot) = rotation_mat {
        model *= rot;
    }

    Mat4::set_uniform(shader_program, "model", model);

    unsafe {
        glDrawArrays(GL_TRIANGLES, 0, vertices.len() as i32);
    }
}
