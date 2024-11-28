use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreSheHulk,
};

pub fn get_ground_stomp() -> Card {
    Card::Event(EventCard {
        id: "core_022",
        name: "Ground Stomp",
        description: "Hero Action: Deal 1 damage to each enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_022.png",
        keywords: vec![],
        traits: vec![CardTrait::Superpower],
        aspect: CardAspect::IdentitySpecific(CoreSheHulk),
        cost: 2,
        res: vec![CardResource::Mental],
        card_amount_max: 2,
    })
}
