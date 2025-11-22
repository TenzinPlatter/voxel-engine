use gl33::{global_loader::*, *};

pub mod vertex;
pub mod buffer;
pub mod shader;

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

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        glClearColor(r, g, b, a);
    }
}
