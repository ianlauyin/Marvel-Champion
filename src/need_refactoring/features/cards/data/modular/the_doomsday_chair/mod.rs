use crate::features::cards::Card;

mod biomechanical_upgrades;
mod modok;
mod the_doomsday_chair;

pub fn get_all() -> Vec<Card> {
    vec![
        the_doomsday_chair::get_the_doomsday_chair(),
        the_doomsday_chair::get_the_doomsday_chair(),
        modok::get_modok(),
        biomechanical_upgrades::get_biomechanical_upgrades(),
        biomechanical_upgrades::get_biomechanical_upgrades(),
        biomechanical_upgrades::get_biomechanical_upgrades(),
    ]
}
