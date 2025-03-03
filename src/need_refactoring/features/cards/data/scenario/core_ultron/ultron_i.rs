use crate::features::cards::{Card, CardTrait, Count, VillainCard};

pub fn get_ultron_i() -> Card {
    Card::Villain(VillainCard {
        id: "core_134",
        name: "Ultron (I)",
        initial_hit_points: Count::PerPlayer(17),
        keywords: vec![],
        traits: vec![CardTrait::Android],
        card_icons: vec![],
        sch: 1,
        atk: 2,
        description: "Forced Response: After Ultron attacks you, choose to either place 1 threat on the main scheme or put the top card of your deck into play facedown, engaged with you as a Drone minion.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_ultron/core_134.png",
    })
}
