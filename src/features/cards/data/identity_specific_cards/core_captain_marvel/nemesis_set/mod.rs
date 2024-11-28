use crate::features::cards::Card;

mod kree_manipulator;
mod the_psyche_magnitron;
mod yon_rogg;
mod yon_roggs_treason;

pub fn get_nemesis_set() -> Vec<Card> {
    vec![
        the_psyche_magnitron::get_the_psyche_magnitron(),
        yon_rogg::get_yogg_rogg(),
        kree_manipulator::get_kree_manipulator(),
        yon_roggs_treason::get_yon_roggs_treason(),
    ]
}
