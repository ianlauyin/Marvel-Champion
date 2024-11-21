use crate::features::cards::Card;

mod alter_ego;
mod aunt_may;
mod backflip;
mod black_cat;
mod enhanced_spider_sense;
mod hero;
mod nemesis_set;
mod spider_tracer;
mod swinging_web_kick;
mod web_shooter;
mod webbed_up;

pub use nemesis_set::get_nemesis_set;

pub fn get_player_cards() -> Vec<Card> {
    vec![
        alter_ego::get_alter_ego(),
        hero::get_hero(),
        aunt_may::get_aunt_may(),
        backflip::get_backflip(),
        black_cat::get_black_cat(),
        enhanced_spider_sense::get_enhanced_spider_sense(),
        spider_tracer::get_spider_tracer(),
        swinging_web_kick::get_swinging_web_kick(),
        web_shooter::get_web_shooter(),
        webbed_up::get_webbed_up(),
    ]
}

pub fn get_all(player_number: u8) -> Vec<Card> {
    let mut cards = get_player_cards();
    cards.append(&mut nemesis_set::get_nemesis_set(player_number));
    cards
}
