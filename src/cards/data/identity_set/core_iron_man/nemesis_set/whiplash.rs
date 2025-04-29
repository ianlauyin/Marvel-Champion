use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_172",
        name: "Whiplash",
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreIronMan).into(),
        is_vertical: true,
    }
}
