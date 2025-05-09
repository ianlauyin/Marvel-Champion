mod alter_ego;
mod aunt_may;
mod backflip;
mod black_cat;
mod enhanced_spider_sense;
mod hero;
mod nemesis_set;
mod obligation;
mod spider_tracer;
mod swinging_web_kick;
mod web_shooter;
mod webbed_up;

use crate::component::Card;

pub fn get_identity_cards<'a>() -> Vec<Card<'a>> {
    vec![alter_ego::get_card(), hero::get_card()]
}

pub fn get_deck_cards<'a>() -> Vec<Card<'a>> {
    vec![
        aunt_may::get_card(),
        backflip::get_card(),
        black_cat::get_card(),
        enhanced_spider_sense::get_card(),
        spider_tracer::get_card(),
        swinging_web_kick::get_card(),
        web_shooter::get_card(),
        webbed_up::get_card(),
    ]
}

pub fn get_obligation_card<'a>() -> Card<'a> {
    obligation::get_card()
}

pub fn get_out_of_play_cards<'a>() -> Vec<Card<'a>> {
    vec![
        nemesis_set::highway_robbery::get_card(),
        nemesis_set::sweeping_swoop::get_card(),
        nemesis_set::vulture::get_card(),
        nemesis_set::vultures_plan::get_card(),
    ]
}
