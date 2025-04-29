use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_060",
        name: "For Justice!",
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Justice).into(),
        is_vertical: true,
    }
}
