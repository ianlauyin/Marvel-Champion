use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreSheHulk,
};

pub fn get_gamma_slam() -> Card {
    Card::Event(EventCard {
        id: "core_021",
        name: "Gamma Slam",
        description: "Hero Action (attack): Deal X damage to an enemy (to a maximum of 15). X is the amount of damage you have sustained.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_021.png",
        keywords: vec![],
        traits: vec![CardTrait::Attack,CardTrait::Superpower],
        aspect: CardAspect::IdentitySpecific(CoreSheHulk),
        cost: 4,
        res: vec![CardResource::Mental],
        card_amount_max: 1,
    })
}
