use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_093",
        name: "Tenacity",
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Basic).into(),
        is_vertical: true,
    }
}
