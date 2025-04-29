use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_110",
        name: "Hydra Bomber",
        card_amount_max: 2,
        belongs: Belong::ModularSet(ModularSet::BombScare).into(),
        is_vertical: true,
    }
}
