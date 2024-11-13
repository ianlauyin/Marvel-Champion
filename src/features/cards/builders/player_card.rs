use bevy::prelude::Component;

use crate::features::cards::{CardIcon, CardResource, CardTrait, Keyword};

#[derive(Component)]
pub struct AllyCard {
    pub id: String,
    pub name: String,
    pub unique: bool,
    pub cost: u8,
    pub res: Vec<CardResource>,
    pub sub_name: String,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub thw: u8,
    pub thw_con_dmg: u8,
    pub atk: u8,
    pub atk_con_dmg: u8,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
    pub card_amount_max: u8,
}

#[derive(Component)]
pub struct AlterEgoCard {
    pub id: String,
    pub name: String,
    pub flip_target_id: Vec<String>,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub rec: u8,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub hand_size: u8,
    pub nemensis_id: String,
    pub image_path: String,
}

#[derive(Component)]
pub struct EventCard {
    pub id: String,
    pub name: String,
    pub cost: u8,
    pub res: Vec<CardResource>,
    pub traits: Vec<CardTrait>,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
    pub card_amount_max: u8,
}

#[derive(Component)]
pub struct HeroCard {
    pub id: String,
    pub name: String,
    pub flip_target_id: Vec<String>,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub thw: u8,
    pub atk: u8,
    pub def: u8,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub hand_size: u8,
    pub nemensis_id: String,
    pub image_path: String,
}

#[derive(Component)]
pub struct ResourceCard {
    pub id: String,
    pub name: String,
    pub res: Vec<CardResource>,
    pub traits: Vec<CardTrait>,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
    pub card_amount_max: u8,
}

#[derive(Component)]
pub struct SupportCard {
    pub id: String,
    pub name: String,
    pub unique: bool,
    pub cost: u8,
    pub res: Vec<CardResource>,
    pub card_icons: Vec<CardIcon>,
    pub traits: Vec<CardTrait>,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
    pub card_amount_max: u8,
}

#[derive(Component)]
pub struct UpgradeCard {
    pub id: String,
    pub name: String,
    pub unique: bool,
    pub cost: u8,
    pub res: Vec<CardResource>,
    pub card_icons: Vec<CardIcon>,
    pub traits: Vec<CardTrait>,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
    pub card_amount_max: u8,
}
