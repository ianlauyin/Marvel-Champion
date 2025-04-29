mod concussion_blasters;
mod concussive_blast;
mod under_attack;
mod vibranium_armor;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        concussion_blasters::get_card(),
        concussive_blast::get_card(),
        under_attack::get_card(),
        vibranium_armor::get_card(),
    ]
}
