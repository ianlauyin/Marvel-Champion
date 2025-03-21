use bevy::prelude::*;

use crate::{
    flow::state::AppState,
    ui_component::{ContainerHeader, CustomButton, MainContainer, ScrollingList},
    util::ComponentUtil,
};

use super::component::CollectionMenuButton;

pub struct CollectionMenuPlugin;

impl Plugin for CollectionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Collection), spawn_menu)
            .add_systems(
                OnExit(AppState::Collection),
                ComponentUtil::cleanup_all::<CollectionMenu>,
            );
    }
}

#[derive(Component)]
struct CollectionMenu;

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((MainContainer, CollectionMenu))
        .with_children(|parent| {
            parent.spawn(ContainerHeader::with_leading_button("<".to_string()));
            parent
                .spawn(Node {
                    align_self: AlignSelf::Stretch,
                    margin: UiRect::all(Val::Px(180.)),
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::scroll_y(),
                    ..default()
                })
                .with_children(|content| {
                    content
                        .spawn(ScrollingList::grid(3, 50.))
                        .with_children(|scrolling_list| {
                            for button in CollectionMenuButton::get_all() {
                                scrolling_list
                                    .spawn((CustomButton::menu_text(button.get_text()), button));
                            }
                        });
                });
        });
}
