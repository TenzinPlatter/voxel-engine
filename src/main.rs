use beryllium::{events::*, *};

use gl33::{global_loader::*, *};
use glam::Vec3;
use voxel_engine::{
    create_mesh,
    engine::world::World,
    render::{
        PolygonMode,
        camera::Camera,
        clear_color, polygon_mode,
        renderer::{Renderer, Viewport},
        texture::Texture,
    },
};

const VERT_SHADER: &str = include_str!("../shaders/vertex.glsl");

const FRAG_SHADER: &str = include_str!("../shaders/fragment.glsl");

fn main() {
    let (sdl, win) = voxel_engine::init_sdl_and_win();

    unsafe { load_global_gl(&|p_name| win.get_proc_address(p_name)) };

    sdl.set_relative_mouse_mode(true).unwrap();

    // Get actual drawable size (may differ from window size)
    let (drawable_width, drawable_height) = win.get_drawable_size();
    let viewport = Viewport {
        width: drawable_width,
        height: drawable_height,
    };

    // Set viewport to match actual drawable size
    unsafe {
        glViewport(0, 0, drawable_width, drawable_height);
    }

    let mut world = World::default();
    let renderer = Renderer::new(VERT_SHADER, FRAG_SHADER);
    let tex = Texture::new().expect("Failed to create texture");
    tex.bind();
    Texture::set_image("assets/wood_container.jpg");

    world.mesh = Some(create_mesh(tex));

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
                            if !world.set_voxel(pos) {
                                // was not added and therefore already existed, so remove it
                                world.remove_voxel(&pos);
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

        if let Some(mesh) = &world.mesh {
            renderer.render_mesh(mesh, &camera, &viewport);
        } else {
            // shouldn't actually happen
            world.mesh = Some(create_mesh(tex));
        }

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
