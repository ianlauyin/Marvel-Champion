use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_068",
        name: "Vision",
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Leadership).into(),
        is_vertical: true,
    }
}
