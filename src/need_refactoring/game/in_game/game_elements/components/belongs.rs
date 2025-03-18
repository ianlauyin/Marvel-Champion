use bevy::prelude::*;

use super::{CardState, Player};

pub struct BelongsPlugin;

impl Plugin for BelongsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(change_card_state_on_belongs_added);
    }
}

#[derive(Component, PartialEq, Eq, Clone)]
pub enum Belongs {
    Villain(usize),    // Index
    MainScheme(usize), // Index
    OutOfPlay,
    PlayerIdentity(usize),       // 0 for alter ego, others for hero
    PlayerDeck(usize),           // Index
    PlayerDiscardPile(usize),    // Index
    EncounterDeck(usize),        // Index
    EncounterDiscardPile(usize), // Index
}

pub fn change_card_state_on_belongs_added(
    on_add: Trigger<OnAdd, Belongs>,
    mut commands: Commands,
    mut component_q: Query<(Option<&mut CardState>, &Belongs)>,
) {
    let entity = on_add.entity();
    let Ok((card_state_op, belongs)) = component_q.get_mut(entity) else {
        warn!("Cannot find CardState on entity: {:?}", entity);
        return;
    };
    let target_card_state = match belongs {
        Belongs::Villain(0) | Belongs::MainScheme(0) | Belongs::PlayerIdentity(0) => {
            CardState::InPlay
        }
        _ => CardState::OutPlay,
    };
    if let Some(mut card_state) = card_state_op {
        *card_state = target_card_state;
    } else {
        commands.entity(entity).insert(target_card_state);
    }
}
