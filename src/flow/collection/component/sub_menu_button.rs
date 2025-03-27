use bevy::ecs::component::Component;

use crate::{cards::SetTrait, component::card::CardBasic};

#[derive(Component)]
pub struct SubMenuButton(Box<dyn SetTrait>);

impl SubMenuButton {
    pub fn new(set: Box<dyn SetTrait>) -> Self {
        Self(set)
    }

    pub fn get_cards_info(&self) -> Vec<CardBasic<'static>> {
        self.0.get_card_infos()
    }
}
