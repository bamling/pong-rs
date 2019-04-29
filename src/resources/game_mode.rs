#[derive(PartialEq)]
pub enum GameMode {
    SinglePlayer,
    MultiPlayer,
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::MultiPlayer
    }
}