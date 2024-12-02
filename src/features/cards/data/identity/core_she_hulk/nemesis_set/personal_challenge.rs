use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_personal_challenge() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_161",
        name: "Personal Challenge",
        boost: 3,
        traits: vec![],
        card_icons: vec![CardIcon::Crisis],
        description: "When Revealed: Place an additional 1  threat here.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_she_hulk/core_161.png",
        initial_threat: Count::Constant(3),
    })
}
