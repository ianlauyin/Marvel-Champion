use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_090",
        name: "Strength",
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Basic).into(),
        is_vertical: true,
    }
}
