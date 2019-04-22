use amethyst::ecs::prelude::Entity;

/// ScoreBoard contains the actual score data.
#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

/// ScoreTest contains the ui text components that display the score.
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}