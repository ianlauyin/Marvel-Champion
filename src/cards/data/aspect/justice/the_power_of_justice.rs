use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_062",
        name: "The Power of Justice",
        card_amount_max: 2,
        belongs: Belong::Aspect(Aspect::Justice).into(),
        is_vertical: true,
    }
}
