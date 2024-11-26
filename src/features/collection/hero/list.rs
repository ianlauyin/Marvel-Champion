use bevy::prelude::*;

use crate::{
    features::{
        cards::{get_all_identity, CardDatas, Identity},
        collection::state::CollectionState,
        shared::{handle_previous_interaction, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::{clean_up, LoadAsset},
};

use super::{card_list::CollectionHeroIdentity, state::CollectionHeroState};

pub struct CollectionHeroListPlugin;

impl Plugin for CollectionHeroListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Hero), spawn_hero_list)
            .add_systems(Update, handle_previous_interaction(CollectionState::Hero))
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(CollectionHeroState::List)),
            )
            .add_systems(OnExit(CollectionState::Hero), clean_up::<HeroList>);
    }
}

#[derive(Component, Clone)]
struct HeroList;

#[derive(Component, Clone)]
struct HeroListButton(Identity);

fn spawn_hero_list(commands: Commands, asset_server: Res<AssetServer>) {
    let identities = get_all_identity();
    let button_map = identities
        .iter()
        .map(|identity| {
            (
                HeroListButton(identity.clone()),
                ListItem {
                    text: identity.to_string().clone(),
                    image: UiImage::new(asset_server.load(identity.get_title_image_path()))
                        .with_color(Color::srgb(0.365, 0.365, 0.365)),
                    ..default()
                },
            )
        })
        .collect();
    MenuBuilder {
        component: HeroList,
        previous_state: CollectionState::Menu,
        list_items: button_map,
        display_method: DisplayMethod::ButtonList,
    }
    .spawn(commands);
}

fn handle_button_interaction(
    mut commands: Commands,
    button_q: Query<(&Interaction, &HeroListButton)>,
    mut next_state: ResMut<NextState<CollectionHeroState>>,
    mut load_asset: ResMut<LoadAsset>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, button) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            for card in CardDatas::get_identity_cards(button.0.clone()) {
                load_asset.0.push((
                    card.get_card_id(),
                    asset_server.load(card.get_card_image_path()),
                ));
            }
            commands.insert_resource(CollectionHeroIdentity(button.0.clone()));
            next_state.set(CollectionHeroState::LoadingCards);
            return;
        }
    }
}
