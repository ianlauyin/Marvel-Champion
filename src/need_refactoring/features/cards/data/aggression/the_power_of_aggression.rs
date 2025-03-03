use crate::features::cards::{Card, CardAspect, CardResource, CardTrait, ResourceCard};
pub fn get_the_power_of_aggression() -> Card {
    Card::Resource(ResourceCard {
        id: "core_055",
        name: "The Power of Aggression",
        aspect: CardAspect::Aggression,
        res: vec![CardResource::Wild],
        traits: vec![CardTrait::Avenger],
        description: "Max 2 per deck. Double the number of resources this card generates while paying for a Aggression (red) card.",
        abilities: vec![],
        card_image_path: "embedded://cards/aggression/core_055.png",
        card_amount_max: 2,
    })
}
