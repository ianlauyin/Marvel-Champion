use crate::features::cards::Card;

mod armored_guard;
mod defence_network;
mod illegal_arms_factory;
mod klaw_i;
mod klaw_ii;
mod klaw_iii;
mod klaws_vengenace;
mod secret_rendezvous_2a;
mod secret_rendezvous_2b;
mod solid_sound_body;
mod sonic_boom;
mod sonic_converter;
mod sound_manipulation;
mod the_immortal_klaw;
mod underground_distribution_1a;
mod underground_distribution_1b;
mod weapon_runner;

pub fn get_standard_villain_cards() -> Vec<Card> {
    vec![klaw_i::get_klaw_i(), klaw_ii::get_klaw_ii()]
}

pub fn get_expert_villain_cards() -> Vec<Card> {
    vec![klaw_ii::get_klaw_ii(), klaw_iii::get_klaw_iii()]
}

pub fn get_main_scheme_cards() -> Vec<Card> {
    vec![
        underground_distribution_1a::get_underground_distribution_1a(),
        underground_distribution_1b::get_underground_distribution_1b(),
        secret_rendezvous_2a::get_secret_rendezvous_2a(),
        secret_rendezvous_2b::get_secret_rendezvous_2b(),
    ]
}

pub fn get_encounter_cards() -> Vec<Card> {
    vec![
        sonic_converter::get_sonic_converter(),
        solid_sound_body::get_solid_sound_body(),
        armored_guard::get_armored_guard(),
        armored_guard::get_armored_guard(),
        armored_guard::get_armored_guard(),
        weapon_runner::get_weapon_runner(),
        weapon_runner::get_weapon_runner(),
        klaws_vengenace::get_klaws_vengenace(),
        klaws_vengenace::get_klaws_vengenace(),
        sonic_boom::get_sonic_boom(),
        sonic_boom::get_sonic_boom(),
        sound_manipulation::get_sound_manipulation(),
        sound_manipulation::get_sound_manipulation(),
        defence_network::get_defence_network(),
        illegal_arms_factory::get_illegal_arms_factory(),
        the_immortal_klaw::get_the_immortal_klaw(),
    ]
}

pub fn get_all() -> Vec<Card> {
    [
        vec![
            klaw_i::get_klaw_i(),
            klaw_ii::get_klaw_ii(),
            klaw_iii::get_klaw_iii(),
        ],
        get_main_scheme_cards(),
        get_encounter_cards(),
    ]
    .concat()
}
