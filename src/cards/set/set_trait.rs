use bevy::color::Color;

use crate::component::Card;

pub trait SetTrait: Sync + Send {
    fn get_boxed_all() -> Vec<Box<dyn SetTrait>>
    where
        Self: Sized;
    fn get_cards<'a>(&self) -> Vec<Card<'a>>;
    fn to_str(&self) -> &str;
    fn get_key(&self) -> &str;
    fn get_thumbnail_key(&self) -> Option<String>;
    fn get_color(&self) -> Option<Color>;
}
