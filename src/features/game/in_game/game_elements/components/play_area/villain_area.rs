use bevy::prelude::*;

use crate::features::{
    cards::Card,
    game::{in_game::game_elements::shared::Card3d, state::GameState},
};

use super::CardState;

pub struct VillainAreaPlugin;

impl Plugin for VillainAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_villain_card.run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_villain_card(
    mut commands: Commands,
    game_card_q: Query<(Entity, &Card, &CardState), Without<Card3d>>,
) {
    for (entity, card, state) in game_card_q.iter() {
        if matches!(card, Card::Villain(_)) && state.is_inplay() {
            commands
                .entity(entity)
                // Need to update position
                .insert(Card3d::face_up(card.clone(), Vec3::new(0., 0., 0.)));
        }
    }
}
