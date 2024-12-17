use bevy::prelude::*;

use crate::systems::{clean_up, AppState};

use super::shared::CustomButton;
pub struct MainMenuPlugin;

const CURRENT_STATE: AppState = AppState::MainMenu;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_main_menu)
            .add_systems(
                Update,
                handle_button_reaction.run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(OnExit(CURRENT_STATE), clean_up::<MainMenu>);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component, Clone)]
struct MainMenuButton(AppState);

const BUTTON_MAP: [(MainMenuButton, &str); 3] = [
    (MainMenuButton(AppState::Game), "Play"),
    (MainMenuButton(AppState::DeckBuilding), "Deck Building"),
    (MainMenuButton(AppState::Collection), "Collection"),
];

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenu,
            Node {
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
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
        ))
        .with_children(|main_menu| {
            for (button_component, text) in BUTTON_MAP {
                main_menu.spawn((
                    button_component,
                    CustomButton {
                        text: text.to_string(),
                        color: Color::srgb(0.576, 0.576, 0.576),
                        ..default()
                    },
                ));
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
