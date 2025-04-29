use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_113",
        name: "Klaw (I)",
        card_amount_max: 1,
        belongs: Belong::Scenario(Scenario::CoreKlaw).into(),
        is_vertical: true,
    }
}
