use bevy::prelude::*;

use crate::features::cards::Card;
use crate::features::game::in_game::game_elements::PlayerNumber;
use crate::features::game::in_game::state::InGameState;

use super::super::shared::Card3d;
use super::{Belongs, Player};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_card_added_ui.run_if(in_state(InGameState::Setup)),
        );
    }
}

const VILLAIN_CARD_POSITION: Vec2 = Vec2::new(-100., 200.);
const MAIN_SCHEME_CARD_POSITION: Vec2 = Vec2::new(0., 200.);
const ENCOUNTER_DECK_CARD_POSITION: Vec2 = Vec2::new(100., 200.);
const OUT_OF_PLAY_CARD_POSITION: Vec2 = Vec2::new(-350., 350.);
const PLAYER_DECK_CARD_POSITION: Vec2 = Vec2::new(-100., -200.);
// TODO update Identity card position
const IDENTITY_CARD_POSITION: Vec2 = Vec2::ZERO;

fn handle_card_added_ui(
    mut commands: Commands,
    game_card_q: Query<(Entity, &Belongs, &Card, Option<&Player>), Without<Card3d>>,
    player_number: Res<PlayerNumber>,
) {
    let mut villain_cards: Vec<(usize, Entity, &Card)> = Vec::new();
    let mut main_scheme_cards: Vec<(usize, Entity, &Card)> = Vec::new();
    let mut encounter_deck_cards: Vec<(usize, Entity, &Card)> = Vec::new();
    let mut out_of_play_cards: Vec<(usize, Entity, &Card)> = Vec::new();
    let mut identity_cards: [Vec<(bool, Entity, &Card)>; 4] =
        [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut player_deck_cards: [Vec<(usize, Entity, &Card)>; 4] =
        [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for (entity, belongs, card, player_op) in game_card_q.iter() {
        match belongs {
            Belongs::Villain(index) => villain_cards.push((*index, entity, card)),
            Belongs::MainScheme(index) => main_scheme_cards.push((*index, entity, card)),
            Belongs::EncounterDeck(index) => encounter_deck_cards.push((*index, entity, card)),
            Belongs::OutOfPlay => {
                out_of_play_cards.push((out_of_play_cards.len(), entity, card));
            }
            Belongs::PlayerDeck(index) => {
                let Some(player) = player_op else {
                    panic!("PlayerDeck should have Player component");
                };
                player_deck_cards[player.tag - 1].push((*index, entity, card))
            }
            Belongs::PlayerIdentity(tag) => {
                let Some(player) = player_op else {
                    panic!("PlayerIdentity should have Player component");
                };
                identity_cards[player.tag - 1].push((*tag == player.identity_tag, entity, card));
            }
            _ => {
                warn!(
                    "Should not have left over belongs card: {:?}",
                    card.get_name()
                );
            }
        };
    }
    spawn_face_up_cards_3d(commands.reborrow(), villain_cards, VILLAIN_CARD_POSITION);
    spawn_face_up_cards_3d(
        commands.reborrow(),
        main_scheme_cards,
        MAIN_SCHEME_CARD_POSITION,
    );
    spawn_face_down_cards_3d(
        commands.reborrow(),
        encounter_deck_cards,
        ENCOUNTER_DECK_CARD_POSITION,
    );
    spawn_face_down_cards_3d(
        commands.reborrow(),
        out_of_play_cards,
        OUT_OF_PLAY_CARD_POSITION,
    );
    for (index, cards) in identity_cards.iter().enumerate() {
        let mut position = IDENTITY_CARD_POSITION;
        alter_player_card_position(&mut position, player_number.0, index + 1);
        spawn_identity_cards_3d(commands.reborrow(), index + 1, cards.clone(), position);
    }
    for (index, cards) in player_deck_cards.iter().enumerate() {
        let mut position = PLAYER_DECK_CARD_POSITION;
        alter_player_card_position(&mut position, player_number.0, index + 1);
        spawn_face_down_cards_3d(commands.reborrow(), cards.clone(), position);
    }
}

fn spawn_face_up_cards_3d(
    mut commands: Commands,
    cards: Vec<(usize, Entity, &Card)>,
    position: Vec2,
) {
    let card_count = cards.len();
    for (index, entity, card) in cards {
        let z_position = card_count - index;
        commands.entity(entity).insert(Card3d::face_up(
            card.clone(),
            position.extend(z_position as f32),
        ));
    }
}

fn spawn_face_down_cards_3d(
    mut commands: Commands,
    cards: Vec<(usize, Entity, &Card)>,
    position: Vec2,
) {
    let card_count = cards.len();
    for (index, entity, card) in cards {
        let z_position = card_count - index;
        commands.entity(entity).insert(Card3d::face_down(
            card.clone(),
            position.extend(z_position as f32),
        ));
    }
}

fn spawn_identity_cards_3d(
    mut commands: Commands,
    current_identity_tag: usize,
    cards: Vec<(bool, Entity, &Card)>,
    position: Vec2,
) {
    let card_count = cards.len();
    let mut current_index = 0;
    for (is_current_tag, entity, card) in cards {
        let mut z_position = current_index;
        if is_current_tag {
            z_position = card_count - 1;
        } else {
            current_index += 1;
        }
        println!(
            "is_current_tag : {} \nz_position: {} \n card: {}",
            is_current_tag,
            z_position,
            card.get_name()
        );
        commands.entity(entity).insert(Card3d::face_up(
            card.clone(),
            position.extend(z_position as f32),
        ));
    }
}

fn alter_player_card_position(position: &mut Vec2, player_number: usize, player_tag: usize) {
    if player_number == 1 {
        return;
    }
}
