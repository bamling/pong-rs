use amethyst::{
    core::{
        timing::Time,
        transform::Transform
    },
    ecs::prelude::{
        Join,
        Read,
        ReadStorage,
        System,
        WriteStorage
    },
};

use crate::components::ball::Ball;

/// The MoveBalls system handles the moving of the balls inside the arena. The balls move according
/// to the amount of time passed between frames.
#[derive(Default)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (time, balls, mut transforms): Self::SystemData) {
        // move every ball according to its speed, and the time passed
        for (ball, transform) in (&balls, &mut transforms).join() {
            let ball: &Ball= ball;
            let transform: &mut Transform = transform;

            transform.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(ball.velocity[1] * time.delta_seconds());

        }
    }
}
