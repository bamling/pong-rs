use amethyst::{
    assets::Handle,
    ecs::prelude::Entity,
    input::is_key_down,
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{
        UiLoader,
        UiPrefab,
    },
};

use crate::resources::CurrentState;

/// The PauseState stops (pauses) the GameState and displays a pause message to the player. This
/// state is positioned on top of the GameState and is exited by pressing the escape key on the
/// keyboard.
#[derive(Default)]
pub struct PausedState {
    current_ui: Option<Entity>,
    current_ui_prefab: Option<Handle<UiPrefab>>,
}

impl PausedState {
    fn enable_current_ui(&mut self, world: &mut World) {
        let ui_prefab_handle = self.current_ui_prefab.get_or_insert_with(|| {
            world.exec(|loader: UiLoader| {
                println!("loading paused.ron");
                return loader.load("ui/paused.ron", ());
            })
        });

        self.current_ui = Some(world.create_entity().with(ui_prefab_handle.clone()).build())
    }
}

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // set CurrentState to CurrentState::Pause to pause game systems
        *world.write_resource::<CurrentState>() = CurrentState::Pause;

        // create ui elements
        self.enable_current_ui(world);
    }

    fn on_stop(&mut self, _data: StateData<GameData>) {
        //let world = data.world;

        // delete ui elements
        // TODO:
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