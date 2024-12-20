use bevy::prelude::*;

use crate::{
    features::{
        cards::Identity,
        shared::{ListBuilder, ListItem, MenuBuilder},
    },
    systems::{clean_up, AppState, LoadAsset},
};

use super::{deck_list::EditIdentity, state::DeckBuildingState};

pub struct DeckBuildingIdentityListPlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::SelectIdentity;

impl Plugin for DeckBuildingIdentityListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_hero_list)
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(OnExit(CURRENT_STATE), clean_up::<IdentityList>);
    }
}

#[derive(Component, Clone)]
struct IdentityList;

#[derive(Component, Clone)]
struct IdentityListButton(Identity);

fn spawn_hero_list(mut commands: Commands, asset_server: Res<AssetServer>) {
    let identities = Identity::get_all();
    let list_map = identities
        .iter()
        .map(|identity| {
            (
                IdentityListButton(identity.clone()),
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
        component: IdentityList,
        previous_state: AppState::MainMenu,
        content_child,
    }
    .spawn(commands);
}

fn handle_button_interaction(
    mut commands: Commands,
    button_q: Query<(&Interaction, &IdentityListButton)>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
    mut load_asset: ResMut<LoadAsset>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, button) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            for card in button.0.get_cards() {
                load_asset.add_card(card, &asset_server);
            }
            commands.insert_resource(EditIdentity(button.0.clone()));
            next_state.set(DeckBuildingState::SelectDeck);
            return;
        }
    }
}
