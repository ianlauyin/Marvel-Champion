use crate::features::cards::{Card, TreacheryCard};

pub fn get_sonic_boom() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_123",
        name: "Sonic Boom",
        description: "When Revealed: Either spend Energy Mental Physical resources or exhaust each character you control. Boost: If this activation deals damage to you, exhaust your hero.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_klaw/core_123.png",
        boost: 0,
        traits: vec![],
        keywords: vec![],
        boost_effect:None,
    })
}
