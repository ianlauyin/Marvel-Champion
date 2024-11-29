use crate::features::cards::{Card, TreacheryCard};

pub fn get_vultures_plans() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_169",
        name: "The Vulture's Plans",
        traits: vec![],
        boost: 2,
        keyword: vec![],
        description: "When Revealed: Discard 1 card at random from each player's hand. Place 1 threat on the main scheme for each different resource type discarded this way.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_spider_man/core_169.png",
    })
}
