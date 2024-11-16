use bevy::prelude::*;

use crate::systems::{AppState, AppStateChangeEvent};

use super::shared::spawn_button;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                handle_button_reaction.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component, Clone)]
enum MainMenuButton {
    Play,
    DeckBuilding,
    Collection,
    Record,
}

const BUTTON_MAP: [(MainMenuButton, &str); 4] = [
    (MainMenuButton::Play, "Play"),
    (MainMenuButton::DeckBuilding, "Deck Building"),
    (MainMenuButton::Collection, "Collection"),
    (MainMenuButton::Record, "Record"),
];

const BUTTON_SIZE: (Val, Val) = (Val::Px(300.), Val::Px(100.));
const BUTTON_COLOR: Color = Color::srgb(0.235, 0.235, 0.235);

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(90.),
                    height: Val::Percent(90.),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
                ..default()
            },
        ))
        .with_children(|main_menu| {
            for (button_component, text) in BUTTON_MAP {
                spawn_button(main_menu, text, BUTTON_COLOR, BUTTON_SIZE).insert(button_component);
            }
        });
}

fn handle_button_reaction(
    commands: Commands,
    mut main_menu_button_q: Query<(&mut BackgroundColor, &Interaction, &MainMenuButton)>,
    mut window_q: Query<&mut Window>,
) {
    let mut window = window_q.get_single_mut().unwrap();
    let mut is_hovered = false;

    for (mut background_color, interaction, main_menu_button) in main_menu_button_q.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                handle_button_click(commands, main_menu_button.clone());
                window.cursor.icon = CursorIcon::default();
                return;
            }
            Interaction::Hovered => {
                background_color.0.set_alpha(0.5);
                is_hovered = true;
            }
            Interaction::None => {
                background_color.0.set_alpha(1.);
            }
        }
    }

    window.cursor.icon = if is_hovered {
        CursorIcon::Pointer
    } else {
        CursorIcon::default()
    };
}

fn handle_button_click(mut commands: Commands, main_menu_button: MainMenuButton) {
    commands.trigger(AppStateChangeEvent({
        match main_menu_button {
            MainMenuButton::Play => AppState::Game,
            MainMenuButton::DeckBuilding => AppState::DeckBuilding,
            MainMenuButton::Collection => AppState::Collection,
            MainMenuButton::Record => AppState::Record,
        }
    }))
}

fn despawn_main_menu(mut commands: Commands, main_menu_q: Query<Entity, With<MainMenu>>) {
    if main_menu_q.is_empty() {
        warn!("Cannot find main menu when despawning");
        return;
    }
    for entity in main_menu_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
