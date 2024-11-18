use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{
        Card, CardAspect, CardResource, CardTrait, Identity::CoreSpiderMan, UpgradeCard,
    },
};
pub fn get_webbed_up() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_009",
        name: "Webbed Up",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        unique: false,
        cost: 4,
        res: vec![CardResource::Physical],
        card_icons: vec![],
        traits: vec![CardTrait::Condition],
        keywords: vec![],
        description: "Hero form only. Attach to an enemy. Max 1 per enemy.
        Forced Interrupt: When attached enemy would attack, discard Webbed Up instead. Then, stun that enemy.",
        abilities: vec![],
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_009.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 2,
    })
}
