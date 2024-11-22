use bevy::prelude::*;

use crate::systems::{clean_up, AppState};

use super::shared::ButtonBuilder;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                handle_button_reaction.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), clean_up::<MainMenu>);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component, Clone)]
struct MainMenuButton(AppState);

const BUTTON_MAP: [(MainMenuButton, &str); 4] = [
    (MainMenuButton(AppState::Game), "Play"),
    (MainMenuButton(AppState::DeckBuilding), "Deck Building"),
    (MainMenuButton(AppState::Collection), "Collection"),
    (MainMenuButton(AppState::Record), "Record"),
];

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
                border_radius: BorderRadius::all(Val::Px(10.)),
                background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
                ..default()
            },
        ))
        .with_children(|main_menu| {
            for (button_component, text) in BUTTON_MAP {
                ButtonBuilder {
                    text: text.to_string(),
                    color: Color::srgb(0.576, 0.576, 0.576),
                    ..default()
                }
                .spawn(main_menu)
                .insert(button_component);
            }
        });
}

fn handle_button_reaction(
    next_state: ResMut<NextState<AppState>>,
    mut main_menu_button_q: Query<(&Interaction, &MainMenuButton)>,
) {
    for (interaction, main_menu_button) in main_menu_button_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            handle_button_click(next_state, main_menu_button.clone());
            return;
        }
    }
}

fn handle_button_click(
    mut next_state: ResMut<NextState<AppState>>,
    main_menu_button: MainMenuButton,
) {
    next_state.set(main_menu_button.0);
}
