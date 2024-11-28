use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{Card, CardAspect, CardResource, CardTrait, EventCard},
};
pub fn get_for_justice() -> Card {
    Card::Event(EventCard {
        id: "core_060",
        name: "For Justice!",
        cost: 2,
        keywords: vec![],
        aspect: CardAspect::Justice,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Thwart],
        description: "Hero Action (thwart): Remove 3 threat from a scheme (4 threat instead if you paid for this card using a Mental resource).",
        abilities: vec![],
        card_image_path: "embedded://cards/justice/core_060.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 3,
    })
}
