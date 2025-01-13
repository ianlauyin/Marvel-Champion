use crate::features::cards::Card;

mod alter_ego;
mod arc_reactor;
mod hero;
mod mark_v_armor;
mod mark_v_helmet;
mod nemesis_set;
mod obligation;
mod pepper_potts;
mod powered_gauntlets;
mod repulsor_blast;
mod rocket_boots;
mod stark_tower;
mod supersonic_punch;
mod war_machine;

pub fn get_player_cards() -> Vec<Card> {
    vec![
        war_machine::get_war_machine(),
        repulsor_blast::get_repulsor_blast(),
        repulsor_blast::get_repulsor_blast(),
        repulsor_blast::get_repulsor_blast(),
        supersonic_punch::get_supersonic_punch(),
        supersonic_punch::get_supersonic_punch(),
        pepper_potts::get_pepper_potts(),
        stark_tower::get_stark_tower(),
        arc_reactor::get_arc_reactor(),
        mark_v_armor::get_mark_v_armor(),
        mark_v_helmet::get_mark_v_helmet(),
        powered_gauntlets::get_powered_gauntlets(),
        powered_gauntlets::get_powered_gauntlets(),
        rocket_boots::get_rocket_boots(),
        rocket_boots::get_rocket_boots(),
    ]
}

pub fn get_hero() -> Vec<Card> {
    vec![hero::get_hero()]
}

pub fn get_identity_cards() -> Vec<Card> {
    [vec![alter_ego::get_alter_ego()], get_hero()].concat()
}

pub fn get_obligation() -> Card {
    obligation::get_obligation()
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

pub use alter_ego::get_alter_ego;
pub use nemesis_set::get_nemesis_set;
