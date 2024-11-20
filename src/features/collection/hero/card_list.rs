use bevy::prelude::*;

use crate::systems::clean_up;

use super::state::CollectionHeroState;

pub struct CollectionHeroCardListPlugin;

impl Plugin for CollectionHeroCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionHeroState::Cards), spawn_hero_cards)
            .add_systems(
                Update,
                handle_card_click.run_if(in_state(CollectionHeroState::Cards)),
            )
            .add_systems(OnExit(CollectionHeroState::Cards), clean_up::<HeroCardList>);
    }
}

#[derive(Component)]
struct HeroCardList;

fn spawn_hero_cards() {}

fn handle_card_click() {}
