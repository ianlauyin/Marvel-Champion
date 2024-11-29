use crate::features::cards::{Card, TreacheryCard};

pub fn get_klaws_vengenace() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_122",
        name: "Klaw's Vengeance",
        description: "When Revealed (Alter-Ego): Discard 1 card at random from your hand. When Revealed (Hero): Klaw attacks you. If this attack deals damage, place 1 threat on the main scheme.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_klaw/core_122.png",
        boost: 1,
        traits: vec![],
        keyword: vec![],
    })
}
