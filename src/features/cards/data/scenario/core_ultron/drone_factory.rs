use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_drone_factory() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_148",
        name: "Drone Factory",
        description: "When Revealed: Each player puts the top card of their deck into play facedown, engaged with them as a Drone minion. Place 1 threat here for each Drone minion in play.",
        abilities: vec![],
        traits: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_148.png",
        boost: 2,
        initial_threat: Count::Constant(4),
        card_icons: vec![CardIcon::Acceleration],
        boost_effect:None,
    })
}
