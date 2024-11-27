use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, CardTrait, SupportCard},
};
pub fn get_interrogation_room() -> Card {
    Card::Support(SupportCard {
        id: "core_063",
        name: "Interrogation Room",
        unique: false,
        cost: 1,
        card_icons: vec![],
        aspect: CardAspect::Justice,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Location],keywords:vec![],
        description: "Max 1 per player. Response: After you defeat a minion, exhaust Interrogation Room -> remove 1 threat from a scheme.",
        abilities: vec![],
        card_image_path: "embedded://cards/justice/core_063.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 1,
    })
}
