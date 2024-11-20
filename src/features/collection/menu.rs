use bevy::prelude::*;

use crate::{
    features::shared::{
        handle_previous_interaction,
        menu::{spawn_list, spawn_menu, ListItem},
    },
    systems::{clean_up, AppState},
};

use super::state::CollectionState;

pub struct CollectionMenuPlugin;

impl Plugin for CollectionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Menu), spawn_card_type_menu)
            .add_systems(
                Update,
                handle_button_reaction.run_if(in_state(CollectionState::Menu)),
            )
            .add_systems(Update, handle_previous_interaction(AppState::Collection))
            .add_systems(OnExit(CollectionState::Menu), clean_up::<CardTypeMenu>);
    }
}

#[derive(Component)]
struct CardTypeMenu;

#[derive(Component, Clone)]
struct CardTypeButton(CollectionState);

fn spawn_card_type_menu(commands: Commands) {
    let button_map = vec![
        (
            CardTypeButton(CollectionState::Hero),
            ListItem {
                text: "Hero".to_string(),
                color: Color::srgb(0.576, 0.576, 0.576),
                ..default()
            },
        ),
        (
            CardTypeButton(CollectionState::Basic),
            ListItem {
                text: "Basic".to_string(),
                color: Color::srgb(0.576, 0.576, 0.576),
                ..default()
            },
        ),
        (
            CardTypeButton(CollectionState::Aggression),
            ListItem {
                text: "Aggression".to_string(),
                color: Color::srgb(0.741, 0.192, 0.192),
                ..default()
            },
        ),
        (
            CardTypeButton(CollectionState::Leadership),
            ListItem {
                text: "Leadership".to_string(),
                color: Color::srgb(0.125, 0.769, 0.882),
                ..default()
            },
        ),
        (
            CardTypeButton(CollectionState::Protection),
            ListItem {
                text: "Protection".to_string(),
                color: Color::srgb(0.075, 0.773, 0.075),
                ..default()
            },
        ),
        (
            CardTypeButton(CollectionState::Justice),
            ListItem {
                text: "Justice".to_string(),
                color: Color::srgb(0.871, 0.941, 0.086),
                ..default()
            },
        ),
        (
            CardTypeButton(CollectionState::Pool),
            ListItem {
                text: "Pool".to_string(),
                color: Color::srgb(0.89, 0.149, 0.816),
                ..default()
            },
        ),
        (
            CardTypeButton(CollectionState::Villain),
            ListItem {
                text: "Villain".to_string(),
                color: Color::srgb(0.576, 0.576, 0.576),
                ..default()
            },
        ),
        (
            CardTypeButton(CollectionState::Modular),
            ListItem {
                text: "Modular".to_string(),
                color: Color::srgb(0.576, 0.576, 0.576),
                ..default()
            },
        ),
    ];
    spawn_menu(
        commands,
        CardTypeMenu,
        AppState::MainMenu,
        button_map,
        spawn_list,
    );
}

fn handle_button_reaction(
    next_state: ResMut<NextState<CollectionState>>,
    mut card_type_button_q: Query<(&Interaction, &CardTypeButton)>,
) {
    for (interaction, card_type_button) in card_type_button_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            handle_button_click(next_state, card_type_button.clone());
            return;
        }
    }
}

fn handle_button_click(
    mut next_state: ResMut<NextState<CollectionState>>,
    card_type_button: CardTypeButton,
) {
    next_state.set(card_type_button.0);
}
