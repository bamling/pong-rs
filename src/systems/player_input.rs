use amethyst::{
    ecs::{Read, ReadExpect, System, Write},
    input::InputHandler,
};

use crate::resources::{
    commands::{Command, CommandChannel},
    players::{Player, PlayersActive},
};

/// PlayerInput system encapsulates player input handling and converts receiver input into
/// Commands. These Commands are then published to other systems via the CommandChannel.
#[derive(Default)]
pub struct PlayerInput;

impl<'s> System<'s> for PlayerInput {
    type SystemData = (
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, PlayersActive>,
        Write<'s, CommandChannel>
    );

    fn run(&mut self, (input, players_active, mut commands): Self::SystemData) {
        // handle axis based input
        for axis in input.bindings.axes() {
            let axis: &String = axis;
            let axis_value = input.axis_value(axis).unwrap_or(0.0) as f32;
            //println!("{} = {}", axis, axis_value);

            if axis_value == 0.0 {
                continue;
            }

            // TODO:
            // is this a good way of handling inputs for (potential) multiple players or would it
            // be better to publish all inputs and ignore them in other systems if needed?
            match axis.as_ref() {
                // handle left_paddle if player 1 is active
                "left_paddle" => if players_active.p1 {
                    commands.single_write(Command::MovePaddle(Player::P1, axis_value));
                },
                // handle right paddle if player 2 is active
                "right_paddle" => if players_active.p2{
                    commands.single_write(Command::MovePaddle(Player::P2, axis_value));
                },
                _ => println!("unhandled axis input {} with value {}", axis, axis_value)
            }
        }
    }
}
