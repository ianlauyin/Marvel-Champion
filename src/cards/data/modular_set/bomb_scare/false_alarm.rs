use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_112",
        name: "False Alarm",
        card_amount_max: 2,
        belongs: Belong::ModularSet(ModularSet::BombScare).into(),
        is_vertical: true,
    }
}
