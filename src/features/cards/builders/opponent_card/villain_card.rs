use bevy::prelude::Component;

use crate::features::cards::{CardIcon, CardTrait, Keyword};

#[derive(Component)]
pub struct VillainCard {
    id: String,
    name: String,
    initial_hit_points: u8,
    keywords: Vec<Keyword>,
    traits: Vec<CardTrait>,
    card_icons: Vec<CardIcon>,
    sch: u8,
    atk: u8,
    description: String,
    search_keywords: Vec<String>,
    hand_size: u8,
    image_path: String,
}
