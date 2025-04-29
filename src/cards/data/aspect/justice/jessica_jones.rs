use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_059",
        name: "Jessica Jones",
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Justice).into(),
        is_vertical: true,
    }
}
