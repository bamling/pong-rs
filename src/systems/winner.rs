use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadExpect, System, Write, WriteStorage},
    ui::UiText,
};

use crate::{
    components::Ball,
    resources::{ScoreBoard, ScoreText},
    states::pong::ARENA_WIDTH,
};

#[derive(Default)]
pub struct Winner;

impl<'s> System<'s> for Winner {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (mut balls, mut transforms, mut ui_text, mut score_board, score_text): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let ball: &mut Ball = ball;
            let transform: &mut Transform = transform;

            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Right player scored on the left side.
                // We top the score at 999 to avoid text overlap.
                score_board.score_right = (score_board.score_right + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = score_board.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Left player scored on the right side.
                // We top the score at 999 to avoid text overlap.
                score_board.score_left = (score_board.score_left + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = score_board.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0]; // Reverse Direction
                transform.set_translation_x(ARENA_WIDTH / 2.0); // Reset Position

                // Print the score board.
                println!(
                    "Score: | {:^3} | {:^3} |",
                    score_board.score_left, score_board.score_right
                );
            }
        }
    }
}