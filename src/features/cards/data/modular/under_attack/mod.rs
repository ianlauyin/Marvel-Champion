use crate::features::cards::Card;

mod concussion_blasters;
mod concussive_blast;
mod under_attack;
mod vibranium_armor;

pub fn get_all() -> Vec<Card> {
    vec![
        under_attack::get_under_attack(),
        vibranium_armor::get_vibranium_armor(),
        concussion_blasters::get_concussion_blasters(),
        concussive_blast::get_concussive_blast(),
        concussive_blast::get_concussive_blast(),
    ]
}
