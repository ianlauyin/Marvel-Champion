use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{
        Card, CardAspect, CardResource, CardTrait, Identity::CoreSpiderMan, SupportCard,
    },
};

pub fn get_aunt_may() -> Card {
    Card::Support(SupportCard {
        id: "core_006",
        name: "Aunt May",
        unique: true,
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        cost: 1,
        res: vec![CardResource::Energy],
        card_icons: vec![],
        traits: vec![CardTrait::Persona],
        keywords: vec![],
        description: "Alter-Ego Action: Exhaust Aunt May -> heal 4 damage from Peter Parker.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_006.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 1,
    })
}
