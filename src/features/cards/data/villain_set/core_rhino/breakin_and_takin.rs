use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_breakin_and_takin() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_107",
        name: "Breakin' & Takin'",
        description: "When Revealed: Place an additional 1  threat here.",
        abilities: vec![],
        traits: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_107.png",
        boost: 2,
        initial_threat: Count::Constant(2),
        card_icons: vec![CardIcon::Hazard],
    })
}
