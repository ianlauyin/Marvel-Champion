use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_081",
        name: "Armored Vest",
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Protection).into(),
        is_vertical: true,
    }
}
