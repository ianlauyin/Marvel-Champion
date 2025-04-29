use crate::{cards::*, component::Card};

pub fn get_card<'a>() -> Card<'a> {
    Card {
        id: "core_192",
        name: "Masterplan",
        card_amount_max: 1,
        belongs: Belong::ExpertSet(ExpertSet::Expert).into(),
        is_vertical: true,
    }
}
