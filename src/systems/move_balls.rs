use amethyst::{
    core::{
        timing::Time,
        transform::Transform
    },
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::ball::Ball;

pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (balls, mut transforms, time): Self::SystemData) {
        // move every ball according to its speed, and the time passed
        for (ball, transform) in (&balls, &mut transforms).join() {
            let ball: &Ball= ball;
            let transform: &mut Transform = transform;

            transform.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(ball.velocity[1] * time.delta_seconds());

        }
    }
}
