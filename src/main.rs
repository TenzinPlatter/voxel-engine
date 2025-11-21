use beryllium::{
    events::{SDLK_DOWN, SDLK_LEFT, SDLK_RIGHT, SDLK_UP},
    *,
};
use bytemuck::cast_slice;
use gl33::{global_loader::*, *};

use voxel_engine::{
    buffer_data, clear_color, polygon_mode, Buffer, BufferType, PolygonMode, ShaderProgram, Vec3, Vertex, VertexArray
};

type TriIndexes = [u32; 3];

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// make window float with my niri setup
const WINDOW_TITLE: &str = "(float)";

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

const VERT_SHADER: &str = include_str!("../shaders/vertex.glsl");

const FRAG_SHADER: &str = include_str!("../shaders/fragment.glsl");

fn setup_verticies() -> [Vertex; 4] {
    let mut verticies: [Vertex; 4] = [
        Vertex::new(Vec3::new(0.5, 0.5, 0.0)),
        Vertex::new(Vec3::new(0.5, -0.5, 0.0)),
        Vertex::new(Vec3::new(-0.5, -0.5, 0.0)),
        Vertex::new(Vec3::new(-0.5, 0.5, 0.0))
    ];

    verticies[0].add_color(Vec3::new(1., 0., 0.));
    verticies[1].add_color(Vec3::new(0., 1., 0.));
    verticies[2].add_color(Vec3::new(0., 0., 1.));
    verticies[3].add_color(Vec3::new(0., 0., 0.));

    verticies
}

fn main() {
    let mut verticies_dirty = true;
    let mut verticies = setup_verticies();

    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    #[cfg(target_os = "macos")]
    sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
        .unwrap();

    let win_args = video::CreateWinArgs {
        title: WINDOW_TITLE,
        width: WIDTH,
        height: HEIGHT,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let win = sdl
        .create_gl_window(win_args)
        .expect("Failed to create window");
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
    buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        GL_STATIC_DRAW,
    );

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

    let shader_program = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

    clear_color(0.2, 0.3, 0.3, 1.0);
    polygon_mode(PolygonMode::Fill);

    'main_loop: loop {
        // send the data to buffer
        if verticies_dirty {
            verticies_dirty = false;

            let flat: Vec<f32> = verticies.iter()
                .flat_map(|v| v.to_flat())
                .collect();
            buffer_data(
                BufferType::Array,
                cast_slice(&flat),
                GL_STATIC_DRAW,
            );
        }

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);
        }

        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                (
                    events::Event::Key {
                        keycode, pressed, ..
                    },
                    _,
                ) => {
                    if !pressed {
                        continue;
                    }

                    match keycode {
                        SDLK_UP => {
                            let other = Vec3::new(0., 0.1, 0.);
                            verticies = verticies.map(|mut v| {
                                v.point.add(&other);
                                v
                            });
                            verticies_dirty = true;
                        }

                        SDLK_DOWN => {
                            let other = Vec3::new(0., -0.1, 0.);
                            verticies = verticies.map(|mut v| {
                                v.point.add(&other);
                                v
                            });
                            verticies_dirty = true;
                        }

                        SDLK_LEFT => {
                            let other = Vec3::new(-0.1, 0., 0.);
                            verticies = verticies.map(|mut v| {
                                v.point.add(&other);
                                v
                            });
                            verticies_dirty = true;
                        }

                        SDLK_RIGHT => {
                            let other = Vec3::new(0.1, 0., 0.);
                            verticies = verticies.map(|mut v| {
                                v.point.add(&other);
                                v
                            });
                            verticies_dirty = true;
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        win.swap_window();
    }
}
