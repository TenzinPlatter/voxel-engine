use beryllium::events::*;

/// Tracks the current state of input keys for smooth, continuous movement
#[derive(Default)]
pub struct InputState {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

impl InputState {
    /// Update the state when a key is pressed or released
    pub fn set_key(&mut self, keycode: SDL_Keycode, pressed: bool) {
        #[allow(non_upper_case_globals)]
        match keycode {
            SDLK_w => self.forward = pressed,
            SDLK_s => self.back = pressed,
            SDLK_a => self.left = pressed,
            SDLK_d => self.right = pressed,
            SDLK_SPACE => self.up = pressed,
            SDLK_c => self.down = pressed,
            _ => {},
        }
    }
}
