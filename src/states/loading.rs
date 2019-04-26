use amethyst::{
    assets::{
        Completion,
        Handle,
        Prefab,
        PrefabLoader,
        ProgressCounter,
        RonFormat,
    },
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{
        UiCreator,
        UiLoader,
        UiPrefab,
    },
};

use super::game::{GamePrefabData, GameState};

/// The `LoadingState` loads all required `Assets` and ensures everything is ready before
/// transitioning into the `GameState`.
#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,

    loading_ui: Option<Entity>,
    scene_handle: Option<Handle<Prefab<GamePrefabData>>>,

    game_ui_handle: Option<Handle<UiPrefab>>,
    paused_ui_handle: Option<Handle<UiPrefab>>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("LoadingState.on_start");
        let world = data.world;

        // load this states ui
        self.loading_ui = Some(world.exec(|mut creator: UiCreator| {
            creator.create("ui/loading.ron", &mut self.progress)
        }));

        // load scene
        self.scene_handle = Some(world.exec(|loader: PrefabLoader<GamePrefabData>| {
            loader.load("prefab/game.ron", RonFormat, (), &mut self.progress)
        }));

        // load other ui handles
        self.game_ui_handle = Some(world.exec(|loader: UiLoader| {
            loader.load("ui/game.ron", &mut self.progress)
        }));
        self.paused_ui_handle = Some(world.exec(|loader: UiLoader| {
            loader.load("ui/paused.ron", &mut self.progress)
        }));
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Loading => {
                Trans::None
            }
            Completion::Complete => {
                info!("Assets loaded, swapping to GameState");
                if let Some(entity) = self.loading_ui {
                    let _ = data.world.delete_entity(entity);
                }

                return Trans::Switch(Box::new(GameState::new(
                    self.scene_handle.as_ref().unwrap().clone(),
                    self.game_ui_handle.as_ref().unwrap().clone(),
                    self.paused_ui_handle.as_ref().unwrap().clone(),
                )));
            }
            Completion::Failed => {
                error!("Failed to load assets, exiting");
                Trans::Quit
            }
        }
    }
}