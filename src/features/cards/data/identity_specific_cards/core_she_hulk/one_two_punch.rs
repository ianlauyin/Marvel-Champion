use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreSheHulk,
};

pub fn get_one_two_punch() -> Card {
    Card::Event(EventCard {
        id: "core_024",
        name: "One-Two Punch",
        description: "Response: After you make a basic attack (using your ATK), ready She-Hulk.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_024.png",
        keywords: vec![],
        traits: vec![CardTrait::Skill],
        aspect: CardAspect::IdentitySpecific(CoreSheHulk),
        cost: 1,
        res: vec![CardResource::Physical],
        card_amount_max: 3,
    })
}
