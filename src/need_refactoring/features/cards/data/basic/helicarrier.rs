use crate::features::cards::{Card, CardAspect, CardResource, CardTrait, SupportCard};

pub fn get_helicarrier() -> Card {
    Card::Support(SupportCard {
        id: "core_092",
        name: "Helicarrier",
        unique: false,
        aspect: CardAspect::Basic,
        cost: 3,
        res: vec![CardResource::Physical],
        card_icons: vec![],
        keywords: vec![],
        traits: vec![CardTrait::SHIELD, CardTrait::Location],
        description: "Max 1 per player. Action: Exhaust Helicarrier -> choose a player. Reduce the resource cost of the next card that player plays this phase by 1.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core_092.png",
        card_amount_max: 1,
    })
}
