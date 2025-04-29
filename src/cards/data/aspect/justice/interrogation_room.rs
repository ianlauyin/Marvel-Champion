use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_063",
        name: "Interrogation Room",
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Justice).into(),
        is_vertical: true,
    }
}
