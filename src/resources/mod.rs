pub use self::{
    commands::{
        Command,
        CommandChannel
    },
    players::{
        Player,
        Players,
        PlayersActive
    },
    score_board::{
        ScoreBoard,
        ScoreText
    },
};

pub mod commands;
pub mod players;
pub mod score_board;

/// CurrentState enum resource for keeping track of the current active state.
#[derive(PartialEq)]
pub enum CurrentState {
    Game,
    Pause,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Pause
    }
}