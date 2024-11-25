use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{
        Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreSpiderMan,
    },
};
pub fn get_swinging_web_kick() -> Card {
    Card::Event(EventCard {
        id: "core_005",
        name: "Swinging Web Kick",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        cost: 3,
        res: vec![CardResource::Mental],
        traits: vec![CardTrait::Aerial, CardTrait::Attack, CardTrait::Superpower],
        description: "Hero Action (attack): Deal 8 damage to an enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_005.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 3,
    })
}
