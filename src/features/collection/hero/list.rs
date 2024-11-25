use bevy::prelude::*;

use crate::{
    features::{
        cards::{get_all_identity, Identity},
        collection::state::CollectionState,
        shared::{
            handle_previous_interaction,
            menu::{spawn_list, spawn_menu, ListItem},
        },
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

#[derive(Component)]
struct HeroList;

#[derive(Component)]
struct HeroListButton(Identity);

fn spawn_hero_list(commands: Commands, asset_server: Res<AssetServer>) {
    let identities = get_all_identity();
    let button_map = identities
        .iter()
        .map(|(identity)| {
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

    spawn_menu(
        commands,
        HeroList,
        CollectionState::Menu,
        button_map,
        spawn_list,
    );
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
            for card in button.0.get_cards() {
                load_asset
                    .0
                    .push(asset_server.load(card.get_card_image_path()));
            }
            commands.insert_resource(CollectionHeroIdentity(button.0.clone()));
            next_state.set(CollectionHeroState::LoadingCards);
            return;
        }
    }
}
