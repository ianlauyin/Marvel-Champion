use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_133",
        name: "Masters of Mayhem",
        card_amount_max: 2,
        belongs: Belong::ModularSet(ModularSet::MastersOfEvil).into(),
        is_vertical: true,
    }
}
