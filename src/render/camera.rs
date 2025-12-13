use glam::{Mat4, Vec3};

use crate::input::InputState;

const WORLD_UP: Vec3 = Vec3::Y;

pub struct Camera {
    // Position
    pub position: Vec3,

    // Orientation (Euler angles in radians)
    pub yaw: f32,   // Rotation around Y axis (left/right)
    pub pitch: f32, // Rotation around X axis (up/down)

    // Camera vectors (computed from yaw/pitch)
    pub front: Vec3,  // Direction camera is looking
    pub right: Vec3,  // Right vector perpendicular to front
    pub up: Vec3,     // Up vector perpendicular to front and right

    // Camera settings
    pub movement_speed: f32,
}

impl Camera {
    /// Creates a new camera at the given position looking in the -Z direction.
    pub fn new(position: Vec3) -> Self {
        let mut camera = Self {
            position,
            yaw: -std::f32::consts::FRAC_PI_2, // -90 degrees (looking down -Z)
            pitch: 0.0,
            front: Vec3::new(0.0, 0.0, -1.0),
            right: Vec3::X,
            up: Vec3::Y,
            movement_speed: 20.0,
        };
        camera.update_vectors();
        camera
    }

    /// Creates a camera looking at a specific point.
    pub fn looking_at(position: Vec3, target: Vec3) -> Self {
        let direction = (target - position).normalize();

        // Convert direction to yaw/pitch
        let yaw = direction.z.atan2(direction.x);
        let pitch = direction.y.asin();

        let mut camera = Self {
            position,
            yaw,
            pitch,
            front: direction,
            right: Vec3::X,
            up: Vec3::Y,
            movement_speed: 20.0,
        };
        camera.update_vectors();
        camera
    }

    /// Returns the view matrix for this camera.
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_to_rh(self.position, self.front, self.up)
    }

    /// Recomputes front, right, and up vectors from yaw and pitch.
    pub fn update_vectors(&mut self) {
        // Calculate new front vector from yaw and pitch
        self.front = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize();

        // Recalculate right and up vectors
        self.right = self.front.cross(WORLD_UP).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}
