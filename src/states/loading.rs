use amethyst::{
    assets::{
        AssetStorage,
        Completion,
        Handle,
        Loader,
        Prefab,
        ProgressCounter,
        RonFormat,
    },
    prelude::*,
    ui::{
        UiCreator,
        UiFormat,
        UiPrefab,
        NoCustomUi
    },
};

use super::game::{GamePrefabData, GameState};

/// The `LoadingState` loads all required `Assets` and ensures everything is ready before
/// transitioning into the `GameState`.
#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
    game_prefab_handle: Option<Handle<Prefab<GamePrefabData>>>,
    game_ui_handle: Option<Handle<UiPrefab>>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("LoadingState.on_start");
        let world = data.world;

        add_loading_ui(world, &mut self.progress_counter);
        self.load_game_prefab(world);
        self.load_game_ui(world);
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress_counter.complete() {
            Completion::Loading => {
                Trans::None
            }
            Completion::Complete => {
                let game_prefab = self.game_prefab_handle
                    .take()
                    .expect("LoadingState.game_prefab_handle was None after loading assets");
                let game_ui = self.game_ui_handle
                    .take()
                    .expect("LoadingState.game_ui_handle was None after loading assets");

                return Trans::Switch(
                    Box::new(
                        GameState::new(game_prefab, game_ui)
                    )
                );
            }
            Completion::Failed => {
                error!("Failed to load assets, exiting");
                Trans::Quit
            }
        }
    }
}

impl LoadingState {
    fn load_game_prefab(&mut self, world: &mut World) {
        let loader = world.read_resource::<Loader>();
        let prefab_storage = world.read_resource::<AssetStorage<Prefab<GamePrefabData>>>();
        let prefab_handle = loader.load(
            "prefab/game.ron",
            RonFormat,
            Default::default(),
            &mut self.progress_counter,
            &prefab_storage,
        );

        self.game_prefab_handle = Some(prefab_handle);
    }

    fn load_game_ui(&mut self, world: &mut World) {
        let loader = world.read_resource::<Loader>();
        let ui_storage = world.read_resource::<AssetStorage<UiPrefab>>();
        let ui_handle = loader.load(
            "ui/game.ron",
            UiFormat::<NoCustomUi>::default(),
            Default::default(),
            &mut self.progress_counter,
            &ui_storage,
        );

        self.game_ui_handle = Some(ui_handle);
    }
}

fn add_loading_ui(world: &mut World, progress: &mut ProgressCounter) {
    world.exec(|mut creator: UiCreator| {
        creator.create("ui/loading.ron", progress);
    });
}



