use amethyst::{
    core::transform::Transform,
    ecs::{
        prelude::*,
        Read,
        ReadStorage,
        System,
        WriteStorage,
    },
};

use crate::{
    components::Ball,
    config::{
        ArenaConfig,
        PaddleConfig,
    },
    resources::Players,
};

#[derive(Default)]
pub struct MovePaddleSystem;

impl<'s> System<'s> for MovePaddleSystem {
    type SystemData = (
        Read<'s, ArenaConfig>,
        Read<'s, PaddleConfig>,
        ReadExpect<'s, Players>,
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (
        arena_config,
        paddle_config,
        players,
        balls,
        mut transforms
    ): Self::SystemData) {

        let ball_y = {
            (&balls, &mut transforms).join()
                .nth(0)
                .map(|(_, transform)| transform.translation().y)
                .expect("No ball found")
        };

        let mut paddle_transform = transforms
            .get_mut(players.p2)
            .expect("No player 2 found");

        let paddle_y = paddle_transform.translation().y;
        let movement = if ball_y > paddle_y {
            1.0
        } else if ball_y < paddle_y {
            -1.0
        } else {
            0.0
        };

        let new_y = calculate_y(
            arena_config.height,
            paddle_config.height,
            movement,
            paddle_y,
        );
        paddle_transform.set_translation_y(new_y);
    }
}

/// Calculates the new y position of the paddle based on the movement value and the current y.
fn calculate_y(arena_height: f32, paddle_height: f32, movement: f32, current_y: f32) -> f32 {
    let scaled_movement = 1.2 * movement as f32;
    (current_y + scaled_movement)
        .min(arena_height - paddle_height * 0.5)
        .max(paddle_height * 0.5)
}