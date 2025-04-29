mod master_of_mayhem;
mod melter;
mod radioactive_man;
mod the_master_of_evil;
mod tiger_shark;
mod whirlwind;

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        master_of_mayhem::get_card(),
        melter::get_card(),
        radioactive_man::get_card(),
        the_master_of_evil::get_card(),
        tiger_shark::get_card(),
        whirlwind::get_card(),
    ]
}
