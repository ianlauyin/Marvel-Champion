use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_174",
        name: "Electromagnetic Backlash",
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreIronMan).into(),
        is_vertical: true,
    }
}
