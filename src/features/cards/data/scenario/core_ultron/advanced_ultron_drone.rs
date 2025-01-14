use crate::features::cards::{Card, CardTrait, Keyword, MinionCard};

pub fn get_advanced_ultron_drone() -> Card {
    Card::Minion(MinionCard {
        id: "core_143",
        name: "Advanced Ultron Drone",
        traits: vec![CardTrait::Drone],
        card_icons: vec![],
        description: "Guard. Forced Interrupt: When Advanced Ultron Drone is defeated, the engaged player puts the top card of their deck into play facedown, engaged with them as a Drone minion.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_ultron/core_143.png",
        boost: 2,
        unique: false,
        initial_hit_points: 4,
        keywords: vec![Keyword::Guard],
        sch: 1,
        atk: 1,
        boost_effect:None,
    })
}
