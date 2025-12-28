use anyhow::Result;
use beryllium::{Sdl, events};
use glam::{IVec3, Vec3};

use crate::{
    engine::{block::BlockType, voxel::Voxel, world::World},
    input::InputState,
    physics::{colliding_with_aabb, dda::get_looking_at_vox_pos, hit_info::HitInfo},
    player::{Player, PlayerState},
    render::atlas::TextureAtlas,
};

pub struct GameState {
    pub state: State,
    pub world: World,
    pub player: Player,
    pub input_state: InputState,
}

pub struct GameResources {
    pub atlas: TextureAtlas,
}

#[derive(Default)]
pub struct State {
    pub last_player: Option<PlayerState>,
    pub current_player: Option<PlayerState>,
    pub looking_at_vox_pos: Option<IVec3>,
    pub selected_block_type: BlockType,
}

impl GameResources {
    pub fn build() -> Result<Self> {
        Ok(Self {
            atlas: TextureAtlas::try_parse_atlas()?,
        })
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            state: State::default(),
            world: World::default(),
            player: Player::new(Vec3::new(-3.0, 2.0, -3.0)),
            input_state: InputState::default(),
        }
    }
}

impl GameState {
    /// Processes input events, updating the player and input state accordingly.
    /// Returns whether a quit event was received.
    pub fn process_input_events(&mut self, sdl: &Sdl, resources: &mut GameResources) -> bool {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => return true,
                (events::Event::Key { keycode, pressed, .. }, _) => {
                    self.input_state.set_key(keycode, pressed);
                }
                (events::Event::MouseMotion { x_delta, y_delta, .. }, _) => {
                    self.player.process_mouse(x_delta as f32, -y_delta as f32);
                }
                (events::Event::MouseButton { button, pressed, .. }, _) => {
                    self.input_state.set_mouse_button(button, pressed);
                }
                _ => {}
            }
        }

        self.state.selected_block_type = if self.input_state.number_key(1).just_pressed {
            BlockType::Dirt
        } else if self.input_state.number_key(2).just_pressed {
            BlockType::Stone
        } else {
            self.state.selected_block_type
        };

        let hit_info = get_looking_at_vox_pos(&self.world, &self.player);
        self.state.looking_at_vox_pos = hit_info.map(|hit| hit.pos);
        if let Some(hit_info) = hit_info {
            self.handle_mouse_presses(resources, &hit_info);
        }

        self.input_state.reset_mouse_buttons();

        false
    }

    pub fn update_player_and_world(&mut self, delta_time: f32) {
        let Self { state, world, player, input_state } = self;

        state.last_player = Some(PlayerState::new(player.step(
            world,
            delta_time,
            input_state,
            state.last_player.as_ref(),
        )));
    }

    pub fn handle_mouse_presses(&mut self, resources: &mut GameResources, hit_info: &HitInfo) {
        let mut dirty = false;

        if self.input_state.mb3.just_pressed {
            self.try_place_block(hit_info);
            dirty = true;
        }

        if let Some(voxpos) = self.state.looking_at_vox_pos
            && let Some(vox) = self.world.voxels.get_mut(&voxpos)
            && self.input_state.mb1.just_pressed
        {
            vox.block_type = match vox.block_type {
                BlockType::Dirt => BlockType::Stone,
                BlockType::Stone => BlockType::Dirt,
            };

            dirty = true;
        }

        if dirty {
            self.world.rebuild_mesh(resources);
        }

        self.input_state.reset_mouse_buttons();
    }

    pub fn try_place_block(&mut self, hit_info: &HitInfo) -> bool {
        let to_place = hit_info.pos + hit_info.normal;
        if self.world.voxels.contains_key(&to_place) {
            return false;
        }

        let vox = Voxel::new(to_place, self.state.selected_block_type);
        if colliding_with_aabb(&vox.body, &self.player.body) {
            return false;
        }

        self.world.voxels.insert(to_place, vox);
        true
    }
}
