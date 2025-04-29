use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_053",
        name: "Relentless Assault",
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Aggression).into(),
        is_vertical: true,
    }
}
