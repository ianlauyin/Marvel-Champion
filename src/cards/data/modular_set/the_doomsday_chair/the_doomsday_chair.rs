use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_183",
        name: "The Doomsday Chair",
        card_amount_max: 2,
        belongs: Belong::ModularSet(ModularSet::TheDoomsdayChair).into(),
        is_vertical: false,
    }
}
