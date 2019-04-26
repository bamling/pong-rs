use amethyst::{
    core::Transform,
    ecs::{
        prelude::*,
        Read,
        ReadExpect,
        Resources,
        System,
        WriteStorage,
    },
    shrev::ReaderId,
};

use crate::{
    resources::{
        Command,
        CommandChannel,
        Player,
        Players
    },
    config::{
        ArenaConfig,
        PaddleConfig,
    }
};

/// The MovePaddleSystem handles the moving of paddles on the X axis, depending on received
/// commands via CommandChannel.
#[derive(Default)]
pub struct MovePaddlesSystem {
    command_reader: Option<ReaderId<Command>>
}

impl<'s> System<'s> for MovePaddlesSystem {
    type SystemData = (
        Read<'s, CommandChannel>,
        Read<'s, ArenaConfig>,
        Read<'s, PaddleConfig>,
        ReadExpect<'s, Players>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (
        commands,
        arena_config,
        paddle_config,
        players,
        mut transforms
    ): Self::SystemData) {

        for command in commands.read(self.command_reader.as_mut().unwrap()) {
            match command {
                // handle movement commands for player 1
                Command::MovePaddle(Player::P1, movement) => {
                    if let Some(transform) = transforms.get_mut(players.p1) {
                        let new_y = calculate_y(
                            arena_config.height,
                            paddle_config.height,
                            *movement,
                            transform.translation().y
                        );
                        transform.set_translation_y(new_y);
                    }
                },
                // handle movement commands for player 2
                Command::MovePaddle(Player::P2, movement) => {
                    if let Some(transform) = transforms.get_mut(players.p2) {
                        let new_y = calculate_y(
                            arena_config.height,
                            paddle_config.height,
                            *movement,
                            transform.translation().y
                        );
                        transform.set_translation_y(new_y);
                    }
                },
                _ => {}
            }
        }
    }

    /// Register reader for the CommandChannel.
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
    }
}

/// Calculates the new y position of the paddle based on the movement value and the current y.
fn calculate_y(arena_height: f32, paddle_height: f32, movement: f32, current_y: f32) -> f32 {
    let scaled_movement = 1.2 * movement as f32;
    (current_y + scaled_movement)
        .min(arena_height - paddle_height * 0.5)
        .max(paddle_height * 0.5)
}
