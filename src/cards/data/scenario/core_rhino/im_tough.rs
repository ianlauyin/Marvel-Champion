use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_105",
        name: "\"I'm Tough\"",
        card_amount_max: 2,
        belongs: Belong::Scenario(Scenario::CoreRhino).into(),
        is_vertical: true,
    }
}
