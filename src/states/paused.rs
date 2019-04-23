use amethyst::{
    ecs::prelude::Entity,
    input::is_key_down,
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{
        UiCreator,
        UiFinder,
    },
};

use crate::resources::CurrentState;

/// The PauseState stops (pauses) the GameState and displays a pause message to the player. This
/// state is positioned on top of the GameState and is exited by pressing the escape key on the
/// keyboard.
pub struct PausedState;

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // set CurrentState to CurrentState::Pause to pause game systems
        *world.write_resource::<CurrentState>() = CurrentState::Pause;

        // create ui elements
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/paused.ron", ());
        })
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // TODO: this feels wrong and inefficient?! there has to be a better way than this...
        // delete ui elements
        let mut paused: Option<Entity> = None;
        world.exec(|finder: UiFinder| {
            paused = finder.find("paused");
        });

        if let Some(entity) = paused {
            world.delete_entity(entity);
        }
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