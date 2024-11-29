use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_illegal_arms_factory() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_126",
        name: "Illegal Arms Factory",
        description: "When Revealed: Place an additional 1 per player threat here.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_klaw/core_126.png",
        boost: 2,
        initial_threat: Count::Constant(3),
        card_icons: vec![CardIcon::Hazard],
    })
}
