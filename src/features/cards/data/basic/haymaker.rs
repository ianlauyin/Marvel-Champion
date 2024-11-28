use crate::features::cards::{Card, CardAspect, CardResource, CardTrait, EventCard};

pub fn get_haymaker() -> Card {
    Card::Event(EventCard {
        id: "core_087",
        name: "Haymaker",
        aspect: CardAspect::Basic,
        cost: 2,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Attack],
        keywords: vec![],
        description: "Hero Action (attack): Deal 3 damage to an enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core_087.png",
        card_amount_max: 3,
    })
}
