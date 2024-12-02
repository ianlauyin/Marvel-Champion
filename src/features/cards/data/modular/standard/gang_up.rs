use crate::features::cards::{Card, TreacheryCard};

pub fn get_gang_up() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_189",
        name: "Gang-Up",
        description: "When Revealed (Alter-Ego): This card gains surge. When Revealed (Hero): The villain and each minion engaged with you attacks you.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/standard/core_189.png",
        boost: 1,
        traits: vec![],
        keywords: vec![],
        boost_effect:None,
    })
}
