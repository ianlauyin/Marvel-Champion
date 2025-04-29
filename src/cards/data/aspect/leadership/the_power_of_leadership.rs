use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_072",
        name: "The Power of Leadership",
        card_amount_max: 2,
        belongs: Belong::Aspect(Aspect::Leadership).into(),
        is_vertical: true,
    }
}
