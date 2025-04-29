use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_184",
        name: "M.O.D.O.K.",
        card_amount_max: 1,
        belongs: Belong::ModularSet(ModularSet::TheDoomsdayChair).into(),
        is_vertical: true,
    }
}
