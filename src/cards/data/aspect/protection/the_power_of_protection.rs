use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_079",
        name: "The Power of Protection",
        card_amount_max: 2,
        belongs: Belong::Aspect(Aspect::Protection).into(),
        is_vertical: true,
    }
}
