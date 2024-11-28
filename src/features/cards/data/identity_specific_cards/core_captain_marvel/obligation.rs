use crate::features::cards::{Card, Identity, ObligationCard};

pub fn get_obligation() -> Card {
    Card::Obligation(ObligationCard {
        id: "core_175",
        name: "Family Emergency",
        belong: Identity::CoreCaptainMarvel,
        instant_effect: true,
        boost: 2,
        card_icons: vec![],
        description: "Give to the Carol Danvers player. You may flip to alter-ego form. Choose: Exhaust Carol Danvers -> remove Family Emergency from the game. You are stunned. This card gains surge. Discard this obligation.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_captain_marvel/core_175.png",
    })
}
