use bevy::prelude::*;

use crate::{
    ui_component::{CustomButton, MainContainer},
    util::SystemUtil,
};

use super::state::AppState;
pub struct MainMenuPlugin;

const CURRENT_STATE: AppState = AppState::MainMenu;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_main_menu)
            .add_systems(
                Update,
                handle_button_reaction.run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(
                OnExit(CURRENT_STATE),
                SystemUtil::cleanup_all::<MainContainer>,
            );
    }
}

#[derive(Component, Clone)]
struct MainMenuButton(AppState);

const BUTTON_MAP: [(MainMenuButton, &str); 3] = [
    (MainMenuButton(AppState::Game), "Play"),
    (MainMenuButton(AppState::DeckBuilding), "Deck Building"),
    (MainMenuButton(AppState::Collection), "Collection"),
];

fn spawn_main_menu(mut commands: Commands) {
    let mut main_container = MainContainer::new();
    main_container.set_space_around();
    commands.spawn(main_container).with_children(|main_menu| {
        for (button_component, text) in BUTTON_MAP {
            main_menu.spawn((button_component, CustomButton::large(text)));
        }
    });
}

fn handle_button_reaction(
    mut next_state: ResMut<NextState<AppState>>,
    main_menu_button_q: Query<(&Interaction, &MainMenuButton), Changed<Interaction>>,
) {
    SystemUtil::handle_button_click(main_menu_button_q, |main_menu_button| {
        next_state.set(main_menu_button.0.clone());
    });
}
