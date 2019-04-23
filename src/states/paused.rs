use amethyst::{
    input::is_key_down,
    prelude::*,
    renderer::VirtualKeyCode,
};

use crate::resources::CurrentState;

/// The PauseState stops (pauses) the GameState and displays a pause message to the player. This
/// state is positioned on top of the GameState and is exited by pressing the escape key on the
/// keyboard.
pub struct PausedState;

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<GameData>) {
        // set CurrentState to CurrentState::Pause to pause game systems
        *data.world.write_resource::<CurrentState>() = CurrentState::Pause;
    }

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