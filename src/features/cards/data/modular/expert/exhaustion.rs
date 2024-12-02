use crate::features::cards::{Card, Keyword, TreacheryCard};

pub fn get_exhaustion() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_191",
        name: "Exhaustion",
        description: "Surge. When Revealed: Exhaust your identity card.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/expert/core_191.png",
        boost: 2,
        traits: vec![],
        keywords: vec![Keyword::Surge],
    })
}
