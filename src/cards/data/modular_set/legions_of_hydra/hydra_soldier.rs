use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_182",
        name: "Hydra Soldier",
        card_amount_max: 3,
        belongs: Belong::ModularSet(ModularSet::LegionsOfHydra).into(),
        is_vertical: true,
    }
}
