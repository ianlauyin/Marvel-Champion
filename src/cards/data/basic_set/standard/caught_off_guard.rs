use crate::features::cards::{Card, TreacheryCard};

pub fn get_caught_off_guard() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_188",
        name: "Caught Off Guard",
        description: "When Revealed: Discard an upgrade or support you control. If no cards were discarded this way, this card gains surge.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/standard/core_188.png",
        boost: 1,
        traits: vec![],
        keywords: vec![],
        boost_effect:None,
    })
}
