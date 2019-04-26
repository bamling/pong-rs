use amethyst::{
    assets::{
        AssetStorage,
        Completion,
        Handle,
        Loader,
        Prefab,
        PrefabLoader,
        ProgressCounter,
        RonFormat,
    },
    ecs::prelude::Entity,
    input::{
        is_close_requested,
        is_key_down,
    },
    prelude::*,
    renderer::{
        PngFormat,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
        VirtualKeyCode,
    },
    ui::{
        UiCreator,
        UiLoader,
        UiPrefab,
    },
};

use super::game::{
    GamePrefabData,
    GameState,
};

/// The `LoadingState` loads all required `Assets` and ensures everything is ready before
/// transitioning into the `GameState`.
#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,

    loading_ui: Option<Entity>,
    scene_handle: Option<Handle<Prefab<GamePrefabData>>>,

    game_ui_handle: Option<Handle<UiPrefab>>,
    paused_ui_handle: Option<Handle<UiPrefab>>,

    sprite_sheet_handle: Option<SpriteSheetHandle>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("LoadingState.on_start");
        let world = data.world;

        // load this states ui
        self.loading_ui = Some(world.exec(|mut creator: UiCreator| {
            creator.create("ui/loading.ron", &mut self.progress)
        }));

        // load scene handle
        self.scene_handle = Some(world.exec(|loader: PrefabLoader<GamePrefabData>| {
            loader.load("prefab/scene.ron", RonFormat, (), &mut self.progress)
        }));

        // load other state specific ui handles
        self.game_ui_handle = Some(world.exec(|loader: UiLoader| {
            loader.load("ui/game.ron", &mut self.progress)
        }));
        self.paused_ui_handle = Some(world.exec(|loader: UiLoader| {
            loader.load("ui/paused.ron", &mut self.progress)
        }));

        // load sprite sheet handle
        self.sprite_sheet_handle = Some(self.load_sprite_sheet(world));
    }

    fn on_stop(&mut self, _data: StateData<GameData>) {
        info!("LoadingState.on_stop");
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        // handle window events and quit the current State if the Escape button is pressed
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        // event was not of type StateEvent, so no transition is required
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress.complete() {
            // still loading, no state transition required
            Completion::Loading => {
                info!("[{}/{}] Loading Assets...",
                      self.progress.num_finished(),
                      self.progress.num_assets()
                );
                Trans::None
            }
            // loading completed, transition to GameState and remove ui
            Completion::Complete => {
                info!("[{}/{}] Assets loaded, swapping to GameState",
                      self.progress.num_finished(),
                      self.progress.num_assets()
                );
                // remove the loading ui from the screen
                if let Some(entity) = self.loading_ui {
                    let _ = data.world.delete_entity(entity);
                }

                // remove LoadingState from the stack and switch to GameState
                Trans::Switch(Box::new(GameState::new(
                    self.scene_handle.take().unwrap(),
                    self.game_ui_handle.take().unwrap(),
                    self.paused_ui_handle.take().unwrap(),
                    self.sprite_sheet_handle.take().unwrap(),
                )))
            }
            // loading failed, quit LoadingState and the game
            Completion::Failed => {
                error!("Failed to load Assets");
                error!("{:?}", self.progress.errors());
                Trans::Quit
            }
        }
    }
}

impl LoadingState {
    /// Load the sprite sheet
    fn load_sprite_sheet(&mut self, world: &mut World) -> SpriteSheetHandle {
        // Load the sprite sheet necessary to render the graphics.
        // The texture is the pixel data
        // `sprite_sheet` is the layout of the sprites on the image
        // `texture_handle` is a cloneable reference to the texture
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "texture/pong_spritesheet.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                &mut self.progress,
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "texture/pong_spritesheet.ron", // Here we load the associated ron file
            SpriteSheetFormat,
            texture_handle, // We pass it the texture we want it to use
            &mut self.progress,
            &sprite_sheet_store,
        )
    }
}