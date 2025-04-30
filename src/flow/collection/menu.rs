use bevy::{ecs::spawn::SpawnIter, prelude::*};

use crate::{
    flow::state::AppState,
    node_ui::{ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer},
    util::SystemUtil,
};

use super::{component::CollectionMenuButton, CURRENT_STATE};

pub struct CollectionMenuPlugin;

impl Plugin for CollectionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_menu)
            .add_systems(
                Update,
                handle_header_button_click.run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(
                OnExit(CURRENT_STATE),
                SystemUtil::cleanup_all::<CollectionMenu>,
            );
    }
}

#[derive(Component)]
struct CollectionMenu;

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        MainContainer::default(),
        CollectionMenu,
        children![
            ContainerHeader::with_leading_button("<"),
            (
                Node {
                    margin: UiRect::all(Val::Px(100.)),
                    flex_grow: 1.,
                    display: Display::Grid,
                    column_gap: Val::Px(50.),
                    row_gap: Val::Px(50.),
                    grid_template_columns: vec![RepeatedGridTrack::auto(3)],
                    ..default()
                },
                Children::spawn(SpawnIter(
                    CollectionMenuButton::get_all()
                        .into_iter()
                        .map(|button| { (CustomButton::large(button.get_text()), button) })
                )),
            )
        ],
    ));
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    menu_q: Query<&Children, With<CollectionMenu>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in event_reader.read() {
        for headers in menu_q.iter() {
            if let ContainerHeaderEvent::LeadingButtonPressed(header_entity) = event {
                if headers.contains(header_entity) {
                    next_state.set(AppState::MainMenu);
                }
            }
        }
    }
}
