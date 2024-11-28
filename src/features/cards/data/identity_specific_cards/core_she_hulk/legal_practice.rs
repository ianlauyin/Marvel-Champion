use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreSheHulk,
};

pub fn get_legal_practice() -> Card {
    Card::Event(EventCard {
        id: "core_023",
        name: "Legal Practice",
        description: "Alter-Ego Action (thwart): Choose and discard up to 5 cards from your hand -> remove 1 threat from a scheme for each card discarded this way.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_023.png",
        keywords: vec![],
        traits: vec![CardTrait::Skill,CardTrait::Thwart],
        aspect: CardAspect::IdentitySpecific(CoreSheHulk),
        cost: 0,
        res: vec![CardResource::Physical],
        card_amount_max: 2,
    })
}
