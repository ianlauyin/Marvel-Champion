use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_ultrons_imperative() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_150",
        name: "Ultron's Imperative",
        description: "When Revealed: The first player puts the top 2 cards of their deck into play facedown, engaged with them as Drone minions.",
        abilities: vec![],
        traits: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_150.png",
        boost: 3,
        initial_threat: Count::PerPlayer(2),
        card_icons: vec![CardIcon::Hazard],
        boost_effect:None,
    })
}
