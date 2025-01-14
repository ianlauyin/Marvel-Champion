use crate::features::cards::{Card, CardTrait, Count, VillainCard};

pub fn get_ultron_iii() -> Card {
    Card::Villain(VillainCard {
        id: "core_136",
        name: "Ultron (III)",
        initial_hit_points: Count::PerPlayer(27),
        keywords: vec![],
        traits: vec![CardTrait::Android],
        card_icons: vec![],
        sch: 2,
        atk: 4,
        description: "Each Drone minion gets +1 ATK and +1 hit point. Ultron cannot take damage while a Drone minion is in play. When Revealed: Search the encounter deck and discard pile for the Ultron's Imperative side scheme and reveal it. Then shuffle the encounter deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_136.png",
    })
}
