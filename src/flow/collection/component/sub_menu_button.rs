use bevy::prelude::*;

use crate::{cards::SetTrait, component::Card, node_ui::CustomButton, util::SystemUtil};

use super::super::CURRENT_STATE;
use super::card_list::CollectionCardList;

#[derive(Component)]
#[require(CustomButton)]
pub struct SubMenuButton(Box<dyn SetTrait>);

impl SubMenuButton {
    pub fn new(set: Box<dyn SetTrait>) -> Self {
        Self(set)
    }

    pub fn get_cards(&self) -> Vec<Card<'static>> {
        self.0.get_cards()
    }
}

pub struct SubMenuButtonPlugin;

impl Plugin for SubMenuButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_interaction.run_if(in_state(CURRENT_STATE)));
    }
}

fn listen_interaction(
    mut commands: Commands,
    sub_menu_button_q: Query<(&Interaction, &SubMenuButton), Changed<Interaction>>,
) {
    SystemUtil::handle_component_click(sub_menu_button_q, |sub_menu_button| {
        commands.spawn(CollectionCardList::new(sub_menu_button.get_cards()));
        return;
    });
}
