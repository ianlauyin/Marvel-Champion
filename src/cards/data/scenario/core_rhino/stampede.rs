use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_106",
        name: "Stampede",
        card_amount_max: 3,
        belongs: Belong::Scenario(Scenario::CoreRhino).into(),
        is_vertical: true,
    }
}
