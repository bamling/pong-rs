use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// Constants.
pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

/// The Ball component contains data that defines a ball on the field, such as the radius
/// and the velocity.
pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}