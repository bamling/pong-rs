#[macro_use]
extern crate log;

use amethyst::{
    assets::PrefabLoaderSystem,
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        DisplayConfig,
        DrawFlat2D,
        Pipeline,
        RenderBundle,
        Stage,
    },
    ui::{
        DrawUi,
        UiBundle,
    },
    utils::application_root_dir,
};

use config::GameConfig;
use states::{
    game::GamePrefabData,
    LoadingState,
};

mod components;
mod config;
mod resources;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // display configuration
    let display_config_path = app_root.join("resources/display_config.ron");
    let display_config = DisplayConfig::load(&display_config_path);

    // game config
    let config_path = app_root.join("resources/config.ron");
    let config = GameConfig::load(&config_path);

    // key bindings
    let key_bindings_path = app_root.join("resources/input.ron");

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new())
                .with_pass(DrawUi::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<String, String>::new().with_bindings_from_file(key_bindings_path)?)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(systems::CoreSystemsBundle)?
        .with(PrefabLoaderSystem::<GamePrefabData>::default(), "", &[]);

    let assets_dir = app_root.join("assets");

    let mut game = Application::build(assets_dir, LoadingState::default())?
        .with_resource(config.arena)
        .with_resource(config.ball)
        .with_resource(config.paddle)
        .build(game_data)?;

    game.run();

    Ok(())
}