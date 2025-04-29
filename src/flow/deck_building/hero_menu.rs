use bevy::prelude::*;

use crate::{
    cards::{IdentitySet, SetTrait},
    flow::state::AppState,
    node_ui::{ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer, ScrollingList},
    resource::AssetLoader,
    util::SystemUtil,
};

use super::{resource::DeckBuildingResource, state::DeckBuildingState};

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::HeroMenu;

pub struct HeroMenuPlugin;

impl Plugin for HeroMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_hero_menu)
            .add_systems(
                Update,
                (handle_header_button_click, handle_hero_menu_button_click)
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
        .spawn((MainContainer::new(), HeroMenu))
        .with_children(|container| {
            container.spawn(ContainerHeader::with_leading_button("<"));
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

fn handle_hero_menu_button_click(
    hero_menu_button_q: Query<(&Interaction, &HeroMenuButton), Changed<Interaction>>,
    mut selected_identity: ResMut<DeckBuildingResource>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
) {
    SystemUtil::handle_button_click(hero_menu_button_q, |hero_menu_button| {
        selected_identity.set_identity(hero_menu_button.0.clone());
        next_state.set(DeckBuildingState::DeckMenu);
    });
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    menu_q: Query<&Children, With<HeroMenu>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in event_reader.read() {
        for menu_children in menu_q.iter() {
            match event {
                ContainerHeaderEvent::LeadingButtonPressed(header_entity) => {
                    if menu_children.contains(header_entity) {
                        next_state.set(AppState::MainMenu);
                    }
                }
                _ => {}
            }
        }
    }
}
