use crate::features::cards::{Card, Keyword, TreacheryCard};

pub fn get_under_fire() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_193",
        name: "Under Fire",
        description: "Surge. When Revealed: Reveal the top card of the encounter deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/expert/core_193.png",
        boost: 3,
        traits: vec![],
        keywords: vec![Keyword::Surge],
    })
}
