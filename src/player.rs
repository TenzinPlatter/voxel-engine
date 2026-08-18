use glam::Vec3;
use uom::si::{acceleration::meter_per_second_squared, f32::Time, time};

use crate::{
    engine::world::World,
    input::InputState,
    physics::{GRAVITY, PHYSICS_DT, PhysicsBody, hit_info::HitFace},
    render::camera::Camera,
};

pub(crate) const DEFAULT_MOUSE_SENS: f32 = 0.1;
pub(crate) const DEFAULT_PLAYER_SPEED: f32 = 6.0;
pub(crate) const DEFAULT_PLAYER_JUMP_HEIGHT: f32 = 1.25;
pub(crate) const DEFAULT_PLAYER_REACH: f32 = 5.0;
const MAX_COLLISION_ITERATIONS: u32 = 3;

#[derive(Debug)]
pub struct PlayerState {
    body: PhysicsBody,
}

pub struct Player {
    pub body: PhysicsBody,
    grounded: bool,

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
            grounded: false,
        }
    }

    /// Updates the player for one frame, handling input, physics, and camera interpolation.
    pub fn step(
        &mut self,
        world: &World,
        frame_delta: Time,
        input_state: &mut InputState,
        last_player_state: Option<&PlayerState>,
    ) -> PhysicsBody {
        // TODO: new method
        //
        // 1. Get proposed movement vec (velocity * dt)
        // 2. Create physics body sweeping across entire movement path
        // 3. Check for a collision, return a value 0-1 representing how far along the path we can go before colliding
        // 4. Move player to that position, leaving a bit of space from the collision surface
        // 5. we have (1 - t) of movement vec left, project that onto collision surface plane
        // 6. Repeat from step 2 with new movement vec until movement vec is zero or max number of iterations reached

        if input_state.back.is_pressed {
            println!("Moving backwards");
        }

        let input_vel = self.get_input_vel_xz(input_state);
        self.body.velocity = input_vel.with_y(self.body.velocity.y);

        self.body.accumulator += frame_delta;

        while self.body.accumulator > *PHYSICS_DT {
            self.body.accumulator -= *PHYSICS_DT;
            self.body.velocity.y = if !self.grounded {
                self.body.velocity.y + GRAVITY.get::<meter_per_second_squared>() * PHYSICS_DT.get::<time::second>()
            } else {
                0.0
            };

            let mut move_vec = self.body.velocity * frame_delta.get::<time::second>();
            let mut iterations = 0;

            while iterations < MAX_COLLISION_ITERATIONS {
                let sweep_body = self.body.extrude(move_vec);
                println!("body pos: {}", self.body.position);
                println!("move vec {}", move_vec);

                if let Some(hit) = world.is_colliding(&sweep_body) {
                    self.grounded = hit.face == HitFace::NegY;
                    let hit_face_pos = hit.pos.as_vec3()
                        + match hit.face {
                            HitFace::PosX | HitFace::PosZ | HitFace::PosY => hit.normal.as_vec3(),
                            // we don't need to add anything for negative faces since pos is already at that face
                            _ => Vec3::ZERO,
                        };

                    let remaining_vec = (hit_face_pos - self.body.position) - move_vec;
                    self.body.position = hit_face_pos + hit.normal.as_vec3() * 0.01;

                    println!("hit face : {:?}", hit.face);
                    println!("hit face pos: {}", hit_face_pos);
                    println!("remaining: {}", remaining_vec);
                    move_vec = remaining_vec - remaining_vec.project_onto(hit.normal.as_vec3());
                } else {
                    self.body.position += move_vec;
                    break;
                }

                iterations += 1;
            }
        }

        self.camera.position = self.get_updated_camera_pos(last_player_state);
        self.camera.update_vectors();
        // println!("Camera pos: {:?}", self.camera.position);
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
        let input_vel_transformed =
            (self.camera.front * input_vel.x) + (self.camera.right * input_vel.z);
        input_vel_transformed * self.move_speed
    }

    /// Converts a body position (feet) to head/camera position (eye level).
    fn get_eye_pos_from_body_pos(&self, body_pos: Vec3) -> Vec3 {
        // Eye level is roughly 90% of body height from the feet
        body_pos + Vec3::Y * (self.body.size.y * 0.9)
    }

    /// Returns the interpolated camera position for smooth rendering.
    fn get_updated_camera_pos(&self, last_player_state: Option<&PlayerState>) -> Vec3 {
        let body_pos = if self.body.accumulator.get::<time::second>() >= 0.
            && let Some(last) = last_player_state
        {
            let last = last.body.position;
            let curr = self.body.position;
            last + ((curr - last) * (self.body.accumulator.get::<time::second>())
                / PHYSICS_DT.get::<time::second>())
        } else {
            self.body.position
        };

        self.get_eye_pos_from_body_pos(body_pos)
    }
}

/// Get the initial velocity of a jump that will reach height `h`
fn get_initial_jump_vel(h: f32) -> f32 {
    assert!(h >= 0., "Jump height must be >= 0");
    (2. * h * GRAVITY.get::<meter_per_second_squared>()).sqrt()
}
