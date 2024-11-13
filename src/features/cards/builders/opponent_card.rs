use bevy::prelude::Component;

use crate::features::cards::{CardIcon, CardTrait, Keyword};

#[derive(Component)]
pub struct AttachmentCard {
    pub id: String,
    pub name: String,
    pub unique: bool,
    pub boost: u8,
    pub card_icons: Vec<CardIcon>,
    pub traits: Vec<CardTrait>,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
}

#[derive(Component)]
pub struct EnvironmentCard {
    pub id: String,
    pub name: String,
    pub boost: u8,
    pub card_icons: Vec<CardIcon>,
    pub traits: Vec<CardTrait>,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
}

#[derive(Component)]
pub struct MainSchemeCard {
    pub id: String,
    pub name: String,
    pub next_stage_id: Option<String>,
    pub target_threat: u8,
    pub initial_threat: u8,
    pub card_icons: Vec<CardIcon>,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
}

#[derive(Component)]
pub struct MinionCard {
    pub id: String,
    pub name: String,
    pub unique: bool,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub sch: u8,
    pub atk: u8,
    pub boost: u8,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
}

#[derive(Component)]
pub struct ObligationCard {
    pub id: String,
    pub name: String,
    pub belong_id: String,
    pub instant_effect: bool,
    pub boost: u8,
    pub card_icons: Vec<CardIcon>,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
}

#[derive(Component)]
pub struct SideSchemeCard {
    pub id: String,
    pub name: String,
    pub boost: u8,
    pub target_threat: u8,
    pub initial_threat: u8,
    pub card_icons: Vec<CardIcon>,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
}

#[derive(Component)]
pub struct TreacheryCard {
    pub id: String,
    pub name: String,
    pub traits: Vec<CardTrait>,
    pub boost: u8,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub image_path: String,
}

#[derive(Component)]
pub struct VillainCard {
    pub id: String,
    pub name: String,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub sch: u8,
    pub atk: u8,
    pub description: String,
    pub search_keywords: Vec<String>,
    pub hand_size: u8,
    pub image_path: String,
}
