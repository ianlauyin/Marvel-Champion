use crate::features::cards::{Card, TreacheryCard};

pub fn get_advance() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_186",
        name: "Advance",
        description: "When Revealed: The villain schemes.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/standard/core_186.png",
        boost: 0,
        traits: vec![],
        keywords: vec![],
    })
}
