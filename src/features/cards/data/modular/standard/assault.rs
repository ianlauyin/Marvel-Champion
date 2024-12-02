use crate::features::cards::{Card, TreacheryCard};

pub fn get_assault() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_187",
        name: "Assault",
        description: "When Revealed (Alter-Ego): This card gains surge. When Revealed (Hero): The villain attacks you.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/standard/core_187.png",
        boost: 0,
        traits: vec![],
        keywords: vec![],
    })
}
