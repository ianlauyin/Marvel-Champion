use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{
    features::{
        cards::Identity,
        shared::{ListBuilder, ListItem, MenuBuilder},
    },
    systems::{clean_up, Deck, DecksStorage},
};

use super::{
    identity::{SelectedPlayer, SelectedPlayers},
    state::GameSelectorState,
};

pub struct GameSelectorDeckPlugin;

const CURRENT_STATE: GameSelectorState = GameSelectorState::Deck;

impl Plugin for GameSelectorDeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_deck_list)
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(OnExit(CURRENT_STATE), clean_up::<IdentityDeckList>);
    }
}

#[derive(Resource)]
pub struct SelectedIdentity(pub Identity);

#[derive(Component, Clone)]
struct IdentityDeckList;

#[derive(Component, Clone)]
struct DeckListButton(Deck);

fn spawn_deck_list(
    mut commands: Commands,
    pkv: ResMut<PkvStore>,
    selected_identity: Res<SelectedIdentity>,
) {
    let mut deck_storage = DecksStorage {
        pkv,
        identity: selected_identity.0.clone(),
    };
    let decks = deck_storage.get_decks();
    let list_map: Vec<(DeckListButton, ListItem)> = decks
        .iter()
        .map(|deck| {
            (
                DeckListButton(deck.clone()),
                ListItem {
                    text: deck.name.clone(),
                    color: Color::srgb(0.576, 0.576, 0.576),
                    ..default()
                },
            )
        })
        .collect();

    let content_child = ListBuilder(list_map).spawn(commands.reborrow());
    MenuBuilder {
        component: IdentityDeckList,
        previous_state: GameSelectorState::Identity,
        content_child,
    }
    .spawn(commands);
}

fn handle_button_interaction(
    button_q: Query<(&Interaction, &DeckListButton)>,
    mut next_state: ResMut<NextState<GameSelectorState>>,
    selected_identity: Res<SelectedIdentity>,
    mut selected_players: ResMut<SelectedPlayers>,
) {
    for (interaction, button) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            selected_players.0.push(SelectedPlayer {
                identity: selected_identity.0.clone(),
                deck: button.0.clone(),
            });
            next_state.set(GameSelectorState::Identity);
            return;
        }
    }
}
