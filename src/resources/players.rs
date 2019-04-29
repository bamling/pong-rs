use amethyst::ecs::prelude::Entity;

/// Enumeration of valid players.
#[derive(Debug)]
pub enum Player {
    P1,
    P2,
}

/// Holds the entities referring to a player. A player entity generally consists of a paddle
/// and a transformation component.
pub struct Players {
    pub p1: Entity,
    pub p2: Entity,
}