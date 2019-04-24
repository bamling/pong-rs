use amethyst::{
    assets::{
        AssetStorage,
        Handle,
        Loader,
        Prefab,
    },
    core::transform::Transform,
    input::is_key_down,
    prelude::*,
    renderer::{
        Flipped,
        PngFormat,
        PosNormTex,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
        VirtualKeyCode,
    },
    ui::{
        Anchor,
        TtfFormat,
        UiPrefab,
        UiText,
        UiTransform,
    },
    utils::scene::BasicScenePrefab,
};

use crate::{
    components::{
        Ball,
        Paddle,
        Side,
    },
    config::{
        ArenaConfig,
        BallConfig,
        PaddleConfig,
    },
    resources::{
        CurrentState,
        Players,
        PlayersActive,
        ScoreText,
    },
    states::paused::PausedState,
};

pub type GamePrefabData = BasicScenePrefab<Vec<PosNormTex>>;

/// The GameState contains the actual game area and gameplay. If the escape key is pressed during
/// gameplay, a state transition to PauseState is initiated.
pub struct GameState {
    game_prefab_handle: Handle<Prefab<GamePrefabData>>,
    game_ui_handle: Handle<UiPrefab>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("GameState.on_start");
        let world = data.world;

        // set CurrentState to CurrentState::Game to enable/unpause game systems
        *world.write_resource::<CurrentState>() = CurrentState::Game;

        // load the sprite sheet necessary to render the graphics
        let sprite_sheet_handle = load_sprite_sheet(world);

        // initialise ui/elements
        self.initialise_prefab(world);
        self.initialise_ui(world);
        initialise_players(world, sprite_sheet_handle.clone());
        initialise_ball(world, sprite_sheet_handle);
        initialise_scoreboard(world);;
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        *data.world.write_resource::<CurrentState>() = CurrentState::Game;
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState::default()));
            }
        }

        Trans::None
    }
}

impl GameState {
    pub fn new(
        game_prefab_handle: Handle<Prefab<GamePrefabData>>,
        game_ui_handle: Handle<UiPrefab>,
    ) -> Self {
        Self {
            game_prefab_handle,
            game_ui_handle,
        }
    }

    fn initialise_prefab(&self, world: &mut World) {
        world
            .create_entity()
            .with(self.game_prefab_handle.clone())
            .build();
    }

    fn initialise_ui(&self, world: &mut World) {
        world
            .create_entity()
            .with(self.game_ui_handle.clone())
            .build();
    }
}

/// Initialise the players.
fn initialise_players(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let (arena_height, arena_width) = {
        let config = &world.read_resource::<ArenaConfig>();
        (config.height, config.width)
    };
    let (paddle_height, paddle_width) = {
        let config = &world.read_resource::<PaddleConfig>();
        (config.height, config.width)
    };

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // correctly position the paddles
    let y = arena_height / 2.0;
    left_transform.set_translation_xyz(paddle_width * 0.5, y, 0.0);
    right_transform.set_translation_xyz(arena_width - paddle_width * 0.5, y, 0.0);

    // assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    // create player 1 entity
    let p1 = world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle {
            side: Side::Left,
            width: paddle_width,
            height: paddle_height,
        })
        .with(left_transform)
        .build();

    // create player 2 entity
    let p2 = world
        .create_entity()
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .with(Paddle {
            side: Side::Right,
            width: paddle_width,
            height: paddle_height,
        })
        .with(right_transform)
        .build();

    world.add_resource(Players { p1, p2 });
    world.add_resource(PlayersActive::default()); // TODO: actually select players
}

/// Initialise the ball.
fn initialise_ball(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let (arena_width, arena_height) = {
        let config = world.read_resource::<ArenaConfig>();
        (config.width, config.height)
    };
    let (ball_velocity_x, ball_velocity_y, ball_radius) = {
        let config = world.read_resource::<BallConfig>();
        (config.velocity.x, config.velocity.y, config.radius)
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(arena_height / 2.0, arena_width / 2.0, 0.0);

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
            velocity: [ball_velocity_x, ball_velocity_y],
            radius: ball_radius,
        })
        .with(local_transform)
        .build();
}

/// Initialise a ui scoreboard
fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
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
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}