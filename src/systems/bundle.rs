use amethyst::{
    core::bundle::SystemBundle,
    ecs::DispatcherBuilder,
    error::Error,
    prelude::*,
};

use crate::resources::CurrentState;

use super::{
    bounce::BounceSystem,
    move_balls::MoveBallsSystem,
    move_paddles::MovePaddlesSystem,
    player_input::PlayerInputSystem,
    winner::WinnerSystem,
};

/// Bundle containing all systems relevant to this game.
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<(), Error> {
        // input system
        dispatcher.add(
            PlayerInputSystem::default().pausable(CurrentState::Game),
            "player_input_system",
            &["input_system"],
        );

        // movement systems
        dispatcher.add(
            MoveBallsSystem::default().pausable(CurrentState::Game),
            "move_balls_system",
            &[],
        );
        dispatcher.add(
            MovePaddlesSystem::default(), // doesn't have to be pausable due to the EventChannel logic
            "move_paddles_system",
            &["player_input_system"],
        );

        // collision systems
        dispatcher.add(
            BounceSystem::default().pausable(CurrentState::Game),
            "bounce_system",
            &["move_balls_system", "move_paddles_system"],
        );

        // etc
        dispatcher.add(
            WinnerSystem::default().pausable(CurrentState::Game),
            "winner_system",
            &["move_balls_system"],
        );

        Ok(())
    }
}