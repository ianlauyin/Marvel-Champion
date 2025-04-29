use bevy::prelude::*;

use crate::{
    cards::{IdentitySet, SetTrait},
    flow::{
        game::{component::PlayerTag, resource::PlayersInfo},
        state::AppState,
    },
    node_ui::{
        ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer, Popup, ScrollingList,
    },
    resource::AssetLoader,
    util::SystemUtil,
};

use super::{deck_menu::DeckMenu, state::PreGameState};

const CURRENT_STATE: PreGameState = PreGameState::HeroMenu;

pub struct HeroMenuPlugin;

impl Plugin for HeroMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_hero_menu)
            .add_systems(
                Update,
                (
                    handle_header_button_click,
                    handle_hero_menu_button_click,
                    handle_players_info_changed,
                )
                    .run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(OnExit(CURRENT_STATE), SystemUtil::cleanup_all::<HeroMenu>);
    }
}

#[derive(Component)]
struct HeroMenu;

#[derive(Component)]
struct HeroMenuButton(IdentitySet);

fn spawn_hero_menu(mut commands: Commands, asset_loader: Res<AssetLoader>) {
    commands
        .spawn((MainContainer::default(), HeroMenu))
        .with_children(|container| {
            container.spawn(ContainerHeader::with_both_button("<", ">"));
            container
                .spawn(ScrollingList::Grid {
                    column: 3,
                    spacing: 50.,
                })
                .with_children(|scrolling_list| {
                    for identity in IdentitySet::get_all() {
                        let mut button = CustomButton::large(identity.to_str());
                        if let Some(key) = identity.get_thumbnail_key() {
                            button.set_image(asset_loader.get(&key).clone());
                        }
                        scrolling_list.spawn((HeroMenuButton(identity.clone()), button));
                    }
                });
        });
}

fn handle_players_info_changed(
    mut commands: Commands,
    players_info: Res<PlayersInfo>,
    hero_menu_button_q: Query<(Entity, &HeroMenuButton)>,
    player_tag_q: Query<Entity, With<PlayerTag>>,
) {
    if players_info.is_changed() {
        for previous_tag_entity in player_tag_q.iter() {
            commands.entity(previous_tag_entity).despawn();
        }
        for (entity, hero_menu_button) in hero_menu_button_q.iter() {
            if let Some((player_tag, _)) = players_info.get_by_identity(&hero_menu_button.0) {
                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn((
                            player_tag.clone(),
                            Node {
                                position_type: PositionType::Absolute,
                                top: Val::Px(1.),
                                right: Val::Px(1.),
                                ..default()
                            },
                        ))
                        .with_child(Text::new(player_tag.to_string()));
                });
            }
        }
    }
}

fn handle_hero_menu_button_click(
    mut commands: Commands,
    hero_menu_button_q: Query<(&Interaction, &HeroMenuButton), Changed<Interaction>>,
) {
    SystemUtil::handle_button_click(hero_menu_button_q, |hero_menu_button| {
        commands.spawn(DeckMenu(hero_menu_button.0.clone()));
    });
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    mut commands: Commands,
    menu_q: Query<&Children, With<HeroMenu>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_pre_game_state: ResMut<NextState<PreGameState>>,
    players_info: Res<PlayersInfo>,
) {
    for event in event_reader.read() {
        for menu_children in menu_q.iter() {
            match event {
                ContainerHeaderEvent::LeadingButtonPressed(header_entity) => {
                    if menu_children.contains(header_entity) {
                        next_app_state.set(AppState::MainMenu);
                    }
                }
                ContainerHeaderEvent::TrailingButtonPressed(header_entity) => {
                    if menu_children.contains(header_entity) {
                        if players_info.have_players() {
                            next_pre_game_state.set(PreGameState::EnemyMenu);
                        } else {
                            commands.spawn(Popup::new("You need to choose players".to_string()));
                        }
                    }
                }
            }
        }
    }
}
