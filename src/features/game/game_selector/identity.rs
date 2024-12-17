use bevy::prelude::*;

use crate::{
    features::{
        cards::Identity,
        shared::{handle_previous_interaction, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::{clean_up, AppState},
};

use super::state::GameSelectorState;

pub struct GameSelectorIdentityPlugin;

const CURRENT_STATE: GameSelectorState = GameSelectorState::Identity;

impl Plugin for GameSelectorIdentityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_identity_list)
            .add_systems(Update, handle_previous_interaction(AppState::Game))
            .add_systems(OnExit(CURRENT_STATE), clean_up::<GameIdentityList>);
    }
}

#[derive(Component, Clone)]
struct GameIdentityList;

#[derive(Component, Clone)]
struct GameIdentityButton(Identity);

fn spawn_identity_list(mut commands: Commands, asset_server: Res<AssetServer>) {
    let identities = Identity::get_all();
    let button_map = identities
        .iter()
        .map(|identity| {
            (
                GameIdentityButton(identity.clone()),
                ListItem {
                    text: identity.to_string().clone(),
                    image: ImageNode::new(asset_server.load(identity.get_title_image_path()))
                        .with_color(Color::srgb(0.365, 0.365, 0.365)),
                    ..default()
                },
            )
        })
        .collect();
    MenuBuilder {
        component: GameIdentityList,
        previous_state: AppState::MainMenu,
        list_items: button_map,
        display_method: DisplayMethod::ButtonList,
    }
    .spawn(commands);
}
