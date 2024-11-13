use bevy::prelude::Component;

use crate::features::cards::{CardIcon, CardTrait, Keyword};

#[derive(Component)]
pub struct MinionCard {
    id: String,
    name: String,
    unique: bool,
    initial_hit_points: u8,
    keywords: Vec<Keyword>,
    traits: Vec<CardTrait>,
    card_icons: Vec<CardIcon>,
    sch: u8,
    atk: u8,
    boost: u8,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
}
