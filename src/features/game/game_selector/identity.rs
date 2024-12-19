use bevy::{prelude::*, scene::ron::Number};

use crate::{
    features::{
        cards::Identity,
        game::state::GameState,
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
            .add_systems(OnEnter(GameState::PreGame), spawn_identity_list)
            .add_systems(
                Update,
                (
                    handle_identity_button_interaction,
                    handle_selected_players_changed,
                )
                    .run_if(in_state(CURRENT_STATE)),
            )
            .add_observer(handle_name_tag_added);
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

#[derive(Component)]
struct NumberTag(usize);

fn handle_selected_players_changed(
    mut commands: Commands,
    selected_players: Res<SelectedPlayers>,
    button_q: Query<(Entity, &Children, &GameIdentityButton)>,
    name_tag_q: Query<Entity, With<NumberTag>>,
) {
    if selected_players.is_changed() {
        for (entity, children, game_identity_button) in button_q.iter() {
            if let Some(index) = selected_players
                .0
                .iter()
                .position(|selected_player| selected_player.identity == game_identity_button.0)
            {
                commands.entity(entity).with_child(NumberTag(index + 1));
            } else {
                for child in children.iter() {
                    let Ok(name_tag_entity) = name_tag_q.get(*child) else {
                        continue;
                    };
                    commands.entity(name_tag_entity).despawn();
                }
            }
        }
    }
}

fn handle_name_tag_added(
    on_add: Trigger<OnAdd, NumberTag>,
    mut commands: Commands,
    number_tag_q: Query<&NumberTag>,
) {
    let number_tag = number_tag_q.get(on_add.entity()).unwrap();
    commands
        .entity(on_add.entity())
        .insert(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(5.),
            ..default()
        })
        .with_child(Text::new(number_tag.0.to_string()));
}
