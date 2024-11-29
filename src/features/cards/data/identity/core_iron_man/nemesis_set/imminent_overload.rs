use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_imminent_overload() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_171",
        name: "Imminent Overload",
        boost: 3,
        card_icons: vec![CardIcon::Acceleration],
        description: "When Revealed: Place an additional 1 per person threat here.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_iron_man/core_171.png",
        initial_threat: Count::Constant(3),
    })
}
