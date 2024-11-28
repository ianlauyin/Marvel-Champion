use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{Card, CardAspect, CardResource, CardTrait, EventCard},
};
pub fn get_chase_them_down() -> Card {
    Card::Event(EventCard {
        id: "core_052",
        name: "Chase Them Down",
        aspect: CardAspect::Aggression,
        cost: 0,
        res: vec![CardResource::Mental],
        keywords: vec![],
        traits: vec![CardTrait::Thwart],
        description: "Response (thwart): After your hero attacks and defeats an enemy, remove 2 threat from a scheme.",
        abilities: vec![],
        card_image_path: "embedded://cards/aggression/core_052.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 3,
    })
}
