use crate::features::cards::{CardAbility, CardIcon, CardTrait, Count, Identity, Keyword};

#[derive(Clone)]
pub struct AttachmentCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub boost: u8,
    pub card_icons: Vec<CardIcon>,
    pub traits: Vec<CardTrait>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
    pub atk_modifier: u8,
    pub sch_modifier: u8,
    pub keywords: Vec<Keyword>,
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
}

#[derive(Clone)]
pub struct MainSchemeACard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub next_stage_id: Option<&'a str>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
}

#[derive(Clone)]
pub struct MainSchemeBCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub next_stage_id: Option<&'a str>,
    pub target_threat: Count,
    pub increase_threat: Count,
    pub initial_threat: Count,
    pub card_icons: Vec<CardIcon>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
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
}

#[derive(Clone)]
pub struct ObligationCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub belong: Identity,
    pub instant_effect: bool,
    pub boost: u8,
    pub card_icons: Vec<CardIcon>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
}

#[derive(Clone)]
pub struct SideSchemeCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub boost: u8,
    pub initial_threat: Count,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
}

#[derive(Clone)]
pub struct TreacheryCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub traits: Vec<CardTrait>,
    pub boost: u8,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub keywords: Vec<Keyword>,
    pub card_image_path: &'a str,
}

#[derive(Clone)]
pub struct VillainCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub initial_hit_points: Count,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub sch: u8,
    pub atk: u8,
    pub description: &'a str,
    pub abilities: Vec<CardAbility>,
    pub card_image_path: &'a str,
}
