use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_legions_of_hydra() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_180",
        name: "Legions of Hydra",
        description:
            "When Revealed: If Madame Hydra is not in play, search the encounter deck and discard pile for Madame Hydra and put her into play engaged with you, then shuffle the encounter deck. Place 2 additional threat here for each Hydra enemy in play.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/legions_of_hydra/core_180.png",
        boost: 3,
        traits: vec![],
        card_icons: vec![CardIcon::Hazard],
        initial_threat: Count::Constant(3),
    })
}
