use std::collections::HashMap;

use amethyst::{
    assets::{
        Handle,
        Prefab,
    },
    ecs::prelude::Entity,
    input::{
        is_close_requested,
        is_key_down,
    },
    prelude::*,
    renderer::{
        SpriteSheetHandle,
        VirtualKeyCode,
    },
    ui::{
        Anchor,
        FontHandle,
        UiPrefab,
        UiText,
        UiTransform,
    },
};

use crate::resources::PlayersActive;

use super::game::{
    GamePrefabData,
    GameState,
};

#[derive(Eq, PartialEq, Hash)]
enum MenuItem {
    SinglePlayer,
    MultiPlayer,
    Quit,
}

impl MenuItem {
    fn next(&self) -> Self {
        match *self {
            MenuItem::SinglePlayer => MenuItem::MultiPlayer,
            MenuItem::MultiPlayer => MenuItem::Quit,
            MenuItem::Quit => MenuItem::SinglePlayer
        }
    }

    fn previous(&self) -> Self {
        match *self {
            MenuItem::SinglePlayer => MenuItem::Quit,
            MenuItem::MultiPlayer => MenuItem::SinglePlayer,
            MenuItem::Quit => MenuItem::MultiPlayer
        }
    }
}

pub struct MenuState {
    /// The current, selected `MenuItem`.
    current_menu_item: MenuItem,

    /// The list of `MenuItem`s, holding references to the `MenuItem` `Entity`s.
    menu_items: HashMap<MenuItem, Entity>,

    /// `Asset` handles.
    scene_handle: Handle<Prefab<GamePrefabData>>,
    game_ui_handle: Handle<UiPrefab>,
    paused_ui_handle: Handle<UiPrefab>,
    sprite_sheet_handle: SpriteSheetHandle,
    font_handle: FontHandle,
}

impl MenuState {
    /// Create a new `MenuState` and populate member variables with sane default values.
    pub fn new(
        scene_handle: Handle<Prefab<GamePrefabData>>,
        game_ui_handle: Handle<UiPrefab>,
        paused_ui_handle: Handle<UiPrefab>,
        sprite_sheet_handle: SpriteSheetHandle,
        font_handle: FontHandle,
    ) -> Self {
        Self {
            current_menu_item: MenuItem::SinglePlayer,
            menu_items: HashMap::new(),
            scene_handle,
            game_ui_handle,
            paused_ui_handle,
            sprite_sheet_handle,
            font_handle,
        }
    }

    fn select_next_menu_item(&mut self, world: &mut World) {
        // Set current menu item color.
        if let Some(entity) = self.menu_items.get(&self.current_menu_item) {
            set_ui_text_color(world, entity, [0.25, 0.25, 0.25, 1.0]);
        }

        self.current_menu_item = self.current_menu_item.next();

        // Set new menu item color.
        if let Some(entity) = self.menu_items.get(&self.current_menu_item) {
            set_ui_text_color(world, entity, [1.0, 1.0, 1.0, 1.0]);
        }
    }

    fn select_previous_menu_item(&mut self, world: &mut World) {
        // Set current menu item color.
        if let Some(entity) = self.menu_items.get(&self.current_menu_item) {
            set_ui_text_color(world, entity, [0.25, 0.25, 0.25, 1.0]);
        }

        self.current_menu_item = self.current_menu_item.previous();

        // Set new menu item color.
        if let Some(entity) = self.menu_items.get(&self.current_menu_item) {
            set_ui_text_color(world, entity, [1.0, 1.0, 1.0, 1.0]);
        }
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("MenuState.on_start");
        let world = data.world;

        // Initialise ui elements
        self.menu_items.insert(MenuItem::SinglePlayer, world
            .create_entity()
            .with(UiTransform::new(
                "sp".to_string(),
                Anchor::Middle,
                0.0, 50.0, 1.0, 200.0, 50.0,
            ))
            .with(UiText::new(
                self.font_handle.clone(),
                "1 Player".to_string(),
                [1.0, 1.0, 1.0, 1.0],
                40.0,
            )).build(),
        );

        self.menu_items.insert(MenuItem::MultiPlayer, world
            .create_entity()
            .with(UiTransform::new(
                "mp".to_string(),
                Anchor::Middle,
                0.0, 0.0, 1.0, 200.0, 50.0,
            ))
            .with(UiText::new(
                self.font_handle.clone(),
                "2 Players".to_string(),
                [0.25, 0.25, 0.25, 1.0],
                40.0,
            )).build(),
        );

        self.menu_items.insert(MenuItem::Quit, world
            .create_entity()
            .with(UiTransform::new(
                "sp".to_string(),
                Anchor::Middle,
                0.0, -50.0, 1.0, 200.0, 50.0,
            ))
            .with(UiText::new(
                self.font_handle.clone(),
                "Quit".to_string(),
                [0.25, 0.25, 0.25, 1.0],
                40.0,
            )).build(),
        );
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        info!("MenuState.on_stop");
        self.menu_items.values().for_each(|&entity| {
            let _ = data.world.delete_entity(entity);
        })
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        let world = data.world;

        // handle window events and quit the current State if the Escape button is pressed
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // toggle previous menu item
            if is_key_down(&event, VirtualKeyCode::Up) || is_key_down(&event, VirtualKeyCode::W) {
                self.select_previous_menu_item(world);
            }

            // toggle next menu item
            if is_key_down(&event, VirtualKeyCode::Down) || is_key_down(&event, VirtualKeyCode::S) {
                self.select_next_menu_item(world);
            }

            // execute menu item command
            if is_key_down(&event, VirtualKeyCode::Return) {
                match self.current_menu_item {
                    MenuItem::SinglePlayer => {
                        world.add_resource(PlayersActive {
                            p1: true,
                            p2: false,
                        });
                    },
                    MenuItem::MultiPlayer => {
                        world.add_resource(PlayersActive {
                            p1: true,
                            p2: true,
                        });
                    },
                    MenuItem::Quit => return Trans::Quit,
                };

                // remove MenuState from the stack and switch to GameState
                return Trans::Switch(Box::new(GameState::new(
                    self.scene_handle.clone(),
                    self.game_ui_handle.clone(),
                    self.paused_ui_handle.clone(),
                    self.sprite_sheet_handle.clone(),
                    self.font_handle.clone(),
                )));
            }
        }

        // event was not of type StateEvent, so no transition is required
        Trans::None
    }
}

fn set_ui_text_color(world: &mut World, entity: &Entity, color: [f32; 4]) {
    if let Some(text) = world.write_storage::<UiText>().get_mut(*entity) {
        text.color = color;
    }
}