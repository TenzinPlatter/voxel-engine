use glam::Vec3;

use crate::{
    engine::world::World,
    input::InputState,
    physics::{PHYSICS_DT, PhysicsBody},
    render::camera::Camera,
};

const DEFAULT_MOUSE_SENS: f32 = 0.2;
const DEFAULT_PLAYER_SPEED: f32 = 20.0;

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
            body: PhysicsBody::new(position, Vec3::new(1., 2., 1.)),
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
        input_state: &InputState,
        last_player_state: Option<&PlayerState>,
    ) -> PhysicsBody {
        self.body.velocity = self.get_velocity_vec(input_state, frame_delta, world.is_colliding(&self.body));

        self.body.accumulator += frame_delta;
        while self.body.accumulator > PHYSICS_DT {
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

    /// Calculates the player's velocity vector from input and physics.
    fn get_velocity_vec(&mut self, input_state: &InputState, frame_delta: f32, is_colliding: bool) -> Vec3 {
        let input_vel = input_state.as_vel();
        let input_vel_transformed = (self.camera.front * input_vel.x) + (self.camera.right * input_vel.z);

        self.body.gravity_accumulator = if input_state.up.just_pressed || is_colliding {
            0.
        } else {
            self.body.gravity_accumulator + frame_delta
        };

        let gravity = Vec3::ZERO.with_y(-9.8 * (self.body.gravity_accumulator));

        (input_vel_transformed.with_y(input_vel.y) * self.move_speed) + gravity
    }

    /// Returns the interpolated camera position for smooth rendering.
    fn get_updated_camera_pos(&self, last_player_state: Option<&PlayerState>) -> Vec3 {
        if self.body.accumulator >= 0.
            && let Some(last) = last_player_state
        {
            let last = last.body.position;
            let curr = self.body.position;
            last + (curr - last) * self.body.accumulator / PHYSICS_DT
        } else {
            self.body.position
        }
    }
}
