use crate::features::cards::{CardAspect, CardIcon, CardResource, CardTrait, Keyword};

#[derive(Clone)]
pub struct AllyCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub sub_name: &'a str,
    pub aspect: CardAspect,
    pub unique: bool,
    pub cost: u8,
    pub res: Vec<CardResource>,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub thw: u8,
    pub thw_con_dmg: u8,
    pub atk: u8,
    pub atk_con_dmg: u8,
    pub description: &'a str,
    pub search_keywords: Vec<&'a str>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
    pub card_amount_max: u8,
}

#[derive(Clone)]
pub struct AlterEgoCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub flip_target_id: Vec<&'a str>,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub rec: u8,
    pub description: &'a str,
    pub search_keywords: Vec<&'a str>,
    pub hand_size: u8,
    pub nemesis_id: &'a str,
    pub nemesis_side_scheme_id: &'a str,
    pub nemesis_card_id: Vec<&'a str>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}

#[derive(Clone)]
pub struct EventCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub aspect: CardAspect,
    pub cost: u8,
    pub res: Vec<CardResource>,
    pub traits: Vec<CardTrait>,
    pub description: &'a str,
    pub search_keywords: Vec<&'a str>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
    pub card_amount_max: u8,
}

#[derive(Clone)]
pub struct HeroCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub flip_target_id: Vec<&'a str>,
    pub initial_hit_points: u8,
    pub keywords: Vec<Keyword>,
    pub traits: Vec<CardTrait>,
    pub card_icons: Vec<CardIcon>,
    pub thw: u8,
    pub atk: u8,
    pub def: u8,
    pub description: &'a str,
    pub search_keywords: Vec<&'a str>,
    pub hand_size: u8,
    pub nemesis_id: &'a str,
    pub nemesis_side_scheme_id: &'a str,
    pub nemesis_card_id: Vec<&'a str>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
}

#[derive(Clone)]
pub struct ResourceCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub aspect: CardAspect,
    pub res: Vec<CardResource>,
    pub traits: Vec<CardTrait>,
    pub description: &'a str,
    pub search_keywords: Vec<&'a str>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
    pub card_amount_max: u8,
}

#[derive(Clone)]
pub struct SupportCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub unique: bool,
    pub aspect: CardAspect,
    pub cost: u8,
    pub res: Vec<CardResource>,
    pub card_icons: Vec<CardIcon>,
    pub traits: Vec<CardTrait>,
    pub description: &'a str,
    pub search_keywords: Vec<&'a str>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
    pub card_amount_max: u8,
}

#[derive(Clone)]
pub struct UpgradeCard<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub aspect: CardAspect,
    pub unique: bool,
    pub cost: u8,
    pub res: Vec<CardResource>,
    pub card_icons: Vec<CardIcon>,
    pub traits: Vec<CardTrait>,
    pub keywords: Vec<Keyword>,
    pub description: &'a str,
    pub search_keywords: Vec<&'a str>,
    pub card_image_path: &'a str,
    pub card_back_image_path: &'a str,
    pub card_amount_max: u8,
}
