use crate::features::cards::{Card, TreacheryCard};

pub fn get_stampede() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_106",
        name: "Stampede",
        description: "When Revealed (Alter-Ego): This card gains surge. When Revealed (Hero): Rhino attacks you. If a character is damaged by this attack, that character is stunned.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_106.png",
        boost: 1,
        traits: vec![],
        keywords: vec![],
    })
}
