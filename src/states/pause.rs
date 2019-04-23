use amethyst::{
    input::is_key_down,
    prelude::*,
    renderer::VirtualKeyCode,
};

/// The PauseState stops (pauses) the GameState and displays a pause message to the player. This
/// state is positioned on top of the GameState and is exited by pressing the escape key on the
/// keyboard.
pub struct PauseState;

impl SimpleState for PauseState {
    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Go back to the GameState.
                return Trans::Pop;
            }
        }

        // Escape isn't pressed, so we stay in this state.
        Trans::None
    }
}