use bevy::prelude::*;

use crate::{
    flow::game::state::GameState,
    node_ui::{ContainerHeader, ContainerHeaderEvent, MainContainer},
    util::SystemUtil,
};

use super::state::PreGameState;

pub struct EnemyMenuPlugin;

const CURRENT_STATE: PreGameState = PreGameState::EnemyMenu;

impl Plugin for EnemyMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_enemy_menu)
            .add_systems(
                Update,
                handle_header_button_click.run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(OnExit(CURRENT_STATE), SystemUtil::cleanup_all::<EnemyMenu>);
    }
}

#[derive(Component)]
struct EnemyMenu;

fn spawn_enemy_menu(mut commands: Commands) {
    let enemy_menu_entity = commands
        .spawn((
            MainContainer::default(),
            EnemyMenu,
            children![ContainerHeader::with_both_button("<", ">")],
        ))
        .id();

    
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    menu_q: Query<&Children, With<EnemyMenu>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_pre_game_state: ResMut<NextState<PreGameState>>,
) {
    for event in event_reader.read() {
        for menu_children in menu_q.iter() {
            match event {
                ContainerHeaderEvent::LeadingButtonPressed(header_entity) => {
                    if menu_children.contains(header_entity) {
                        next_pre_game_state.set(PreGameState::HeroMenu);
                    }
                }
                ContainerHeaderEvent::TrailingButtonPressed(header_entity) => {
                    if menu_children.contains(header_entity) {
                        // TODO: check enemy res and start game
                        next_game_state.set(GameState::InGame);
                    }
                }
            }
        }
    }
}
