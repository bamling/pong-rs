use amethyst::{
    assets::Handle,
    ecs::prelude::Entity,
    input::{
        is_close_requested,
        is_key_down,
    },
    prelude::*,
    renderer::VirtualKeyCode,
    ui::UiPrefab,
};

/// The `PauseState` stops (pauses) the `GameState` and displays a pause message to the player. This
/// state is positioned on top of the `GameState` and is exited by pressing the escape key on the
/// keyboard.
pub struct PausedState {
    paused_ui: Option<Entity>,
    paused_ui_handle: Handle<UiPrefab>,
}

impl PausedState {
    pub fn new(paused_ui_handle: Handle<UiPrefab>) -> Self {
        Self {
            paused_ui: None,
            paused_ui_handle,
        }
    }
}

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("PausedState.on_start");
        // create the paused ui
        self.paused_ui = Some(data.world
            .create_entity()
            .with(self.paused_ui_handle.clone())
            .build()
        );
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        info!("PausedState.on_stop");
        // delete paused ui
        if let Some(entity) = self.paused_ui {
            let _ = data.world.delete_entity(entity);
        }
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}