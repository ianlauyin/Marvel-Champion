mod avengers_mansion;
mod emergency;
mod energy;
mod first_aid;
mod genius;
mod haymaker;
mod helicarrier;
mod mockingbird;
mod nick_fury;
mod strength;
mod tenacity;

use crate::features::cards::Card;

pub fn get_all() -> Vec<Card> {
    vec![
        mockingbird::get_mockingbird(),
        nick_fury::get_nick_fury(),
        emergency::get_emergency(),
        first_aid::get_first_aid(),
        haymaker::get_haymaker(),
        energy::get_energy(),
        genius::get_genius(),
        strength::get_strength(),
        avengers_mansion::get_avengers_mansion(),
        helicarrier::get_helicarrier(),
        tenacity::get_tenacity(),
    ]
}
