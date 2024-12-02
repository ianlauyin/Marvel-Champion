use crate::features::cards::{Card, CardTrait, Count, VillainCard};

pub fn get_ultron_ii() -> Card {
    Card::Villain(VillainCard {
        id: "core_135",
        name: "Ultron (II)",
        initial_hit_points: Count::PerPlayer(22),
        keywords: vec![],
        traits: vec![CardTrait::Android],
        card_icons: vec![],
        sch: 2,
        atk: 2,
        description: "Forced Interrupt: When Ultron attacks you, put the top card of your deck into play facedown, engaged with you as a Drone minion. Until the end of his attack, Ultron gets +1 ATK for each Drone minion engaged with you.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_135.png",
    })
}
