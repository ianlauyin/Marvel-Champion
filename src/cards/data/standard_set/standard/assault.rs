use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_187",
        name: "Assault",
        card_amount_max: 2,
        belongs: Belong::StandardSet(StandardSet::Standard).into(),
        is_vertical: true,
    }
}
