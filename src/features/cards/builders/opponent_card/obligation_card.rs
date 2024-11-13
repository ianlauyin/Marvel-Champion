use bevy::prelude::Component;

use crate::features::cards::CardIcon;

#[derive(Component)]
pub struct ObligationCard {
    id: String,
    name: String,
    belong_id: String,
    instant_effect: bool,
    boost: u8,
    card_icons: Vec<CardIcon>,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
}
