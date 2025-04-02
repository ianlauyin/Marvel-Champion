use bevy::prelude::*;

use crate::{
    cards::{IdentitySet, SetTrait},
    flow::state::AppState,
    resource::AssetLoader,
    ui_component::{
        ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer, ScrollingList,
    },
    util::SystemUtil,
};

use super::resource::{DeckBuildingResource, DeckBuildingState};

pub struct HeroMenuPlugin;

impl Plugin for HeroMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_hero_menu,
                handle_header_button_click,
                handle_hero_menu_button_click,
                despawn_hero_menu,
            )
                .run_if(in_state(AppState::DeckBuilding)),
        );
    }
}

#[derive(Component)]
struct HeroMenu;

#[derive(Component)]
struct HeroMenuButton(IdentitySet);

fn spawn_hero_menu(
    mut commands: Commands,
    asset_loader: Res<AssetLoader>,
    res: Res<DeckBuildingResource>,
) {
    if res.is_changed() && res.get_state() == DeckBuildingState::HeroMenu {
        commands
            .spawn((MainContainer::new(), HeroMenu))
            .with_children(|container| {
                container.spawn(ContainerHeader::with_leading_button("<"));
                container
                    .spawn(Node {
                        width: Val::Percent(100.),
                        align_self: AlignSelf::Stretch,
                        justify_self: JustifySelf::Start,
                        justify_content: JustifyContent::Center,
                        overflow: Overflow::scroll_y(),
                        ..default()
                    })
                    .with_children(|content| {
                        content.spawn(ScrollingList::grid(3, 50.)).with_children(
                            |scrolling_list| {
                                for identity in IdentitySet::get_all() {
                                    let mut button = CustomButton::large(identity.to_str());
                                    if let Some(key) = identity.get_thumbnail_key() {
                                        button.set_image(asset_loader.get(&key).clone());
                                    }
                                    scrolling_list
                                        .spawn((HeroMenuButton(identity.clone()), button));
                                }
                            },
                        );
                    });
            });
    }
}

fn handle_hero_menu_button_click(
    hero_menu_button_q: Query<(&Interaction, &HeroMenuButton), Changed<Interaction>>,
    mut selected_identity: ResMut<DeckBuildingResource>,
) {
    SystemUtil::handle_button_click(hero_menu_button_q, |hero_menu_button| {
        selected_identity.set_identity(hero_menu_button.0.clone());
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
            }
        }
    }
}

fn despawn_hero_menu(
    commands: Commands,
    hero_menu_q: Query<Entity, With<HeroMenu>>,
    res: Res<DeckBuildingResource>,
) {
    if res.is_changed() && res.get_state() != DeckBuildingState::HeroMenu {
        SystemUtil::cleanup_all(commands, hero_menu_q);
    }
}
