use amethyst::{
    ecs::{
        Read,
        ReadExpect,
        System,
        Write,
    },
    input::InputHandler,
};

use crate::resources::{
    Command,
    CommandChannel,
    GameMode,
    Player,
};

/// PlayerInput system encapsulates player input handling and converts receiver input into
/// Commands. These Commands are then published to other systems via the CommandChannel.
#[derive(Default)]
pub struct PlayerInputSystem;

impl<'s> System<'s> for PlayerInputSystem {
    type SystemData = (
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, GameMode>,
        Write<'s, CommandChannel>
    );

    fn run(&mut self, (input, game_mode, mut commands): Self::SystemData) {
        // always handle left player movement
        let movement = input.axis_value("left_paddle");
        if let Some(movement) = movement {
            commands.single_write(Command::MovePaddle(Player::P1, movement as f32));
        }

        // handle right player if GameMode == MultiPlayer
        if *game_mode == GameMode::MultiPlayer {
            let movement = input.axis_value("right_paddle");
            if let Some(movement) = movement {
                commands.single_write(Command::MovePaddle(Player::P2, movement as f32));
            }
        }
    }
}
