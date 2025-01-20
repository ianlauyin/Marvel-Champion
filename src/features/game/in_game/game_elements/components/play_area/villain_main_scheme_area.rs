use bevy::prelude::*;

use crate::features::{
    cards::Card,
    game::{in_game::game_elements::shared::Card3d, state::GameState},
};

use super::CardState;

pub struct VillainAreaAndMainSchemePlugin;

impl Plugin for VillainAreaAndMainSchemePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_villain_and_main_scheme_added.run_if(in_state(GameState::InGame)),
        );
    }
}

// TODO: Update
const VILLAIN_CARD_POSITION: Vec3 = Vec3::new(-200., 200., 0.);
const MAIN_SCHEME_CARD_POSITION: Vec3 = Vec3::new(0., 200., 0.);

fn handle_villain_and_main_scheme_added(
    mut commands: Commands,
    game_card_q: Query<(Entity, &Card, &CardState), Without<Card3d>>,
) {
    for (entity, card, state) in game_card_q.iter() {
        if matches!(card, Card::Villain(_)) && state.is_inplay() {
            commands
                .entity(entity)
                .insert(Card3d::face_up(card.clone(), VILLAIN_CARD_POSITION));
        }
        if matches!(card, Card::MainSchemeA(_) | Card::MainSchemeB(_)) && state.is_inplay() {
            commands
                .entity(entity)
                .insert(Card3d::face_up(card.clone(), MAIN_SCHEME_CARD_POSITION));
        }
    }
}
