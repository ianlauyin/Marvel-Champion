use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_180",
        name: "Legions of Hydra",
        card_amount_max: 2,
        belongs: Belong::ModularSet(ModularSet::LegionsOfHydra).into(),
        is_vertical: false,
    }
}
