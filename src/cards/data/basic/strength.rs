use crate::features::cards::{Card, CardAspect, CardResource, ResourceCard};

pub fn get_strength() -> Card {
    Card::Resource(ResourceCard {
        id: "core_090",
        name: "Strength",
        aspect: CardAspect::Basic,
        res: vec![CardResource::Physical, CardResource::Physical],
        traits: vec![],
        description: "Max 1 per deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core_090.png",
        card_amount_max: 1,
    })
}
