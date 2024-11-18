use bevy::prelude::*;

use crate::{
    features::shared::{handle_previous_interaction, spawn_menu},
    systems::{clean_up, AppState},
};

use super::state::{CollectionState, CollectionStateChangeEvent};

pub struct CollectionMenuPlugin;

impl Plugin for CollectionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Menu), spawn_card_type_menu)
            .add_systems(
                Update,
                (
                    handle_button_reaction,
                    handle_previous_interaction::<AppState>,
                )
                    .run_if(in_state(CollectionState::Menu)),
            )
            .add_systems(OnExit(CollectionState::Menu), clean_up::<CardTypeMenu>);
    }
}

#[derive(Component)]
struct CardTypeMenu;

#[derive(Component, Clone)]
enum CardTypeButton {
    Hero,
    Basic,
    Aggression,
    Leadership,
    Protection,
    Justice,
    Pool,
    Villain,
    Modular,
}

const BUTTON_MAP: [(CardTypeButton, &str, Option<Color>); 9] = [
    (CardTypeButton::Hero, "Hero", None),
    (CardTypeButton::Basic, "Basic", None),
    (
        CardTypeButton::Aggression,
        "Aggression",
        Some(Color::srgb(0.741, 0.192, 0.192)),
    ),
    (
        CardTypeButton::Leadership,
        "Leadership",
        Some(Color::srgb(0.125, 0.769, 0.882)),
    ),
    (
        CardTypeButton::Protection,
        "Protection",
        Some(Color::srgb(0.075, 0.773, 0.075)),
    ),
    (
        CardTypeButton::Justice,
        "Justice",
        Some(Color::srgb(0.871, 0.941, 0.086)),
    ),
    (
        CardTypeButton::Pool,
        "Pool",
        Some(Color::srgb(0.89, 0.149, 0.816)),
    ),
    (CardTypeButton::Villain, "Villain", None),
    (CardTypeButton::Modular, "Modular", None),
];

fn spawn_card_type_menu(commands: Commands) {
    spawn_menu(
        commands,
        CardTypeMenu,
        AppState::MainMenu,
        BUTTON_MAP.to_vec(),
    );
}

fn handle_button_reaction(
    commands: Commands,
    mut card_type_button_q: Query<(&Interaction, &CardTypeButton)>,
) {
    for (interaction, card_type_button) in card_type_button_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            handle_button_click(commands, card_type_button.clone());
            return;
        }
    }
}

fn handle_button_click(mut commands: Commands, card_type_button: CardTypeButton) {
    commands.trigger(CollectionStateChangeEvent({
        match card_type_button {
            CardTypeButton::Hero => CollectionState::Hero,
            CardTypeButton::Basic => CollectionState::Basic,
            CardTypeButton::Aggression => CollectionState::Aggression,
            CardTypeButton::Leadership => CollectionState::Leadership,
            CardTypeButton::Protection => CollectionState::Protection,
            CardTypeButton::Justice => CollectionState::Justice,
            CardTypeButton::Pool => CollectionState::Pool,
            CardTypeButton::Villain => CollectionState::Villain,
            CardTypeButton::Modular => CollectionState::Modular,
        }
    }))
}
