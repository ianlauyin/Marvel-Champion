use bevy::prelude::*;

use crate::{
    features::{
        cards::{core_spider_man, Card},
        shared::menu::{spawn_card_list, spawn_menu, ListItem},
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
            .add_systems(OnExit(CollectionHeroState::Cards), clean_up::<HeroCardList>);
    }
}

#[derive(Component)]
struct HeroCardList;

#[derive(Component)]
struct HeroCardButton(Card);

fn spawn_hero_cards(commands: Commands, asset_server: Res<AssetServer>) {
    // testing
    let cards: Vec<(HeroCardButton, ListItem)> = core_spider_man::get_all(1)
        .iter()
        .map(|card| {
            (
                HeroCardButton(card.clone()),
                ListItem {
                    image: UiImage::new(
                        asset_server.get_handle(card.get_card_image_path()).unwrap(),
                    ),
                    ..default()
                },
            )
        })
        .collect();
    spawn_menu(
        commands,
        HeroCardList,
        CollectionHeroState::List,
        cards,
        spawn_card_list,
    );
}

fn handle_card_click() {}
