use glam::Mat4;

use crate::{degrees_to_radians, render::{camera::Camera, mesh::Mesh, shader::{ShaderProgram, ShaderUniformType}}};

pub struct Renderer {
    shader_program: ShaderProgram,
}

// TEMP: should this live here? should it exist?
pub struct Viewport {
    pub width: i32,
    pub height: i32,
}

impl Renderer {
    /// Creates a new renderer with the given vertex and fragment shaders.
    pub fn new(vertex_shader: &str, fragment_shader: &str) -> Self {
        let shader_program =
            ShaderProgram::from_vert_frag(vertex_shader, fragment_shader).expect("Failed to create shader program");

        let res = Self { shader_program };
        // TODO: does this need to happen?
        res.bind();

        res
    }

    /// Binds the renderer's shader program for use.
    pub fn bind(&self) {
        self.shader_program.use_program();
    }

    /// Renders a mesh with the given camera and viewport settings.
    pub fn render_mesh(&self, mesh: &Mesh, camera: &Camera, viewport: &Viewport) {
        let view = camera.view_matrix();
        let proj = Mat4::perspective_rh_gl(
            degrees_to_radians(45.0),
            viewport.width as f32 / viewport.height as f32,
            0.1,
            100.0,
        );

        Mat4::set_uniform(&self.shader_program, "view", view);
        Mat4::set_uniform(&self.shader_program, "proj", proj);
        Mat4::set_uniform(&self.shader_program, "transform", mesh.transform);

        mesh.draw(&self.shader_program);
    }
}
