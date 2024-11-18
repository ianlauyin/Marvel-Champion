use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{
        Card, CardAspect, CardResource, CardTrait, Identity::CoreSpiderMan, UpgradeCard,
    },
};
pub fn get_spider_tracer() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_007",
        name: "Spider-Tracer",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        unique: false,
        cost: 1,
        res: vec![CardResource::Energy],
        card_icons: vec![],
        traits: vec![CardTrait::Item,CardTrait::Tech],
        keywords: vec![],
        description: "Attach to a minion. Forced Interrupt: When attached minion is defeated, remove 3 threat from a scheme.",
        abilities: vec![],
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_007.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 2,
    })
}
