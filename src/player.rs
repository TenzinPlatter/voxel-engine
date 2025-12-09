use glam::{Mat4, Vec3};

use crate::{
    input::InputState,
    physics::{PHYSICS_DT, PhysicsBody},
    render::camera::Camera,
};

const DEFAULT_MOUSE_SENS: f32 = 0.2;
const DEFAULT_PLAYER_SPEED: f32 = 20.0;

pub struct Player {
    position: Vec3,
    velocity: Vec3,

    mouse_sensitivity: f32,
    /// multiplier for a normalized velocity vector, player speed
    move_speed: f32,

    pub input_state: InputState,
    pub camera: Camera,
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            camera: Camera::looking_at(position, Vec3::ZERO),
            input_state: InputState::default(),
            mouse_sensitivity: DEFAULT_MOUSE_SENS,
            move_speed: DEFAULT_PLAYER_SPEED,
        }
    }

    pub fn step(&mut self, frame_delta: f32) {
        self.velocity = self.camera.view_matrix().transform_vector3(self.input_state.as_vel()) * self.move_speed;

        let mut acc = frame_delta;
        while acc > PHYSICS_DT {
            self.position += self.velocity * PHYSICS_DT;
            acc -= PHYSICS_DT;
        }

        self.camera.position = self.position;
        self.camera.update_vectors();
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

impl PhysicsBody for Player {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn size(&self) -> Vec3 {
        Vec3::new(1., 2., 1.)
    }

    fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }
}
