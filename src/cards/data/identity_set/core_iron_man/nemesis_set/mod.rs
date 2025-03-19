use crate::features::cards::Card;

mod electric_whip_attack;
mod electromagnetic_backlash;
mod imminent_overload;
mod whiplash;

pub fn get_nemesis_set() -> Vec<Card> {
    vec![
        imminent_overload::get_imminent_overload(),
        whiplash::get_whiplash(),
        electric_whip_attack::get_whiplash(),
        electric_whip_attack::get_whiplash(),
        electromagnetic_backlash::get_electromagnetic_backlash(),
    ]
}
