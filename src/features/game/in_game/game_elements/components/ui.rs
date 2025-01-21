use bevy::prelude::*;

use crate::features::cards::Card;
use crate::features::game::state::GameState;

use super::super::shared::Card3d;
use super::Belongs;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_card_added_ui.run_if(in_state(GameState::InGame)),
        );
    }
}

const VILLAIN_CARD_POSITION: Vec2 = Vec2::new(-100., 200.);
const MAIN_SCHEME_CARD_POSITION: Vec2 = Vec2::new(0., 200.);

fn handle_card_added_ui(
    mut commands: Commands,
    game_card_q: Query<(Entity, &Belongs, &Card), Without<Card3d>>,
) {
    let mut villain_cards: Vec<(usize, Entity, &Card)> = Vec::new();
    let mut main_scheme_cards: Vec<(usize, Entity, &Card)> = Vec::new();

    for (entity, belongs, card) in game_card_q.iter() {
        match belongs {
            Belongs::Villain(index) => villain_cards.push((*index, entity, card)),
            Belongs::MainScheme(index) => main_scheme_cards.push((*index, entity, card)),
            _ => {
                commands
                    .entity(entity)
                    .insert(Card3d::face_down(card.clone(), Vec3::ZERO));
            }
        };
    }
    spawn_villain_card_3d(commands.reborrow(), villain_cards);
    spawn_main_scheme_card_3d(commands, main_scheme_cards);
}

fn spawn_villain_card_3d(mut commands: Commands, villain_cards: Vec<(usize, Entity, &Card)>) {
    let villain_card_count = villain_cards.len();
    for (index, entity, card) in villain_cards {
        let z_position = villain_card_count - index;
        let card_3d = if index == 0 {
            Card3d::face_up(
                card.clone(),
                VILLAIN_CARD_POSITION.extend(z_position as f32),
            )
        } else {
            Card3d::face_down(
                card.clone(),
                VILLAIN_CARD_POSITION.extend(z_position as f32),
            )
        };
        commands.entity(entity).insert(card_3d);
    }
}

fn spawn_main_scheme_card_3d(
    mut commands: Commands,
    main_scheme_cards: Vec<(usize, Entity, &Card)>,
) {
    let main_scheme_card_count = main_scheme_cards.len();
    for (index, entity, card) in main_scheme_cards {
        let z_position = main_scheme_card_count - index;
        let card_3d = if index == 0 {
            Card3d::face_up(
                card.clone(),
                MAIN_SCHEME_CARD_POSITION.extend(z_position as f32),
            )
        } else {
            Card3d::face_down(
                card.clone(),
                MAIN_SCHEME_CARD_POSITION.extend(z_position as f32),
            )
        };
        commands.entity(entity).insert(card_3d);
    }
}
