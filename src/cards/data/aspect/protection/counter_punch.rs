use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_077",
        name: "Counter-Punch",
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Protection).into(),
        is_vertical: true,
    }
}
