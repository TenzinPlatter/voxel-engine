pub mod engine;
pub mod input;
pub mod render;
pub mod physics;
pub mod player;

use beryllium::{video::GlWindow, *};
use glam::IVec3;

use crate::{
    engine::{block::{parse_block_atlas, BlockAtlas}, world::World}, player::PlayerState, render::texture::Texture
};

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 900;

// make window float with my niri setup
const WINDOW_TITLE: &str = "(float)";

#[derive(Default)]
pub struct State {
    pub last_player: Option<PlayerState>,
    pub current_player: Option<PlayerState>,
}

pub struct Resources {
    block_atlas: BlockAtlas,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            block_atlas: parse_block_atlas(),
        }
    }
}

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
pub fn create_mesh(world: &mut World, tex: Texture, resources: &Resources) {
    for z in -32..32 {
        for x in -32..32 {
            world.set_voxel(IVec3::new(x, 0, z));
        }
    }

    world.rebuild_mesh(Some(tex), resources);
}
