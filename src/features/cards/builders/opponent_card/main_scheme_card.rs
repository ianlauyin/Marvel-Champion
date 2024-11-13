use bevy::prelude::Component;

use crate::features::cards::CardIcon;

#[derive(Component)]
pub struct MainSchemeCard {
    id: String,
    name: String,
    next_stage_id: Option<String>,
    target_threat: u8,
    initial_threat: u8,
    card_icons: Vec<CardIcon>,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
}
