use anyhow::Result;
use beryllium::{video::GlWindow, *};
use glam::{IVec3, Mat4, Vec2};

use crate::{
    engine::{block::BlockType, world::World},
    input::InputState,
    player::{Player, PlayerState},
    render::{
        atlas::TextureAtlas,
        camera::Camera,
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

#[derive(Default)]
pub struct State {
    pub last_player: Option<PlayerState>,
    pub current_player: Option<PlayerState>,
    pub looking_at_vox_pos: Option<IVec3>,
}

pub struct Resources {
    atlas: TextureAtlas,
}

impl Resources {
    pub fn build() -> Result<Self> {
        Ok(Self {
            atlas: TextureAtlas::try_parse_atlas()?,
        })
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
pub fn create_world_mesh(world: &mut World, resources: &Resources) {
    for z in -32..32 {
        for x in -32..32 {
            world.set_voxel(IVec3::new(x, 0, z));
        }
    }

    world.rebuild_mesh(resources);
}

/// Returns the delta time since the last frame in seconds.
pub fn get_delta_time(sdl: &Sdl, last_frame_time: u32) -> f32 {
    let current_frame_time = sdl.get_ticks();
    let delta_time = current_frame_time - last_frame_time;

    delta_time as f32 / 1000.0
}

/// Processes input events, updating the player and input state accordingly.
/// Returns None if a quit event is received, otherwise returns Some(clicked)
pub fn process_input_events(sdl: &Sdl, player: &mut Player, input_state: &mut InputState) -> Option<bool> {
    let mut clicked = false;
    while let Some(event) = sdl.poll_events() {
        match event {
            (events::Event::Quit, _) => return None,
            (events::Event::Key { keycode, pressed, .. }, _) => {
                input_state.set_key(keycode, pressed);
            }
            (events::Event::MouseMotion { x_delta, y_delta, .. }, _) => {
                player.process_mouse(x_delta as f32, -y_delta as f32);
            }
            (events::Event::MouseButton { button, pressed, .. }, _) => {
                clicked = button == 1 && pressed;
            }
            _ => {}
        }
    }

    Some(clicked)
}

pub fn update_player_and_world(
    state: &mut State,
    world: &mut World,
    player: &mut Player,
    resources: &Resources,
    input_state: &mut InputState,
    delta_time: f32,
    clicked: bool,
) {
    state.last_player = Some(PlayerState::new(player.step(
        world,
        delta_time,
        input_state,
        state.last_player.as_ref(),
    )));

    world.set_looking_at_vox(state, player);
    if clicked
        && let Some(voxpos) = state.looking_at_vox_pos
        && let Some(vox) = world.voxels.get_mut(&voxpos)
    {
        vox.block_type = match vox.block_type {
            BlockType::Dirt => BlockType::Stone,
            BlockType::Stone => BlockType::Dirt,
        };

        world.rebuild_mesh(resources);
    }
}

pub fn render_world(renderer: &Renderer, world: &World, player: &Player, viewport: &Viewport) {
    renderer.render_mesh_3d(
        world.mesh.as_ref().expect("Mesh shouldve been build on world init"),
        &player.camera,
        viewport,
    );
}

pub fn create_ui_mesh(resources: &Resources, viewport: &Viewport) -> Mesh {
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

    println!("{:?}", vertices);

    Mesh::new(&vertices, Mat4::IDENTITY, resources.atlas.texture)
}

pub fn render_ui(renderer: &Renderer, mesh: &Mesh, viewport: &Viewport) {
    renderer.render_mesh_2d(mesh, viewport);
}
