use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_051",
        name: "Tigra",
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Aggression).into(),
        is_vertical: true,
    }
}
