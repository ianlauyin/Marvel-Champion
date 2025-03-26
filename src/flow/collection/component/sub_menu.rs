use bevy::prelude::*;

use crate::{
    cards::*,
    flow::state::AppState,
    ui_component::{
        ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer, ScrollingList,
    },
};

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
            handle_header_button_click.run_if(in_state(AppState::Collection)),
        );
    }
}

fn on_sub_menu_added(
    trigger: Trigger<OnAdd, SubMenu>,
    mut commands: Commands,
    sub_menu_q: Query<&mut SubMenu>,
) {
    let sub_menu = sub_menu_q.get(trigger.entity()).unwrap();
    commands
        .entity(trigger.entity())
        .insert(MainContainer)
        .with_children(|container| {
            container.spawn(ContainerHeader::with_leading_button("X"));
            container
                .spawn(Node {
                    align_self: AlignSelf::Stretch,
                    justify_self: JustifySelf::Start,
                    flex_grow: 1.,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::scroll_y(),
                    ..default()
                })
                .with_children(|content| {
                    content
                        .spawn(ScrollingList::grid(3, 50.))
                        .with_children(|scrolling_list| {
                            for set in sub_menu.get_sets() {
                                scrolling_list.spawn(CustomButton::menu(set.to_str()));
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
