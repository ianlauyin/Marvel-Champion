use bevy::prelude::Component;

use crate::features::cards::{CardIcon, CardTrait, Keyword};

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

#[derive(Component)]
pub struct EnvironmentCard {
    id: String,
    name: String,
    boost: u8,
    card_icons: Vec<CardIcon>,
    traits: Vec<CardTrait>,
    description: String,
    search_keywords: Vec<String>,
    image_path: String,
}

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
