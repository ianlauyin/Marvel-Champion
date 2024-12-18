use bevy::prelude::*;

use crate::{
    features::{
        cards::Identity,
        shared::{DisplayMethod, ListItem, MenuBuilder},
    },
    systems::{clean_up, AppState, Deck},
};

use super::{deck::SelectedIdentity, state::GameSelectorState};

pub struct GameSelectorIdentityPlugin;

const CURRENT_STATE: GameSelectorState = GameSelectorState::Identity;

impl Plugin for GameSelectorIdentityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPlayers>()
            .add_systems(OnEnter(CURRENT_STATE), spawn_identity_list)
            .add_systems(
                Update,
                handle_identity_button_interaction.run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(OnExit(CURRENT_STATE), clean_up::<GameIdentityList>);
    }
}

#[derive(Resource, Default)]
pub struct SelectedPlayers(pub Vec<SelectedPlayer>);

pub struct SelectedPlayer {
    pub identity: Identity,
    pub deck: Deck,
}

#[derive(Component, Clone)]
struct GameIdentityList;

#[derive(Component, Clone)]
struct GameIdentityButton(Identity);

fn spawn_identity_list(commands: Commands, asset_server: Res<AssetServer>) {
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

fn handle_identity_button_interaction(
    mut commands: Commands,
    button_q: Query<(&Interaction, &GameIdentityButton)>,
    mut next_state: ResMut<NextState<GameSelectorState>>,
) {
    for (interaction, button) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            commands.insert_resource(SelectedIdentity(button.0.clone()));
            next_state.set(GameSelectorState::Deck);
            return;
        }
    }
}
