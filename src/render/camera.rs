use glam::{Mat4, Vec3};

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
    pub mouse_sensitivity: f32,
}

impl Camera {
    /// Creates a new camera at the given position looking in the -Z direction
    pub fn new(position: Vec3) -> Self {
        let mut camera = Self {
            position,
            yaw: -std::f32::consts::FRAC_PI_2, // -90 degrees (looking down -Z)
            pitch: 0.0,
            front: Vec3::new(0.0, 0.0, -1.0),
            right: Vec3::X,
            up: Vec3::Y,
            movement_speed: 100.0,
            mouse_sensitivity: 0.1,
        };
        camera.update_vectors();
        camera
    }

    /// Creates a camera looking at a specific point
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
            movement_speed: 100.0,
            mouse_sensitivity: 0.1,
        };
        camera.update_vectors();
        camera
    }

    /// Returns the view matrix for this camera
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_to_rh(self.position, self.front, self.up)
    }

    /// Move camera forward (in the direction it's looking, ignoring Y)
    pub fn move_forward(&mut self, delta_time: f32) {
        let forward = Vec3::new(self.front.x, 0.0, self.front.z).normalize();
        self.position += forward * self.movement_speed * delta_time;
    }

    /// Move camera backward
    pub fn move_backward(&mut self, delta_time: f32) {
        let forward = Vec3::new(self.front.x, 0.0, self.front.z).normalize();
        self.position -= forward * self.movement_speed * delta_time;
    }

    /// Move camera left (strafe)
    pub fn move_left(&mut self, delta_time: f32) {
        self.position -= self.right * self.movement_speed * delta_time;
    }

    /// Move camera right (strafe)
    pub fn move_right(&mut self, delta_time: f32) {
        self.position += self.right * self.movement_speed * delta_time;
    }

    /// Move camera up
    pub fn move_up(&mut self, delta_time: f32) {
        self.position += WORLD_UP * self.movement_speed * delta_time;
    }

    /// Move camera down
    pub fn move_down(&mut self, delta_time: f32) {
        self.position -= WORLD_UP * self.movement_speed * delta_time;
    }

    /// Process mouse movement to rotate the camera
    pub fn process_mouse(&mut self, x_offset: f32, y_offset: f32) {
        let x_offset = x_offset * self.mouse_sensitivity * 0.01;
        let y_offset = y_offset * self.mouse_sensitivity * 0.01;

        self.yaw += x_offset;
        self.pitch += y_offset;

        // Constrain pitch to prevent gimbal lock
        const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01; // ~89 degrees
        self.pitch = self.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

        self.update_vectors();
    }

    /// Recompute front, right, and up vectors from yaw and pitch
    fn update_vectors(&mut self) {
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
