use beryllium::{
    events::{SDLK_DOWN, SDLK_LEFT, SDLK_RIGHT, SDLK_UP},
    *,
};

use bytemuck::cast_slice;
use gl33::{global_loader::*, *};
use glam::{Vec2, Vec3};
use voxel_engine::render::{
    buffer::{buffer_data, Buffer, BufferType, VertexArray}, clear_color, polygon_mode, shader::ShaderProgram, texture::Texture, vertex::{Vertex, VertexColor, VertexTex}, PolygonMode
};

type TriIndexes = [u32; 3];

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// make window float with my niri setup
const WINDOW_TITLE: &str = "(float)";

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

const VERT_SHADER: &str = include_str!("../shaders/vertex_tex.glsl");

const FRAG_SHADER: &str = include_str!("../shaders/fragment.glsl");

fn main() {
    let mut verticies_dirty = true;
    let mut verticies = [
        VertexTex::new(Vec3::new(0.5, 0.5, 0.0), Vec2::new(1.0, 1.0)),
        VertexTex::new(Vec3::new(0.5, -0.5, 0.0), Vec2::new(1.0, 0.0)),
        VertexTex::new(Vec3::new(-0.5, -0.5, 0.0), Vec2::new(0.0, 0.0)),
        VertexTex::new(Vec3::new(-0.5, 0.5, 0.0), Vec2::new(0.0, 1.0)),
    ];

    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    #[cfg(target_os = "macos")]
    sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE).unwrap();

    let win_args = video::CreateWinArgs {
        title: WINDOW_TITLE,
        width: WIDTH,
        height: HEIGHT,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let win = sdl.create_gl_window(win_args).expect("Failed to create window");
    win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    unsafe { load_global_gl(&|p_name| win.get_proc_address(p_name)) };

    let vao = VertexArray::new().expect("Failed to create VAO");
    vao.bind();

    // generate and bind vertex buffer object
    let vbo = Buffer::new().expect("Failed to create VBO");
    vbo.bind(BufferType::Array);

    // generate element buffer object to store triangle indicies
    let ebo = Buffer::new().expect("Failed to create EBO");
    ebo.bind(BufferType::ElementArray);
    buffer_data(BufferType::ElementArray, bytemuck::cast_slice(&INDICES), GL_STATIC_DRAW);

    let tex = Texture::new().expect("Failed to create texture");
    tex.bind();
    Texture::set_image("assets/wood_container.jpg");

    let shader_program = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

    VertexTex::configure_attributes();

    clear_color(0.2, 0.3, 0.3, 1.0);
    polygon_mode(PolygonMode::Fill);

    'main_loop: loop {
        // send the data to buffer
        if verticies_dirty {
            verticies_dirty = false;

            buffer_data(BufferType::Array, cast_slice(&verticies), GL_STATIC_DRAW);
        }

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);
        }

        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                (events::Event::Key { keycode, pressed, .. }, _) => {
                    if !pressed {
                        continue;
                    }
                    let mut other = None;
                    match keycode {
                        SDLK_UP => {
                            other = Some(Vec3::new(0., 0.1, 0.));
                        }
                        SDLK_DOWN => {
                            other = Some(Vec3::new(0., -0.1, 0.));
                        }
                        SDLK_LEFT => {
                            other = Some(Vec3::new(-0.1, 0., 0.));
                        }
                        SDLK_RIGHT => {
                            other = Some(Vec3::new(0.1, 0., 0.));
                        }
                        _ => {}
                    }

                    if let Some(other) = other {
                        verticies_dirty = true;
                        for v in verticies.iter_mut() {
                            v.point += other;
                        }
                    }
                }

                _ => {}
            }
        }

        win.swap_window();
    }
}
