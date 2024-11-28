use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{Card, CardAspect, CardResource, CardTrait, EventCard},
};
pub fn get_uppercut() -> Card {
    Card::Event(EventCard {
        id: "core_054",
        name: "Uppercut",
        aspect: CardAspect::Aggression,
        cost: 3,
        res: vec![CardResource::Physical],
        keywords: vec![],
        traits: vec![CardTrait::Attack],
        description: "Hero Action (attack): Deal 5 damage to an enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/aggression/core_054.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 3,
    })
}
