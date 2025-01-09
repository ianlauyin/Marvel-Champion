use crate::features::cards::Card;

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

pub fn get_player_cards() -> Vec<Card> {
    vec![
        black_cat::get_black_cat(),
        backflip::get_backflip(),
        backflip::get_backflip(),
        enhanced_spider_sense::get_enhanced_spider_sense(),
        enhanced_spider_sense::get_enhanced_spider_sense(),
        swinging_web_kick::get_swinging_web_kick(),
        swinging_web_kick::get_swinging_web_kick(),
        swinging_web_kick::get_swinging_web_kick(),
        aunt_may::get_aunt_may(),
        spider_tracer::get_spider_tracer(),
        spider_tracer::get_spider_tracer(),
        web_shooter::get_web_shooter(),
        web_shooter::get_web_shooter(),
        webbed_up::get_webbed_up(),
        webbed_up::get_webbed_up(),
    ]
}

pub fn get_identity_cards() -> Vec<Card> {
    vec![alter_ego::get_alter_ego(), hero::get_hero()]
}

pub fn get_all() -> Vec<Card> {
    [
        get_identity_cards(),
        get_player_cards(),
        vec![get_obligation()],
        nemesis_set::get_nemesis_set(),
    ]
    .concat()
}

pub fn get_obligation() -> Card {
    obligation::get_obligation()
}

pub use nemesis_set::get_nemesis_set;
