use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreCaptainMarvel,
};
pub fn get_crisis_interdiction() -> Card {
    Card::Event(EventCard {
        id: "core_012",
        name: "Crisis Interdiction",
        aspect: CardAspect::IdentitySpecific(CoreCaptainMarvel),
        cost: 2,
        res: vec![CardResource::Energy],
        keywords: vec![],
        traits: vec![CardTrait::Thwart],
        description: "Hero Action (thwart): Remove 2 threat from a scheme. Then, if you have the Aerial trait, remove 2 threat from a different scheme.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_captain_marvel/core_012.png",
        card_amount_max: 3,
    })
}
