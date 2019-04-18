use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline, RenderBundle, Stage, VirtualKeyCode},
    utils::application_root_dir,
};

use pong::Pong;

mod pong;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // display configuration
    let display_config_path = app_root.join("resources/display_config.ron");
    let display_config = DisplayConfig::load(&display_config_path);

    // bindings configuration
    let bindings_config_path = app_root.join("resources/bindings_config.ron");

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new())
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<String, String>::new().with_bindings_from_file(bindings_config_path)?)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "move_balls_system", &[])
        .with(systems::BounceSystem, "bounce_system", &["paddle_system", "move_balls_system"]);

    let assets_dir = app_root.join("assets");

    let mut game = Application::build(assets_dir, Pong)?
        .build(game_data)?;

    game.run();

    Ok(())
}