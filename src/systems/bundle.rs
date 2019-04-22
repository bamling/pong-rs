use amethyst::{
    core::bundle::SystemBundle,
    ecs::DispatcherBuilder,
    error::Error
};

use super::{
    bounce::Bounce,
    move_balls::MoveBalls,
    move_paddles::MovePaddles,
    player_input::PlayerInput,
    winner::Winner
};

/// Bundle containing all systems relevant to this game.
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<(), Error> {
        // input system
        dispatcher.add(PlayerInput::default(), "player_input_system", &[
            "input_system"
        ]);

        // movement systems
        dispatcher.add(MoveBalls::default(), "move_balls_system", &[]);
        dispatcher.add(MovePaddles::default(), "move_paddles_system", &[
            "player_input_system"
        ]);

        // collision systems
        dispatcher.add(Bounce::default(), "bounce_system", &[
            "move_balls_system",
            "move_paddles_system"
        ]);

        // etc
        dispatcher.add(Winner::default(), "winner_system", &[
            "move_balls_system",
        ]);

        Ok(())
    }
}