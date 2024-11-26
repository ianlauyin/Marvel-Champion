use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, CardTrait, SupportCard},
};

pub fn get_avengers_mansion() -> Card {
    Card::Support(SupportCard {
        id: "core_091",
        name: "Avengers Mansion",
        unique: false,
        aspect: CardAspect::Basic,
        cost: 4,
        res: vec![CardResource::Mental],
        card_icons: vec![],
        traits: vec![CardTrait::Avenger, CardTrait::Location],
        description: "Max 1 per player. Action: Exhaust Avengers Mansion -> choose a player. That player draws 1 card.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core/core_091.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 1,
    })
}
