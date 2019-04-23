use amethyst::ecs::prelude::{
    Component,
    DenseVecStorage
};

/// Side enumeration describes on which side of the arena the paddle is located at.
#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// The paddle component contains data that defines a paddle on the field, such as the side
/// of the field and the width and height.
pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
