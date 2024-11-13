use bevy::prelude::Component;

use crate::features::cards::CardTrait;

#[derive(Component)]
pub struct TreacheryCard {
    id: String,
    name: String,
    traits: Vec<CardTrait>,
    boost: u8,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
}
