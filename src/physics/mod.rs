use glam::Vec3;

pub trait PhysicsBody {
    /// return whether a body is colliding with another body
    fn colliding_with(&self, other: &dyn PhysicsBody) -> bool;

    /// return the current position of the body
    fn position(&self) -> Vec3;

    /// size of the body's (assumed to be a rectangular prism) sides across each axis (x, y, z)
    fn size(&self) -> Vec3;

    /// move the body by the delta provided
    fn translate(&mut self, delta: Vec3);
}
