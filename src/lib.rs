use anyhow::Result;
use beryllium::{video::GlWindow, *};
use glam::{IVec3, Mat4, Vec2, Vec3};

use crate::{
    engine::{block::BlockType, game::{GameResources, GameState}, voxel::Voxel, world::World},
    input::InputState,
    physics::{colliding_with_aabb, dda::get_looking_at_vox_pos, hit_info::HitInfo},
    player::{Player, PlayerState},
    render::{
        atlas::TextureAtlas,
        camera::Camera,
        debug_line::draw_debug_line,
        mesh::Mesh,
        renderer::{Renderer, Viewport},
        vertex::Vertex2D,
    },
};

pub mod engine;
pub mod input;
pub mod physics;
pub mod player;
pub mod render;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 900;

// make window float with my niri setup
const WINDOW_TITLE: &str = "(float)";

/// Initializes SDL and creates an OpenGL window with default settings.
pub fn init_sdl_and_win() -> (Sdl, GlWindow) {
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
        resizable: false,
    };

    let win = sdl.create_gl_window(win_args).expect("Failed to create window");
    win.set_swap_interval(video::GlSwapInterval::Immediate).unwrap();

    (sdl, win)
}

/// Converts degrees to radians.
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Converts radians to degrees.
pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

/// Creates a flat ground mesh in the world from -32 to 32 on x and z axes.
pub fn create_world_mesh(game: &mut GameState, resources: &GameResources) {
    for z in -32..32 {
        for x in -32..32 {
            game.world.set_voxel(IVec3::new(x, 0, z));
        }
    }

    game.world.set_voxel(IVec3::new(0, 1, 0));

    game.world.rebuild_mesh(resources);
}

/// Returns the delta time since the last frame in seconds.
pub fn get_delta_time(sdl: &Sdl, last_frame_time: u32) -> f32 {
    let current_frame_time = sdl.get_ticks();
    let delta_time = current_frame_time - last_frame_time;

    delta_time as f32 / 1000.0
}

pub fn render_world(game: &mut GameState, renderer: &Renderer, viewport: &Viewport) {
    renderer.render_mesh_3d(
        game.world.mesh.as_ref().expect("Mesh shouldve been build on world init"),
        &game.player.camera,
        viewport,
    );
}

pub fn create_ui_mesh(resources: &GameResources, viewport: &Viewport) -> Mesh {
    let crosshair_size = 16.0;
    let half_size = crosshair_size / 2.0;
    let center = Vec2::new(viewport.width as f32 / 2.0, viewport.height as f32 / 2.0);

    let uvs = resources
        .atlas
        .textures
        .get("crosshair")
        .expect("Crosshair texture missing from atlas")
        .to_uvs();

    let vertices = vec![
        Vertex2D {
            position: Vec2::new(center.x - half_size, center.y - half_size),
            tex: uvs[0],
        },
        Vertex2D {
            position: Vec2::new(center.x + half_size, center.y - half_size),
            tex: uvs[1],
        },
        Vertex2D {
            position: Vec2::new(center.x + half_size, center.y + half_size),
            tex: uvs[2],
        },
        Vertex2D {
            position: Vec2::new(center.x + half_size, center.y + half_size),
            tex: uvs[2],
        },
        Vertex2D {
            position: Vec2::new(center.x - half_size, center.y + half_size),
            tex: uvs[3],
        },
        Vertex2D {
            position: Vec2::new(center.x - half_size, center.y - half_size),
            tex: uvs[0],
        },
    ];

    Mesh::new(&vertices, Mat4::IDENTITY, resources.atlas.texture)
}

pub fn render_ui(renderer: &Renderer, mesh: &Mesh, viewport: &Viewport) {
    renderer.render_mesh_2d(mesh, viewport);
}

pub fn draw_axis(camera: &Camera, viewport: &Viewport) {
    let axis_length = 10.0;
    // draw at y=1 to be above ground
    let origin = Vec3::ZERO.with_y(1.);

    // X axis in red
    draw_debug_line(
        origin,
        origin + Vec3::new(axis_length, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        camera,
        viewport,
    );

    // Y axis in green
    draw_debug_line(
        origin,
        origin + Vec3::new(0.0, axis_length, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        camera,
        viewport,
    );

    // Z axis in blue
    draw_debug_line(
        origin,
        origin + Vec3::new(0.0, 0.0, axis_length),
        Vec3::new(0.0, 0.0, 1.0),
        camera,
        viewport,
    );
}
