use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// Constants.
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

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

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}
