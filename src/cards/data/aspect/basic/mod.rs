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

use crate::component::Card;

pub fn get_cards<'a>() -> Vec<Card<'a>> {
    vec![
        avengers_mansion::get_card(),
        emergency::get_card(),
        energy::get_card(),
        first_aid::get_card(),
        genius::get_card(),
        haymaker::get_card(),
        helicarrier::get_card(),
        mockingbird::get_card(),
        nick_fury::get_card(),
        strength::get_card(),
        tenacity::get_card(),
    ]
}
