use crate::features::cards::Card;

mod alter_ego;
mod focused_rage;
mod gamma_slam;
mod ground_stomp;
mod hellcat;
mod hero;
mod legal_practice;
mod nemesis_set;
mod obligation;
mod one_two_punch;
mod split_personality;
mod superhuman_law_division;
mod superhuman_strength;

pub fn get_player_cards() -> Vec<Card> {
    vec![
        hero::get_hero(),
        alter_ego::get_alter_ego(),
        hellcat::get_hellcat(),
        gamma_slam::get_gamma_slam(),
        ground_stomp::get_ground_stomp(),
        legal_practice::get_legal_practice(),
        one_two_punch::get_one_two_punch(),
        split_personality::get_split_personality(),
        superhuman_law_division::get_superhuman_law_division(),
        focused_rage::get_focused_rage(),
        superhuman_strength::get_superhuman_strength(),
    ]
}

pub fn get_obligation() -> Card {
    obligation::get_obligation()
}

pub fn get_all() -> Vec<Card> {
    [
        get_player_cards(),
        vec![get_obligation()],
        nemesis_set::get_nemesis_set(),
    ]
    .concat()
}
