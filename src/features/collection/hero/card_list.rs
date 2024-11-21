use bevy::prelude::*;

use crate::{
    features::{
        cards::{Card, Identity},
        shared::{
            handle_previous_interaction,
            menu::{spawn_card_list, spawn_menu, ListItem},
        },
    },
    systems::clean_up,
};

use super::state::CollectionHeroState;

pub struct CollectionHeroCardListPlugin;

impl Plugin for CollectionHeroCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionHeroState::Cards), spawn_hero_cards)
            .add_systems(
                Update,
                handle_card_click.run_if(in_state(CollectionHeroState::Cards)),
            )
            .add_systems(
                Update,
                handle_previous_interaction(CollectionHeroState::Cards),
            )
            .add_systems(OnExit(CollectionHeroState::Cards), clean_up::<HeroCardList>);
    }
}

#[derive(Resource)]
pub struct CollectionHeroIdentity(pub Identity);

#[derive(Component)]
struct HeroCardList;

#[derive(Component)]
struct HeroCardButton(Card);

fn spawn_hero_cards(
    commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionHeroIdentity>,
) {
    let list_items = identity
        .0
        .get_cards(1)
        .iter()
        .map(|card| {
            (
                HeroCardButton(card.clone()),
                ListItem {
                    image: UiImage::new(asset_server.load(card.get_card_image_path())),
                    ..default()
                },
            )
        })
        .collect();

    spawn_menu(
        commands,
        HeroCardList,
        CollectionHeroState::List,
        list_items,
        spawn_card_list,
    );
}

fn handle_card_click() {}
