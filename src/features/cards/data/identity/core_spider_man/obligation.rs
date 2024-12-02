use crate::features::cards::{Card, Identity, ObligationCard};
pub fn get_obligation() -> Card {
    Card::Obligation(ObligationCard {
        id: "core_165",
        name: "Eviction Notice",
        belong: Identity::CoreSpiderMan,
        instant_effect: true,
        boost: 2,
        card_icons: vec![],
        description: "Give to the Peter Parker player. You may flip to alter-ego form. Choose: Exhaust Peter Parker -> remove Eviction Notice from the game. Discard 1 card at random from your hand. This card gains surge. Discard this obligation.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_spider_man/core_165.png",
        boost_effect:None,
    })
}
