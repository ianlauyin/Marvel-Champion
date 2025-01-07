use bevy::prelude::*;

use crate::{
    features::{
        cards::Identity,
        collection::state::CollectionState,
        shared::{ListBuilder, ListItem, MenuBuilder},
    },
    systems::{clean_up, LoadAsset},
};

use super::{card_list::CollectionHeroIdentity, state::CollectionHeroState};

pub struct CollectionHeroListPlugin;

impl Plugin for CollectionHeroListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Hero), spawn_hero_list)
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(CollectionHeroState::List)),
            )
            .add_systems(OnExit(CollectionState::Hero), clean_up::<HeroList>);
    }
}

#[derive(Component, Clone)]
struct HeroList;

fn spawn_hero_list(mut commands: Commands, asset_server: Res<AssetServer>) {
    let identities = Identity::get_all();
    let list_map = identities
        .iter()
        .map(|identity| {
            (
                identity.clone(),
                ListItem {
                    text: identity.to_string().clone(),
                    image: ImageNode::new(asset_server.load(identity.get_title_image_path()))
                        .with_color(Color::srgb(0.365, 0.365, 0.365)),
                    ..default()
                },
            )
        })
        .collect();
    let content_child = ListBuilder(list_map).spawn(commands.reborrow());
    MenuBuilder {
        next_state: None::<CollectionState>,
        component: HeroList,
        previous_state: CollectionState::Menu,
        content_child,
    }
    .spawn(commands);
}

fn handle_button_interaction(
    mut commands: Commands,
    button_q: Query<(&Interaction, &Identity), With<Button>>,
    mut next_state: ResMut<NextState<CollectionHeroState>>,
    mut load_asset: ResMut<LoadAsset>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, identity) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            for card in identity.get_cards() {
                load_asset.add_card(&card, &asset_server);
            }
            commands.insert_resource(CollectionHeroIdentity(identity.clone()));
            next_state.set(CollectionHeroState::LoadingCards);
            return;
        }
    }
}
