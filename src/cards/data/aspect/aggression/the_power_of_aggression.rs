use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_055",
        name: "The Power of Aggression",
        card_amount_max: 2,
        belongs: Belong::Aspect(Aspect::Aggression).into(),
        is_vertical: true,
    }
}
