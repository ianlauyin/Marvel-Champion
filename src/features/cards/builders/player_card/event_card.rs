use bevy::prelude::Component;

use crate::features::cards::{CardResource, CardTrait};

#[derive(Component)]
pub struct EventCard {
    id: String,
    name: String,
    cost: u8,
    res: Vec<CardResource>,
    traits: Vec<CardTrait>,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
    card_amount_max: u8,
}
