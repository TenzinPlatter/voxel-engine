use beryllium::*;

use anyhow::Result;
use gl33::{global_loader::*, *};
use glam::Vec3;
use voxel_engine::{
    Resources, State, create_ui_mesh, create_world_mesh, draw_axis,
    engine::world::World,
    get_delta_time,
    input::InputState,
    player::Player,
    process_input_events,
    render::{
        PolygonMode, clear_color, polygon_mode,
        renderer::{Renderer, Viewport},
        setup_2d_rendering, setup_3d_rendering,
    },
    render_ui, render_world, update_player_and_world,
};

const VERT_SHADER_3D: &str = include_str!("../shaders/3d/vertex.glsl");
const FRAG_SHADER_3D: &str = include_str!("../shaders/3d/fragment.glsl");

const VERT_SHADER_2D: &str = include_str!("../shaders/2d/vertex.glsl");
const FRAG_SHADER_2D: &str = include_str!("../shaders/2d/fragment.glsl");

fn main() -> Result<()> {
    env_logger::init();

    let (sdl, win) = voxel_engine::init_sdl_and_win();

    unsafe { load_global_gl(&|p_name| win.get_proc_address(p_name)) };

    sdl.set_relative_mouse_mode(true).unwrap();
    win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

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

    let mut state = State::default();
    let mut world = World::default();
    let renderer = Renderer::new((VERT_SHADER_3D, FRAG_SHADER_3D), (VERT_SHADER_2D, FRAG_SHADER_2D));
    let resources = Resources::build()?;
    let mut player = Player::new(Vec3::new(-3.0, 2.0, -3.0));

    let ui_mesh = create_ui_mesh(&resources, &viewport);
    create_world_mesh(&mut world, &resources);

    clear_color(0.2, 0.3, 0.3, 1.0);
    polygon_mode(PolygonMode::Fill);

    // Delta time tracking
    let mut last_frame_time = sdl.get_ticks();
    let mut input_state = InputState::default();

    'main_loop: loop {
        // Calculate delta time
        let delta_time = get_delta_time(&sdl, last_frame_time);
        last_frame_time = sdl.get_ticks();

        setup_3d_rendering();

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }

        if process_input_events(&sdl, &mut player, &mut input_state, &mut state, &mut world, &resources) {
            break 'main_loop;
        }

        update_player_and_world(&mut state, &mut world, &mut player, &mut input_state, delta_time);

        render_world(&renderer, &world, &player, &viewport);

        draw_axis(&player.camera, &viewport);

        setup_2d_rendering();
        render_ui(&renderer, &ui_mesh, &viewport);

        win.swap_window();
    }

    Ok(())
}
