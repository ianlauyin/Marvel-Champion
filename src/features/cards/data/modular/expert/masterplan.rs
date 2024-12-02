use crate::features::cards::{Card, TreacheryCard};

pub fn get_masterplan() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_192",
        name: "Masterplan",
        description: "When Revealed: Place 4 threat on each side scheme. If there are no side schemes in play, discard cards from the top of the encounter deck until a side scheme is discarded. Reveal that side scheme.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/expert/core_192.png",
        boost: 2,
        traits: vec![],
        keywords: vec![],
    })
}
