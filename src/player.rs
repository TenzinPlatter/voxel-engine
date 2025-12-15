use glam::Vec3;

use crate::{
    engine::world::World,
    input::InputState,
    physics::{GRAVITY, PHYSICS_DT, PhysicsBody},
    render::camera::Camera,
};

pub(crate) const DEFAULT_MOUSE_SENS: f32 = 0.2;
pub(crate) const DEFAULT_PLAYER_SPEED: f32 = 6.0;
pub(crate) const DEFAULT_PLAYER_JUMP_HEIGHT: f32 = 1.25;
pub(crate) const DEFAULT_PLAYER_REACH: f32 = 5.0;

#[derive(Debug)]
pub struct PlayerState {
    body: PhysicsBody,
}

pub struct Player {
    body: PhysicsBody,

    mouse_sensitivity: f32,
    /// multiplier for a normalized velocity vector, player speed
    move_speed: f32,

    pub camera: Camera,
}

impl PlayerState {
    /// Creates a new player state snapshot from a physics body.
    pub fn new(body: PhysicsBody) -> Self {
        Self { body }
    }
}

impl Player {
    /// Creates a new player at the given position with default settings.
    pub fn new(position: Vec3) -> Self {
        Self {
            body: PhysicsBody::new(position, Vec3::new(0.8, 1.8, 0.8)),
            camera: Camera::looking_at(position, Vec3::ZERO),
            mouse_sensitivity: DEFAULT_MOUSE_SENS,
            move_speed: DEFAULT_PLAYER_SPEED,
        }
    }

    /// Updates the player for one frame, handling input, physics, and camera interpolation.
    pub fn step(
        &mut self,
        world: &World,
        frame_delta: f32,
        input_state: &mut InputState,
        last_player_state: Option<&PlayerState>,
    ) -> PhysicsBody {
        // TODO: probably some position clamping stuff?

        let input_vel = self.get_input_vel_xz(input_state);
        self.body.velocity = input_vel.with_y(self.body.velocity.y);

        self.body.accumulator += frame_delta;
        while self.body.accumulator > PHYSICS_DT {
            let is_colliding = world.is_colliding(&self.body);

            if !is_colliding {
                self.body.velocity.y += GRAVITY * PHYSICS_DT;
            } else {
                self.body.velocity.y = 0.;
            }

            if input_state.up.just_pressed && is_colliding {
                self.body.velocity.y = get_initial_jump_vel(DEFAULT_PLAYER_JUMP_HEIGHT);
            }

            self.body.position += self.body.velocity * PHYSICS_DT;
            self.body.accumulator -= PHYSICS_DT;
        }

        self.camera.position = self.get_updated_camera_pos(last_player_state);
        self.camera.update_vectors();
        self.body.clone()
    }

    /// Processes mouse movement to rotate the camera.
    pub fn process_mouse(&mut self, x_offset: f32, y_offset: f32) {
        let x_offset = x_offset * self.mouse_sensitivity * 0.01;
        let y_offset = y_offset * self.mouse_sensitivity * 0.01;

        self.camera.yaw += x_offset;
        self.camera.pitch += y_offset;

        // Constrain pitch to prevent gimbal lock
        const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01; // ~89 degrees
        self.camera.pitch = self.camera.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

        self.camera.update_vectors();
    }

    /// Calculates the velocity vector of the players input state
    // TODO: jumping?
    fn get_input_vel_xz(&self, input_state: &mut InputState) -> Vec3 {
        let input_vel = input_state.as_vel();
        let input_vel_transformed = (self.camera.front * input_vel.x) + (self.camera.right * input_vel.z);
        input_vel_transformed * self.move_speed
    }

    /// Converts a body position (feet) to head/camera position (eye level).
    fn get_eye_pos_from_body_pos(&self, body_pos: Vec3) -> Vec3 {
        // Eye level is roughly 90% of body height from the feet
        body_pos + Vec3::Y * (self.body.size.y * 0.9)
    }

    /// Returns the interpolated camera position for smooth rendering.
    fn get_updated_camera_pos(&self, last_player_state: Option<&PlayerState>) -> Vec3 {
        let body_pos = if self.body.accumulator >= 0.
            && let Some(last) = last_player_state
        {
            let last = last.body.position;
            let curr = self.body.position;
            last + (curr - last) * self.body.accumulator / PHYSICS_DT
        } else {
            self.body.position
        };

        self.get_eye_pos_from_body_pos(body_pos)
    }
}

/// Get the initial velocity of a jump that will reach height `h`
fn get_initial_jump_vel(h: f32) -> f32 {
    assert!(h >= 0., "Jump height must be >= 0");
    (2. * h * GRAVITY.abs()).sqrt()
}
