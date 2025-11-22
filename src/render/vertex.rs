use bytemuck::{Pod, Zeroable};
use gl33::{global_loader::*, *};
use glam::{Vec2, Vec3};

pub trait Vertex {
    fn configure_attributes();
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct VertexColor {
    pub point: Vec3,
    pub color: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct VertexTex {
    pub point: Vec3,
    pub tex: Vec2,
}

impl Vertex for VertexTex {
    fn configure_attributes() {
        unsafe {
            let vec3_size: i32 = size_of::<Vec3>().try_into().unwrap();
            let vec2_size: i32 = size_of::<Vec2>().try_into().unwrap();
            let stride = vec3_size + vec2_size;
            // setup position attribute
            glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE.0 as u8, stride, 0 as *const _);
            // setup color attribute
            glVertexAttribPointer(
                1,
                2,
                GL_FLOAT,
                GL_FALSE.0 as u8,
                stride,
                // texture coords are offset from beginning by the size of the vec3 with the point
                vec3_size as *const _,
            );

            glEnableVertexAttribArray(0);
            glEnableVertexAttribArray(1);
        }
    }
}

impl VertexTex {
    pub fn new(point: Vec3, tex: Vec2) -> Self {
        Self { point, tex }
    }
    pub fn set_tex(&mut self, tex: Vec2) {
        self.tex = tex;
    }
}

impl Vertex for VertexColor {
    fn configure_attributes() {
        unsafe {
            let vec3_size: i32 = size_of::<Vec3>() as i32;

            // setup position attribute
            glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE.0 as u8, 2 * vec3_size, 0 as *const _);

            // setup color attribute
            glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE.0 as u8, 2 * vec3_size, vec3_size as *const _);

            glEnableVertexAttribArray(0);
            glEnableVertexAttribArray(1);
        }
    }
}

impl VertexColor {
    pub fn new(point: Vec3, color: Vec3) -> Self {
        Self { point, color }
    }

    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }
}
