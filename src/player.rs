use std::env::current_dir;

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
    pub fn new(body: PhysicsBody) -> Self {
        Self { body }
    }
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        Self {
            body: PhysicsBody::new(position, Vec3::ZERO),
            camera: Camera::looking_at(position, Vec3::ZERO),
            mouse_sensitivity: DEFAULT_MOUSE_SENS,
            move_speed: DEFAULT_PLAYER_SPEED,
        }
    }

    /// step a players (self) physics
    /// @param last_player_state
    pub fn step(
        &mut self,
        world: &World,
        frame_delta: f32,
        input_state: &InputState,
        last_player_state: Option<&PlayerState>,
    ) -> PhysicsBody {
        let is_colliding = world.is_colliding(&self.body);

        let input_vel = input_state.as_vel();
        let input_vel_transformed =
            (self.camera.front * input_vel.x) + (self.camera.right * input_vel.z) + (self.camera.up * input_vel.y);

        self.body.gravity_accumulator = if input_state.up.just_pressed || is_colliding {
            0.
        } else {
            self.body.gravity_accumulator + frame_delta
        };
        let gravity = Vec3::ZERO.with_y(-9.8 * (self.body.gravity_accumulator));

        self.body.velocity = (input_vel_transformed * self.move_speed) + gravity;

        self.body.accumulator += frame_delta;
        while self.body.accumulator > PHYSICS_DT {
            self.body.position += self.body.velocity * PHYSICS_DT;
            self.body.accumulator -= PHYSICS_DT;
        }

        self.camera.position = if self.body.accumulator >= 0.
            && let Some(last) = last_player_state
        {
            let last = last.body.position;
            let curr = self.body.position;
            last + (curr - last) * self.body.accumulator / PHYSICS_DT
        } else {
            self.body.position
        };

        self.camera.update_vectors();
        self.body.clone()
    }

    /// Process mouse movement to rotate the camera
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
}
