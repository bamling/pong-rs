use amethyst::{
    core::transform::Transform,
    ecs::prelude::{
        Join,
        Read,
        ReadStorage,
        System,
        WriteStorage,
    },
};

use crate::{
    components::{
        Ball,
        Paddle,
        Side,
    },
    config::ArenaConfig,
};

/// The Bounce system handles the collision between balls and the paddles and arena. When a
/// collision takes places, the balls velocity is reversed.
#[derive(Default)]
pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        Read<'s, ArenaConfig>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
    );

    fn run(&mut self, (arena_config, paddles, transforms, mut balls): Self::SystemData) {
        // Check whether a ball collided, and bounce off accordingly.
        //
        // We also check for the velocity of the ball every time, to prevent multiple collisions
        // from occurring.
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball: &mut Ball = ball;
            let transform: &Transform = transform;

            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // Bounce at the top or the bottom of the arena.
            if (ball_y <= ball.radius && ball.velocity[1] < 0.0)
                || (ball_y >= arena_config.height - ball.radius && ball.velocity[1] > 0.0) {
                ball.velocity[1] = -ball.velocity[1];
            }

            // Bounce at the paddles.
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle: &Paddle = paddle;
                let paddle_transform: &Transform = paddle_transform;

                let paddle_x = paddle_transform.translation().x - paddle.width * 0.5;
                let paddle_y = paddle_transform.translation().y - paddle.height * 0.5;

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its centre is within the larger wrapper
                // rectangle.
                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                ) {
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0) {
                        ball.velocity[0] = -ball.velocity[0];
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}