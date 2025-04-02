use bevy::prelude::*;

use crate::{
    cards::*,
    resource::AssetLoader,
    ui_component::{
        ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer, ScrollingList,
    },
    util::SystemUtil,
};

use super::super::CURRENT_STATE;
use super::{card_list::CollectionCardList, SubMenuButton};

#[derive(Component)]
pub enum SubMenu {
    Aspect,
    IdentitySet,
    ModularSet,
    Scenario,
    StandardSet,
    ExpertSet,
}

impl SubMenu {
    fn get_sets(&self) -> Vec<Box<dyn SetTrait>> {
        match self {
            Self::Aspect => Aspect::get_boxed_all(),
            Self::IdentitySet => IdentitySet::get_boxed_all(),
            Self::ModularSet => ModularSet::get_boxed_all(),
            Self::Scenario => Scenario::get_boxed_all(),
            Self::StandardSet => StandardSet::get_boxed_all(),
            Self::ExpertSet => ExpertSet::get_boxed_all(),
        }
    }
}

pub struct SubMenuPlugin;

impl Plugin for SubMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_sub_menu_added).add_systems(
            Update,
            (handle_sub_menu_button_click, handle_header_button_click)
                .run_if(in_state(CURRENT_STATE)),
        );
    }
}

fn on_sub_menu_added(
    trigger: Trigger<OnAdd, SubMenu>,
    mut commands: Commands,
    sub_menu_q: Query<&mut SubMenu>,
    asset_loader: Res<AssetLoader>,
) {
    let sub_menu = sub_menu_q.get(trigger.entity()).unwrap();
    commands
        .entity(trigger.entity())
        .insert(MainContainer::new())
        .with_children(|container| {
            container.spawn(ContainerHeader::with_leading_button("X"));
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
                    content
                        .spawn(ScrollingList::grid(3, 50.))
                        .with_children(|scrolling_list| {
                            for set in sub_menu.get_sets() {
                                let mut button = CustomButton::large(set.to_str());
                                if let Some(color) = set.get_color() {
                                    button.set_color(color);
                                }
                                if let Some(key) = set.get_thumbnail_key() {
                                    button.set_image(asset_loader.get(&key).clone());
                                }
                                scrolling_list.spawn((SubMenuButton::new(set), button));
                            }
                        });
                });
        });
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    mut commands: Commands,
    menu_q: Query<(Entity, &Children), With<SubMenu>>,
) {
    for event in event_reader.read() {
        for (entity, menu_children) in menu_q.iter() {
            match event {
                ContainerHeaderEvent::LeadingButtonPressed(header_entity) => {
                    if menu_children.contains(header_entity) {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

fn handle_sub_menu_button_click(
    mut commands: Commands,
    sub_menu_button_q: Query<(&Interaction, &SubMenuButton), Changed<Interaction>>,
) {
    SystemUtil::handle_button_click(sub_menu_button_q, |sub_menu_button| {
        commands.spawn(CollectionCardList::new(sub_menu_button.get_cards_info()));
        return;
    });
}
