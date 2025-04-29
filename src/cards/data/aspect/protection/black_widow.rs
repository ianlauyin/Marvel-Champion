use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_075",
        name: "Black Widow",
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Protection).into(),
        is_vertical: true,
    }
}
