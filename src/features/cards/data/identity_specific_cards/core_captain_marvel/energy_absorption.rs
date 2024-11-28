use crate::features::cards::{
    Card, CardAspect, CardResource, Identity::CoreCaptainMarvel, ResourceCard,
};
pub fn get_energy_absorption() -> Card {
    Card::Resource(ResourceCard {
        id: "core_014",
        name: "Energy Absorption",
        aspect: CardAspect::IdentitySpecific(CoreCaptainMarvel),
        res: vec![
            CardResource::Energy,
            CardResource::Energy,
            CardResource::Energy,
        ],
        traits: vec![],
        description: "",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_captain_marvel/core_014.png",
        card_amount_max: 2,
    })
}
