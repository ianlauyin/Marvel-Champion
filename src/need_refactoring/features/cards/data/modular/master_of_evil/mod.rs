use crate::features::cards::Card;

mod master_of_mayhem;
mod melter;
mod radioactive_man;
mod the_master_of_evil;
mod tiger_shark;
mod whirlwind;

pub fn get_all() -> Vec<Card> {
    vec![
        the_master_of_evil::get_the_master_of_evil(),
        radioactive_man::get_radioactive_man(),
        whirlwind::get_whirlwind(),
        tiger_shark::get_tiger_shark(),
        melter::get_melter(),
        master_of_mayhem::get_master_of_mayhem(),
        master_of_mayhem::get_master_of_mayhem(),
    ]
}
