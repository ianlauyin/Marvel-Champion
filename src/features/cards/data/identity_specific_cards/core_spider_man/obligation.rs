use crate::{
    constants::ENCOUNTER_CARD_BACK_ASSET,
    features::cards::{Card, ObligationCard},
};
pub fn get_obligation() -> Card {
    Card::Obligation(ObligationCard {
        id: "core_165",
        name: "Eviction Notice",
        belong_id: vec!["core_1a", "core_1b"],
        instant_effect: true,
        boost: 2,
        card_icons: vec![],
        description: "Give to the Peter Parker player. You may flip to alter-ego form. Choose: Exhaust Peter Parker -> remove Eviction Notice from the game. Discard 1 card at random from your hand. This card gains surge. Discard this obligation.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_165.png",
        card_back_image_path: ENCOUNTER_CARD_BACK_ASSET.path,
    })
}
