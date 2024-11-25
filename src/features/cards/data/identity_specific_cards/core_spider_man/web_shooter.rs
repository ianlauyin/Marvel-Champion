use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{
        Card, CardAspect, CardResource, CardTrait, Counter, Identity::CoreSpiderMan, Keyword,
        UpgradeCard,
    },
};
pub fn get_web_shooter() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_008",
        name: "Web-Shooter",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        unique: false,
        cost: 1,
        res: vec![CardResource::Physical],
        card_icons: vec![],
        traits: vec![CardTrait::Item,CardTrait::Tech],
        keywords: vec![Keyword::Use(3, Counter::Web)],
        description: "Uses (3 web counters). (Enters play with 3 counters. When those are gone, discard this card)\n
        Hero Resource: Exhaust Web-Shooter and remove 1 web counter from it -> generate a  resource.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_008.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 2,
    })
}
