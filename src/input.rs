use beryllium::events::*;
use glam::Vec3;

#[derive(Default, Debug)]
pub struct KeyState {
    pub is_pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

pub enum MovementKeys {
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
}

/// Represents the key events that occured in a single frame
#[derive(Default, Debug)]
pub struct InputState {
    pub forward: KeyState,
    pub back: KeyState,
    pub left: KeyState,
    pub right: KeyState,
    pub up: KeyState,
}

impl KeyState {
    /// Creates a key state by comparing previous and current press states.
    fn from_pressed_last_and_curr(last_pressed: bool, curr_pressed: bool) -> Self {
        Self {
            is_pressed: curr_pressed,
            just_pressed: !last_pressed && curr_pressed,
            just_released: last_pressed && !curr_pressed,
        }
    }
}

impl InputState {
    /// Converts the current input state to a normalized velocity vector.
    pub fn as_vel(&self) -> Vec3 {
        let mut accel = Vec3::ZERO;

        if self.forward.is_pressed {
            accel.x += 1.;
        }

        if self.back.is_pressed {
            accel.x -= 1.;
        }

        if self.left.is_pressed {
            accel.z -= 1.;
        }

        if self.right.is_pressed {
            accel.z += 1.;
        }

        if self.up.is_pressed {
            accel.y += 1.;
        }

        // Normalize to prevent faster diagonal movement
        if accel.length_squared() > 0.0 {
            accel = accel.normalize();
        }

        accel
    }

    /// Updates the state when a key is pressed or released.
    pub fn set_key(&mut self, keycode: SDL_Keycode, pressed: bool) {
        #[allow(non_upper_case_globals)]
        match keycode {
            SDLK_w => self.forward = KeyState::from_pressed_last_and_curr(self.forward.is_pressed, pressed),
            SDLK_s => self.back = KeyState::from_pressed_last_and_curr(self.back.is_pressed, pressed),
            SDLK_a => self.left = KeyState::from_pressed_last_and_curr(self.left.is_pressed, pressed),
            SDLK_d => self.right = KeyState::from_pressed_last_and_curr(self.right.is_pressed, pressed),
            SDLK_SPACE => self.up = KeyState::from_pressed_last_and_curr(self.up.is_pressed, pressed),
            _ => {}
        }
    }
}
