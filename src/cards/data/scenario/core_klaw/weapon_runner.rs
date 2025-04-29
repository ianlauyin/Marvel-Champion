use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_121",
        name: "Weapons Runner",
        card_amount_max: 2,
        belongs: Belong::Scenario(Scenario::CoreKlaw).into(),
        is_vertical: true,
    }
}
