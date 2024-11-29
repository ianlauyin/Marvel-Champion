use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreIronMan,
};

pub fn get_repulsor_blast() -> Card {
    Card::Event(EventCard {
        id: "core_031",
        name: "Repulsor Blast",
        description:
            "Hero Action (attack): Deal 1 damage to an enemy and discard the top 5 cards of your deck. For each printed Energy resource discarded this way, deal 2 additional damage to that enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_iron_man/core_031.png",
        traits: vec![CardTrait::Attack,CardTrait::Superpower],
        keywords: vec![],
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        cost: 1,
        res: vec![CardResource::Physical],
        card_amount_max: 3,
    })
}
