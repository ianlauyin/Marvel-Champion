use crate::features::cards::{Card, Identity, ObligationCard};

pub fn get_obligation() -> Card {
    Card::Obligation(ObligationCard {
        id: "core_170",
        name: "Business Problems",
        belong: Identity::CoreIronMan,
        instant_effect: true,
        boost: 2,
        card_icons: vec![],
        description: "Give to the Tony Stark player. You may flip to alter-ego form. Choose: Exhaust Tony Stark -> remove Business Problems from the game. Exhaust each upgrade you control. Discard this obligation.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_iron_man/core_170.png",
    })
}
