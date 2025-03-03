use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreCaptainMarvel,
};
pub fn get_photonic_blast() -> Card {
    Card::Event(EventCard {
        id: "core_013",
        name: "Photonic Blast",
        aspect: CardAspect::IdentitySpecific(CoreCaptainMarvel),
        cost: 3,
        res: vec![CardResource::Physical],
        keywords: vec![],
        traits: vec![CardTrait::Attack,CardTrait::Superpower],
        description: "Hero Action (attack): Deal 5 damage to an enemy. If you paid for this card using a Energy resource, draw 1 card.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_captain_marvel/core_013.png",
        card_amount_max: 3,
    })
}
