use crate::features::cards::{Card, Identity, ObligationCard};

pub fn get_obligation() -> Card {
    Card::Obligation(ObligationCard {
        id: "core_160",
        name: "Legal Work",
        belong: Identity::CoreSheHulk,
        instant_effect: true,
        boost: 2,
        card_icons: vec![],
        description: "Give to the Jennifer Walters player. You may flip to alter-ego form. Choose: Exhaust Jennifer Walters → remove Legal Work from the game. Give the main scheme 1 acceleration token. Discard this obligation.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_she_hulk/core_160.png",
    })
}
