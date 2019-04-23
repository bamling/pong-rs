use amethyst::{
    core::bundle::SystemBundle,
    ecs::DispatcherBuilder,
    error::Error
};

use super::{
    bounce::BounceSystem,
    move_balls::MoveBallsSystem,
    move_paddles::MovePaddlesSystem,
    player_input::PlayerInputSystem,
    winner::WinnerSystem
};

/// Bundle containing all systems relevant to this game.
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<(), Error> {
        // input system
        dispatcher.add(PlayerInputSystem::default(), "player_input_system", &[
            "input_system"
        ]);

        // movement systems
        dispatcher.add(MoveBallsSystem::default(), "move_balls_system", &[]);
        dispatcher.add(MovePaddlesSystem::default(), "move_paddles_system", &[
            "player_input_system"
        ]);

        // collision systems
        dispatcher.add(BounceSystem::default(), "bounce_system", &[
            "move_balls_system",
            "move_paddles_system"
        ]);

        // etc
        dispatcher.add(WinnerSystem::default(), "winner_system", &[
            "move_balls_system",
        ]);

        Ok(())
    }
}