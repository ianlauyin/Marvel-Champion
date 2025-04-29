use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_188",
        name: "Caught Off Guard",
        card_amount_max: 1,
        belongs: Belong::StandardSet(StandardSet::Standard).into(),
        is_vertical: true,
    }
}
