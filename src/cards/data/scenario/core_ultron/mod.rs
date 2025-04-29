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

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        advanced_ultron_drone::get_card(),
        android_efficiency_a::get_card(),
        android_efficiency_b::get_card(),
        android_efficiency_c::get_card(),
        assault_on_norad_2a::get_card(),
        assault_on_norad_2b::get_card(),
        countdown_to_oblivion_3a::get_card(),
        countdown_to_oblivion_3b::get_card(),
        drone_factory::get_card(),
        invasive_ai::get_card(),
        program_transmitter::get_card(),
        rage_of_ultron::get_card(),
        repair_sequence::get_card(),
        swarm_attack::get_card(),
        the_crimson_cowl_1a::get_card(),
        the_crimson_cowl_1b::get_card(),
        ultron_drones::get_card(),
        ultron_i::get_card(),
        ultron_ii::get_card(),
        ultron_iii::get_card(),
        ultrons_imperative::get_card(),
        upgraded_drones::get_card(),
    ]
}
