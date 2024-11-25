use crate::features::cards::{CardAbility, CardIcon, CardTrait, Count, Keyword};

#[derive(Clone)]
pub struct AttachmentCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub unique: bool,
    pub boost: u8,
    pub card_icons: Vec<CardIcon>,
    pub traits: Vec<CardTrait>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}

#[derive(Clone)]
pub struct EnvironmentCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub boost: u8,
    pub card_icons: Vec<CardIcon>,
    pub traits: Vec<CardTrait>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}

#[derive(Clone)]
pub struct MainSchemeCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub next_stage_id: Option<&'a str>,
    pub target_threat: u8,
    pub initial_threat: u8,
    pub card_icons: Vec<CardIcon>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}

#[derive(Clone)]
pub struct MinionCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub unique: bool,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub sch: u8,
    pub atk: u8,
    pub boost: u8,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}

#[derive(Clone)]
pub struct ObligationCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub belong_id: &'a str,
    pub instant_effect: bool,
    pub boost: u8,
    pub card_icons: Vec<CardIcon>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}

#[derive(Clone)]
pub struct SideSchemeCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub boost: u8,
    pub initial_threat: Count,
    pub card_icons: Vec<CardIcon>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}

#[derive(Clone)]
pub struct TreacheryCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub traits: Vec<CardTrait>,
    pub boost: u8,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}

#[derive(Clone)]
pub struct VillainCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub sch: u8,
    pub atk: u8,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub hand_size: u8,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}
