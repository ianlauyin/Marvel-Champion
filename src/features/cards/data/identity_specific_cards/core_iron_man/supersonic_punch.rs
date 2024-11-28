use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreIronMan,
};

pub fn get_supersonic_punch() -> Card {
    Card::Event(EventCard {
        id: "core_032",
        name: "Supersonic Punch",
        description:
            "Hero Action (attack): Deal 4 damage to an enemy (8 damage instead if you have the Aerial trait).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_iron_man/core_032.png",
        traits: vec![CardTrait::Attack],
        keywords: vec![],
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        cost: 2,
        res: vec![CardResource::Energy],
        card_amount_max: 2,
    })
}
