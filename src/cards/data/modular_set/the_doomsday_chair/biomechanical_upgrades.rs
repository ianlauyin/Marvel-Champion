use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_185",
        name: "Biomechanical Upgrades",
        card_amount_max: 3,
        belongs: Belong::ModularSet(ModularSet::TheDoomsdayChair).into(),
        is_vertical: true,
    }
}
