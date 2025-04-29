use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_111",
        name: "Explosion",
        card_amount_max: 1,
        belongs: Belong::ModularSet(ModularSet::BombScare).into(),
        is_vertical: true,
    }
}
