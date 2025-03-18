use crate::features::cards::{Card, Identity, ObligationCard};

pub fn get_obligation() -> Card {
    Card::Obligation(ObligationCard {
        id: "core_155",
        name: "Affairs of State",
        belong: Identity::CoreBlackPanther,
        instant_effect: true,
        boost: 2,
        card_icons: vec![],
        description: "Give to the T'Challa player. You may flip to alter-ego form. Choose: Exhaust T'Challa → remove Affairs of State from the game. Choose and discard a Black Panther upgrade you control. Discard this obligation.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_black_panther/core_155.png",
        boost_effect:None,
    })
}
