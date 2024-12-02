use crate::features::cards::{Card, TreacheryCard};

pub fn get_electromagnetic_backlash() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_174",
        name: "Electromagnetic Backlash",
        boost: 2,
        description: "When Revealed: Each player discards the top 5 cards of their deck. For each printed Energy resource a player discards this way, that player takes 1 damage.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_iron_man/core_174.png",
        traits: vec![],
        keywords: vec![],
        boost_effect:None,
    })
}
