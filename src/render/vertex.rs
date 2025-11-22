use gl33::{global_loader::*, *};
use ultraviolet::vec::Vec3;

pub trait Vertex {
    fn configure_attributes();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexColor {
    pub point: Vec3,
    pub color: Option<Vec3>,
    pub texture: Option<Vec3>,
}

impl Vertex for VertexColor {
    fn configure_attributes() {
        unsafe {
            let vec3_size: i32 = size_of::<Vec3>().try_into().unwrap();

            // setup position attribute
            glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                GL_FALSE.0 as u8,
                2 * vec3_size,
                0 as *const _,
            );

            // setup color attribute
            glVertexAttribPointer(
                1,
                3,
                GL_FLOAT,
                GL_FALSE.0 as u8,
                2 * vec3_size,
                vec3_size as *const _,
            );

            glEnableVertexAttribArray(0);
            glEnableVertexAttribArray(1);
        }
    }
}

impl VertexColor {
    pub fn new(point: Vec3) -> Self {
        Self {
            point,
            color: None,
            texture: None,
        }
    }

    pub fn add_color(&mut self, color: Vec3) {
        self.color = Some(color);
    }

    pub fn add_texture_coords(&mut self, p: Vec3) {
        self.texture = Some(p);
    }

    pub fn to_flat(&self) -> Vec<f32> {
        let mut res = vec![self.point.x, self.point.y, self.point.z];

        if let Some(c) = &self.color {
            res.extend_from_slice(&[c.x, c.y, c.z]);
        }

        if let Some(t) = &self.texture {
            res.extend_from_slice(&[t.x, t.y, t.z]);
        }

        res
    }
}
