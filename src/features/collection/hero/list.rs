use bevy::prelude::*;

use crate::{
    features::{
        cards::{identity_specific_cards, Identity},
        collection::state::CollectionState,
        shared::{handle_previous_interaction, spawn_menu, ButtonMapItem},
    },
    systems::clean_up,
};

pub struct CollectionHeroListPlugin;

impl Plugin for CollectionHeroListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Hero), spawn_hero_list)
            .add_systems(Update, handle_previous_interaction(CollectionState::Hero))
            .add_systems(OnExit(CollectionState::Hero), clean_up::<HeroList>);
    }
}

#[derive(Component)]
struct HeroList;

#[derive(Component)]
struct HeroListButton(Identity);

fn spawn_hero_list(commands: Commands, asset_server: Res<AssetServer>) {
    let identity_cards = identity_specific_cards::get_all(1);
    let button_map = identity_cards
        .iter()
        .map(|(identity, _)| {
            (
                HeroListButton(identity.clone()),
                ButtonMapItem {
                    text: identity.to_string().clone(),
                    image: UiImage::new(asset_server.load(identity.get_title_image_path()))
                        .with_color(Color::srgb(0.365, 0.365, 0.365)),
                    ..default()
                },
            )
        })
        .collect();

    spawn_menu(commands, HeroList, CollectionState::Menu, button_map);
}
