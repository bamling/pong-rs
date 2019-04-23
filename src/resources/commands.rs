use amethyst::shrev::EventChannel;

use super::players::Player;

/// List of commands that are interpreted by systems.
#[derive(Debug)]
pub enum Command {
    MovePaddle(Player, f32),
    //LaunchBall(Player),
    //Pause,
}

/// Custom type alias for EventChannel<Command>. Mostly for convenience.
pub type CommandChannel = EventChannel<Command>;