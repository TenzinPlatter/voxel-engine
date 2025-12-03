use crate::render::{camera::Camera, mesh::Mesh, shader::ShaderProgram};

pub struct Renderer {
    shader_program: ShaderProgram,
}

impl Renderer {
    pub fn new(vertex_shader: &str, fragment_shader: &str) -> Self {
        let shader_program =
            ShaderProgram::from_vert_frag(vertex_shader, fragment_shader).expect("Failed to create shader program");

        let res = Self { shader_program };
        res.bind();

        res
    }

    pub fn bind(&self) {
        self.shader_program.use_program();
    }

    // NOTE: should Viewport be a type?
    pub fn render_mesh(&self, mesh: &Mesh, camera: &Camera, viewport: &Viewport) {

    }
}
