use crate::features::cards::{Card, EnvironmentCard};

pub fn get_ultron_drones() -> Card {
    Card::Environment(EnvironmentCard {
        id: "core_140",
        name: "Ultron Drones",
        traits: vec![],
        card_icons: vec![],
        description: "Each facedown Drone minion engaged with a player has a base SCH of 1, a base ATK of 1, and a base hit points of 1. Forced Response: After a facedown Drone minion is defeated, place that card in it's owners discard pile.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_140.png",
        boost: 0,
        boost_effect:None,
    })
}
