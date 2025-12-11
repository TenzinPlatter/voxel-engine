use glam::Vec3;

use crate::{
    GameState,
    input::InputState,
    physics::{PHYSICS_DT, PhysicsBody},
    render::camera::Camera,
};

const DEFAULT_MOUSE_SENS: f32 = 0.2;
const DEFAULT_PLAYER_SPEED: f32 = 20.0;

pub struct PlayerState {
    body: Option<Player>,
}

pub struct Player {
    body: PhysicsBody,

    mouse_sensitivity: f32,
    /// multiplier for a normalized velocity vector, player speed
    move_speed: f32,

    pub camera: Camera,
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

    pub fn step(&mut self, frame_delta: f32, input_state: &InputState, game_state: &mut GameState) {
        let input_vel = input_state.as_vel();
        let vel = ((self.camera.front * input_vel.x) + (self.camera.right * input_vel.z) + (self.camera.up * input_vel.y))
            * self.move_speed;

        self.body.accumulator += frame_delta;
        while self.body.accumulator > PHYSICS_DT {
            self.body.position += vel * PHYSICS_DT;
            self.body.accumulator -= PHYSICS_DT;
        }

        // TODO: interpolate for smoother graphics
        // if acc >= 0. {
        //     self.lerp()
        // }

        self.camera.position = self.body.position;
        self.camera.update_vectors();
    }

    pub fn lerp(&mut self, last_state: &PlayerState, current_state: &PlayerState) {}

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
