use crate::features::cards::Card;

mod advanced_ultron_drone;
mod android_efficiency_a;
mod android_efficiency_b;
mod android_efficiency_c;
mod assault_on_norad_2a;
mod assault_on_norad_2b;
mod countdown_to_oblivion_3a;
mod countdown_to_oblivion_3b;
mod drone_factory;
mod invasive_ai;
mod program_transmitter;
mod rage_of_ultron;
mod repair_sequence;
mod swarm_attack;
mod the_crimson_cowl_1a;
mod the_crimson_cowl_1b;
mod ultron_drones;
mod ultron_i;
mod ultron_ii;
mod ultron_iii;
mod ultrons_imperative;
mod upgraded_drones;

pub fn get_standard_villain_cards() -> Vec<Card> {
    vec![ultron_i::get_ultron_i(), ultron_ii::get_ultron_ii()]
}

pub fn get_expert_villain_cards() -> Vec<Card> {
    vec![ultron_ii::get_ultron_ii(), ultron_iii::get_ultron_iii()]
}

pub fn get_main_scheme_cards() -> Vec<Card> {
    vec![
        the_crimson_cowl_1a::get_the_crimson_cowl_1a(),
        the_crimson_cowl_1b::get_the_crimson_cowl_1b(),
        assault_on_norad_2a::get_assault_on_norad_2a(),
        assault_on_norad_2b::get_assault_on_norad_2b(),
        countdown_to_oblivion_3a::get_countdown_to_oblivion_3a(),
        countdown_to_oblivion_3b::get_countdown_to_oblivion_3b(),
    ]
}

pub fn get_encounter_cards() -> Vec<Card> {
    vec![
        ultron_drones::get_ultron_drones(),
        program_transmitter::get_program_transmitter(),
        upgraded_drones::get_upgraded_drones(),
        upgraded_drones::get_upgraded_drones(),
        advanced_ultron_drone::get_advanced_ultron_drone(),
        advanced_ultron_drone::get_advanced_ultron_drone(),
        advanced_ultron_drone::get_advanced_ultron_drone(),
        android_efficiency_a::get_android_efficiency_a(),
        android_efficiency_b::get_android_efficiency_b(),
        android_efficiency_c::get_android_efficiency_c(),
        rage_of_ultron::get_rage_of_ultron(),
        rage_of_ultron::get_rage_of_ultron(),
        repair_sequence::get_repair_sequence(),
        repair_sequence::get_repair_sequence(),
        swarm_attack::get_swarm_attack(),
        swarm_attack::get_swarm_attack(),
        drone_factory::get_drone_factory(),
        invasive_ai::get_invasive_ai(),
        ultrons_imperative::get_ultrons_imperative(),
    ]
}

pub fn get_all() -> Vec<Card> {
    [
        vec![
            ultron_i::get_ultron_i(),
            ultron_ii::get_ultron_ii(),
            ultron_iii::get_ultron_iii(),
        ],
        get_main_scheme_cards(),
        get_encounter_cards(),
    ]
    .concat()
}
