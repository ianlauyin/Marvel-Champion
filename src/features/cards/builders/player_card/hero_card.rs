use bevy::prelude::Component;

use crate::features::cards::{CardIcon, CardTrait, Keyword};

#[derive(Component)]
pub struct HeroCard {
    id: String,
    name: String,
    flip_target_id: Vec<String>,
    initial_hit_points: u8,
    keywords: Vec<Keyword>,
    traits: Vec<CardTrait>,
    card_icons: Vec<CardIcon>,
    thw: u8,
    atk: u8,
    def: u8,
    description: String,
    search_keywords: Vec<String>,
    hand_size: u8,
    nemensis_id: String,
    image_path: String,
}
