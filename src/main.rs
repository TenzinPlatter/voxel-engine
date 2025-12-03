use beryllium::{events::*, *};

use gl33::{global_loader::*, *};
use glam::{Mat4, Vec3};
use voxel_engine::{
    create_mesh, degrees_to_radians,
    engine::world::World,
    render::{
        PolygonMode,
        camera::Camera,
        clear_color, polygon_mode,
        shader::{ShaderProgram, ShaderUniformType},
        texture::Texture,
        vertex::{Vertex, VertexTex},
    },
};

const VERT_SHADER: &str = include_str!("../shaders/vertex.glsl");

const FRAG_SHADER: &str = include_str!("../shaders/fragment.glsl");

fn main() {
    let trans = Mat4::IDENTITY;

    let (sdl, win) = voxel_engine::init_sdl_and_win();

    unsafe { load_global_gl(&|p_name| win.get_proc_address(p_name)) };

    sdl.set_relative_mouse_mode(true).unwrap();

    // Get actual drawable size (may differ from window size)
    let (drawable_width, drawable_height) = win.get_drawable_size();

    // Set viewport to match actual drawable size
    unsafe {
        glViewport(0, 0, drawable_width, drawable_height);
    }

    let mut world = World::new(VERT_SHADER, FRAG_SHADER);
    create_mesh(&mut world);

    VertexTex::configure_attributes();

    let tex = Texture::new().expect("Failed to create texture");
    tex.bind();
    Texture::set_image("assets/wood_container.jpg");

    // let shader_program = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    // shader_program.use_program();

    clear_color(0.2, 0.3, 0.3, 1.0);
    polygon_mode(PolygonMode::Fill);

    unsafe {
        glEnable(GL_DEPTH_TEST);
    }

    // Create camera looking at the scene
    let mut camera = Camera::looking_at(Vec3::new(-3.0, 10.0, -3.0), Vec3::ZERO);

    // Delta time tracking
    let mut last_frame_time = sdl.get_ticks();

    'main_loop: loop {
        // Calculate delta time
        let current_frame_time = sdl.get_ticks();
        let delta_time = (current_frame_time - last_frame_time) as f32 / 1000.0;
        last_frame_time = current_frame_time;

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }

        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                (events::Event::Key { keycode, pressed, .. }, _) => {
                    if !pressed {
                        continue;
                    }
                    match keycode {
                        SDLK_SPACE => {
                            let pos = camera.position.as_ivec3();
                            if world.voxel_positions.contains(&pos) {
                                world.voxel_positions.remove(&pos);
                            } else {
                                world.voxel_positions.insert(pos);
                            }
                        }
                        _ => process_input(&mut camera, keycode, delta_time),
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
            100.0,
        );

        Mat4::set_uniform(&world.shader_program, "view", view);
        Mat4::set_uniform(&world.shader_program, "proj", proj);
        Mat4::set_uniform(&world.shader_program, "transform", trans);


        if world.mesh.is_none() {
            create_mesh(&mut world);
        }
        world.draw();

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
