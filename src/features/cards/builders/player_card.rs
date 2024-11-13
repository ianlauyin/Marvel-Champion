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

#[derive(Component)]
pub struct AlterEgoCard {
    id: String,
    name: String,
    flip_target_id: Vec<String>,
    initial_hit_points: u8,
    keywords: Vec<Keyword>,
    traits: Vec<CardTrait>,
    card_icons: Vec<CardIcon>,
    rec: u8,
    description: String,
    search_keywords: Vec<String>,
    hand_size: u8,
    nemensis_id: String,
    image_path: String,
}

#[derive(Component)]
pub struct EventCard {
    id: String,
    name: String,
    cost: u8,
    res: Vec<CardResource>,
    traits: Vec<CardTrait>,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
    card_amount_max: u8,
}

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

#[derive(Component)]
pub struct ResourceCard {
    id: String,
    name: String,
    res: Vec<CardResource>,
    traits: Vec<CardTrait>,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
    card_amount_max: u8,
}

#[derive(Component)]
pub struct SupportCard {
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
