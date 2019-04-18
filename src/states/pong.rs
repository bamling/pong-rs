use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
               SpriteSheetHandle, Texture, TextureMetadata},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::{
    components::{Ball, Paddle, Side},
    constants::{
        ARENA_HEIGHT,
        ARENA_WIDTH,
        BALL_RADIUS,
        BALL_VELOCITY_X,
        BALL_VELOCITY_Y,
        PADDLE_WIDTH,
    },
    resources::ScoreText,
};

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // load the sprite sheet necessary to render the graphics
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_paddles(world, sprite_sheet_handle.clone());
        initialise_ball(world, sprite_sheet_handle);
        initialise_scoreboard(world);
        initialise_camera(world);
    }
}

/// Initialise the camera
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

/// Initialise the paddles
fn initialise_paddles(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // correctly position the paddles
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    // create a left plank entity
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    // create a right plank entity
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

/// Initialise the ball
fn initialise_ball(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_HEIGHT / 2.0, ARENA_WIDTH / 2.0, 0.0);

    // assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1, // ball is the second sprite in the sprite_sheet
    };

    // create the ball entity
    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}

/// Initialise a ui scoreboard
fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle,
        -50.0, -50.0, 1.0, 200.0, 50.0,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle,
        50.0, -50.0, 1.0, 200.0, 50.0,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        )).build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        )).build();

    world.add_resource(ScoreText { p1_score, p2_score });
}

/// Load the sprite sheet
fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}