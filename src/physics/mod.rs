use glam::Vec3;

pub trait PhysicsBody {
    /// return the current position of the body
    fn position(&self) -> Vec3;

    /// size of the body's (assumed to be a rectangular prism) sides across each axis (x, y, z)
    fn size(&self) -> Vec3;

    /// move the body by the delta provided
    fn translate(&mut self, delta: Vec3);
}

/// Check whether two physics bodies are colliding using AABB collision detection
pub fn colliding_with<A: PhysicsBody, B: PhysicsBody>(a: &A, b: &B) -> bool {
    let a_pos = a.position();
    let a_size = a.size();
    let b_pos = b.position();
    let b_size = b.size();

    // AABB collision detection (symmetric check)
    a_pos.x < b_pos.x + b_size.x
        && a_pos.x + a_size.x > b_pos.x
        && a_pos.y < b_pos.y + b_size.y
        && a_pos.y + a_size.y > b_pos.y
        && a_pos.z < b_pos.z + b_size.z
        && a_pos.z + a_size.z > b_pos.z
}
