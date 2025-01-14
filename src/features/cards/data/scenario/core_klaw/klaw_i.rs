use crate::features::cards::{Card, CardTrait, Count, VillainCard};

pub fn get_klaw_i() -> Card {
    Card::Villain(VillainCard {
        id: "core_113",
        name: "Klaw (I)",
        initial_hit_points: Count::PerPlayer(12),
        keywords: vec![],
        traits: vec![CardTrait::MasterOfEvil],
        card_icons: vec![],
        sch: 2,
        atk: 0,
        description: " Forced Interrupt: When Klaw attacks, give him 1 additional boost card for this activation.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_klaw/core_113.png",
    })
}
