use bevy::prelude::Component;

use crate::features::cards::CardIcon;

#[derive(Component)]
pub struct SideSchemeCard {
    id: String,
    name: String,
    boost: u8,
    target_threat: u8,
    initial_threat: u8,
    card_icons: Vec<CardIcon>,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
}
