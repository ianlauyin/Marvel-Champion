use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_098",
        name: "Armored Rhino Suit",
        card_amount_max: 1,
        belongs: Belong::Scenario(Scenario::CoreRhino).into(),
        is_vertical: true,
    }
}
