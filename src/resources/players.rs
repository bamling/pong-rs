use amethyst::ecs::prelude::Entity;

/// Enumeration of valid players.
#[derive(Debug)]
pub enum Player {
    P1,
    P2,
}

/// Keep track of which players are active and can move paddles.
pub struct PlayersActive {
    pub p1: bool,
    pub p2: bool,
}

/// Holds the entities referring to a player. A player entity generally consists of a paddle
/// and a transformation component.
pub struct Players {
    pub p1: Entity,
    pub p2: Entity,
}