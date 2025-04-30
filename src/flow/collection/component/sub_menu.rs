use bevy::prelude::*;

use crate::{
    cards::*,
    node_ui::{ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer, ScrollingList},
    resource::AssetLoader,
};

use super::super::CURRENT_STATE;
use super::SubMenuButton;

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
        app.add_systems(
            Update,
            handle_header_interaction.run_if(in_state(CURRENT_STATE)),
        )
        .add_observer(on_added);
    }
}

fn on_added(
    trigger: Trigger<OnAdd, SubMenu>,
    mut commands: Commands,
    sub_menu_q: Query<&mut SubMenu>,
    asset_loader: Res<AssetLoader>,
) {
    let main_container = commands
        .entity(trigger.target())
        .insert((
            MainContainer::default(),
            children![ContainerHeader::with_leading_button("X")],
        ))
        .id();

    let content_container = commands
        .spawn((
            ScrollingList::Grid {
                column: 3,
                spacing: 50.,
            },
            ChildOf(main_container),
        ))
        .id();

    let sub_menu = sub_menu_q.get(trigger.target()).unwrap();
    for set in sub_menu.get_sets() {
        let mut button = CustomButton::large(set.to_str());
        if let Some(color) = set.get_color() {
            button.set_color(color);
        }
        if let Some(key) = set.get_thumbnail_key() {
            button.set_image(asset_loader.get(&key).clone());
        }
        commands.spawn((SubMenuButton::new(set), button, ChildOf(content_container)));
    }
}

fn handle_header_interaction(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    mut commands: Commands,
    menu_q: Query<(Entity, &Children), With<SubMenu>>,
) {
    for event in event_reader.read() {
        for (entity, menu_children) in menu_q.iter() {
            if let ContainerHeaderEvent::LeadingButtonPressed(header_entity) = event {
                if menu_children.contains(header_entity) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
