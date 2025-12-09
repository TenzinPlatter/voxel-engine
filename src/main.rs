use beryllium::*;

use gl33::{global_loader::*, *};
use glam::Vec3;
use voxel_engine::{
    create_mesh,
    engine::world::World,
    player::Player,
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

    let mut player = Player::new(Vec3::new(-3.0, 10.0, -3.0));

    create_mesh(&mut world, tex);

    clear_color(0.2, 0.3, 0.3, 1.0);
    polygon_mode(PolygonMode::Fill);

    unsafe {
        glEnable(GL_DEPTH_TEST);
    }

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
                    player.camera.input_state.set_key(keycode, pressed);
                }
                (events::Event::MouseMotion { x_delta, y_delta, .. }, _) => {
                    player.camera.process_mouse(x_delta as f32, -y_delta as f32);
                }
                (events::Event::MouseButton { button, pressed, .. }, _) => {
                    if pressed && button == 1 {
                        let pos = player.camera.position.as_ivec3();
                        match world.get_voxel(&pos).is_some() {
                            true => world.remove_voxel(&pos),
                            false => world.set_voxel(pos),
                        };
                        world.rebuild_mesh(Some(tex));
                    }
                }
                _ => {}
            }
        }

        player.camera.handle_move(delta_time);

        renderer.render_mesh(
            world.mesh.as_ref().expect("Mesh shouldve been build on world init"),
            &player.camera,
            &viewport,
        );

        win.swap_window();
    }
}
