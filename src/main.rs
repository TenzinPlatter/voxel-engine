use beryllium::{events::*, *};

use bytemuck::cast_slice;
use gl33::{global_loader::*, *};
use glam::{Mat4, Vec2, Vec3};
use voxel_engine::{
    degrees_to_radians,
    render::{
        buffer::{buffer_data, Buffer, BufferType, VertexArray}, camera::Camera, clear_color, polygon_mode, shader::{ShaderProgram, ShaderUniformType}, texture::Texture, vertex::{Vertex, VertexTex}, PolygonMode
    },
};

type TriIndexes = [u32; 3];

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// make window float with my niri setup
const WINDOW_TITLE: &str = "float";

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

const VERT_SHADER: &str = include_str!("../shaders/vertex.glsl");

const FRAG_SHADER: &str = include_str!("../shaders/fragment.glsl");

fn main() {
    let mut vertices_dirty = true;
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
    let trans = Mat4::IDENTITY;
    let cube_positions = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 5.0, -15.0),
        Vec3::new(-1.5, -2.2, -2.5),
        Vec3::new(-3.8, -2.0, -12.3),
        Vec3::new(2.4, -0.4, -3.5),
        Vec3::new(-1.7, 3.0, -7.5),
        Vec3::new(1.3, -2.0, -2.5),
        Vec3::new(1.5, 2.0, -2.5),
        Vec3::new(1.5, 0.2, -1.5),
        Vec3::new(-1.3, 1.0, -1.5),
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
        allow_high_dpi: false,
        borderless: false,
        resizable: true,
    };

    let win = sdl.create_gl_window(win_args).expect("Failed to create window");
    win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    unsafe { load_global_gl(&|p_name| win.get_proc_address(p_name)) };

    sdl.set_relative_mouse_mode(true).unwrap();

    // Get actual drawable size (may differ from window size)
    let (drawable_width, drawable_height) = win.get_drawable_size();

    // Set viewport to match actual drawable size
    unsafe {
        glViewport(0, 0, drawable_width, drawable_height);
    }

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

    unsafe {
        glEnable(GL_DEPTH_TEST);
    }

    // Create camera looking at the scene
    let mut camera = Camera::looking_at(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO);

    // Delta time tracking
    let mut last_frame_time = sdl.get_ticks();

    'main_loop: loop {
        // Calculate delta time
        let current_frame_time = sdl.get_ticks();
        let delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;
        last_frame_time = current_frame_time;
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            // glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);
            // glDrawArrays(GL_TRIANGLES, 0, 36);
        }

        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                (events::Event::Key { keycode, pressed, .. }, _) => {
                    if pressed {
                        process_input(&mut camera, keycode, delta_time);
                    }
                }
                (events::Event::MouseMotion { x_delta, y_delta, .. }, _) => {
                    camera.process_mouse(x_delta as f32, -y_delta as f32);
                }
                _ => {}
            }
        }

        let view = camera.view_matrix();
        let proj = Mat4::perspective_rh_gl(
            degrees_to_radians(45.0),
            drawable_width as f32 / drawable_height as f32,
            0.1,
            100.0
        );

        // send the data to buffer
        if vertices_dirty {
            vertices_dirty = false;

            buffer_data(BufferType::Array, cast_slice(&vertices), GL_STATIC_DRAW);
        }

        Mat4::set_uniform(&shader_program, "view", view);
        Mat4::set_uniform(&shader_program, "proj", proj);

        let time = sdl.get_ticks() as f32 / 1000.0;
        for (i, cube_pos) in cube_positions.iter().enumerate() {
            let mut cube_model = Mat4::IDENTITY;
            cube_model *= Mat4::from_translation(*cube_pos);
            cube_model *= Mat4::from_axis_angle(
                Vec3::new(1.0, 0.3, 0.5).normalize(),
                if i % 3 == 0 {
                    time * degrees_to_radians(120.0)
                } else {
                    i as f32 * degrees_to_radians(20.0)
                },
            );

            Mat4::set_uniform(&shader_program, "model", cube_model);

            unsafe {
                glDrawArrays(GL_TRIANGLES, 0, 36);
            }
        }

        Mat4::set_uniform(&shader_program, "transform", trans);

        win.swap_window();
    }
}

fn process_input(camera: &mut Camera, keycode: SDL_Keycode, delta_time: f32) {
    #[allow(non_upper_case_globals)]
    match keycode {
        SDLK_w => camera.move_forward(delta_time),
        SDLK_s => camera.move_backward(delta_time),
        SDLK_a => camera.move_left(delta_time),
        SDLK_d => camera.move_right(delta_time),
        SDLK_SPACE => camera.move_up(delta_time),
        SDLK_LSHIFT | SDLK_RSHIFT => camera.move_down(delta_time),
        _ => {}
    }
}
