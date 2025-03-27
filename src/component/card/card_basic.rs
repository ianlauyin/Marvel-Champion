use bevy::prelude::Component;

use crate::cards::Belongs;

#[derive(Component, Clone)]
pub struct CardBasic<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub sub_name: Option<&'a str>,
    pub unique: bool,
    pub card_amount_max: u8,
    pub belongs: Belongs,
    pub is_vertical: bool,
}

impl CardBasic<'_> {
    pub fn get_key(&self) -> String {
        let belong_key = self.belongs.get_key();
        format!("cards/{belong_key}/{}", self.id)
    }
}
