use amethyst::ecs::prelude::Entity;

/// Enumeration of valid players.
#[derive(Debug)]
pub enum Player {
    P1,
    P2,
}

/// Keep track of which players are active and can move paddles.
//#[derive(Default)]
pub struct PlayersActive {
    pub p1: bool,
    pub p2: bool,
}

// TODO: remove
impl Default for PlayersActive {
    fn default() -> Self {
        PlayersActive {
            p1: true,
            p2: true,
        }
    }
}

/// Holds the entities referring to a player. A player entity generally consists of a paddle
/// and a transformation component.
pub struct Players {
    pub p1: Entity,
    pub p2: Entity,
}