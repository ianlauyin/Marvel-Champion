use bevy::prelude::*;

use crate::{
    features::{
        cards::Identity,
        shared::{CardDetailButton, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionHeroState;

pub struct CollectionHeroCardListPlugin;

impl Plugin for CollectionHeroCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionHeroState::Cards), spawn_hero_cards)
            .add_systems(OnExit(CollectionHeroState::Cards), clean_up::<HeroCardList>);
    }
}

#[derive(Resource)]
pub struct CollectionHeroIdentity(pub Identity);

#[derive(Component, Clone)]
struct HeroCardList;

fn spawn_hero_cards(
    commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionHeroIdentity>,
) {
    let list_items = identity
        .0
        .get_cards()
        .iter()
        .map(|card| {
            (
                CardDetailButton(card.clone()),
                ListItem {
                    image: ImageNode::new(asset_server.load(card.get_image_path())),
                    ..default()
                },
            )
        })
        .collect();
    MenuBuilder {
        component: HeroCardList,
        previous_state: CollectionHeroState::List,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}
