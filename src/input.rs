use beryllium::events::*;
use glam::Vec3;

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
    pub fn as_vel(&self) -> Vec3 {
        let mut accel = Vec3::ZERO;

        if self.forward {
            accel.x -= 1.0;
        }
        if self.back {
            accel.x += 1.0;
        }
        if self.right {
            accel.z -= 1.0;
        }
        if self.left {
            accel.z += 1.0;
        }
        if self.up {
            accel.y += 1.0;
        }
        if self.down {
            accel.y -= 1.0;
        }

        // Normalize to prevent faster diagonal movement
        if accel.length_squared() > 0.0 {
            accel = accel.normalize();
        }

        accel
    }

    /// Update the state when a key is pressed or released
    pub fn set_key(&mut self, keycode: SDL_Keycode, pressed: bool) {
        #[allow(non_upper_case_globals)]
        match keycode {
            SDLK_w => self.forward = pressed,
            SDLK_s => self.back = pressed,
            SDLK_a => self.left = pressed,
            SDLK_d => self.right = pressed,
            SDLK_SPACE => self.up = pressed,
            SDLK_c | SDLK_LCTRL => self.down = pressed,
            _ => {},
        }
    }
}
