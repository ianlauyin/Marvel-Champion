use crate::features::cards::{
    Card, CardAspect, CardResource, Identity::CoreBlackPanther, ResourceCard,
};

pub fn get_vibranium() -> Card {
    Card::Resource(ResourceCard {
        id: "core_044",
        name: "Vibranium",
        description: "",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_black_panther/core_044.png",
        traits: vec![],
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        res: vec![CardResource::Wild, CardResource::Wild],
        card_amount_max: 3,
    })
}
