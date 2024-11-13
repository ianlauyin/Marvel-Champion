use bevy::prelude::Component;

use crate::features::cards::{CardIcon, CardTrait};

#[derive(Component)]
pub struct AttachmentCard {
    id: String,
    name: String,
    unique: bool,
    boost: u8,
    card_icons: Vec<CardIcon>,
    traits: Vec<CardTrait>,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
}
