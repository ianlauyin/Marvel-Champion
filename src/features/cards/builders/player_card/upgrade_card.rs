use bevy::prelude::Component;

use crate::features::cards::{CardIcon, CardResource, CardTrait};

#[derive(Component)]
pub struct UpgradeCard {
    id: String,
    name: String,
    unique: bool,
    cost: u8,
    res: Vec<CardResource>,
    card_icons: Vec<CardIcon>,
    traits: Vec<CardTrait>,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
    card_amount_max: u8,
}
