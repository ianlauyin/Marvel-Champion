use bevy::prelude::Component;

use crate::features::cards::{CardIcon, CardResource, CardTrait, Keyword};

#[derive(Component)]
pub struct AllyCard {
    id: String,
    name: String,
    unique: bool,
    cost: u8,
    res: Vec<CardResource>,
    sub_name: String,
    initial_hit_points: u8,
    keywords: Vec<Keyword>,
    traits: Vec<CardTrait>,
    card_icons: Vec<CardIcon>,
    thw: u8,
    thw_con_dmg: u8,
    atk: u8,
    atk_con_dmg: u8,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
    card_amount_max: u8,
}
