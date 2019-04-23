use amethyst::ecs::prelude::{
    Component,
    DenseVecStorage
};

/// The Ball component contains data that defines a ball on the field, such as the radius
/// and the velocity.
pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}