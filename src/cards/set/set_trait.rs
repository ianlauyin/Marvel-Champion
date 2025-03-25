use bevy::ecs::{entity::Entity, system::Commands};

use crate::component::card::CardBasic;

pub trait SetTrait {
    fn get_boxed_all() -> Vec<Box<dyn SetTrait>>
    where
        Self: Sized;
    fn get_card_infos(&self) -> Vec<CardBasic>;
    fn get_cards(&self) -> Vec<(CardBasic<'static>, fn(Commands) -> Entity)>;
    fn to_str(&self) -> &str;
    fn get_key(&self) -> &str;
    fn get_thumbnail_key(&self) -> Option<String>;
}
