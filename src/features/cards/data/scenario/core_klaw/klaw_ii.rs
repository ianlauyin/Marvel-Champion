use crate::features::cards::{Card, CardTrait, Count, VillainCard};

pub fn get_klaw_ii() -> Card {
    Card::Villain(VillainCard {
        id: "core_114",
        name: "Klaw (II)",
        initial_hit_points: Count::PerPlayer(18),
        keywords: vec![],
        traits: vec![CardTrait::MasterOfEvil],
        card_icons: vec![],
        sch: 2,
        atk: 1,
        description: "When Revealed: Search the encounter deck and discard pile for The \"Immortal\" Klaw and reveal it. Shuffle the encounter deck. Forced Interrupt: When Klaw attacks, give him 1 additional boost card for this activation.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_klaw/core_114.png",
    })
}
