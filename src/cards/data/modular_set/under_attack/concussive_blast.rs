use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_154",
        name: "Concussive Blast",
        card_amount_max: 2,
        belongs: Belong::ModularSet(ModularSet::UnderAttack).into(),
        is_vertical: true,
    }
}
