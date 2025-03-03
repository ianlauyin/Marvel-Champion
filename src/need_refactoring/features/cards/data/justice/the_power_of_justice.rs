use crate::features::cards::{Card, CardAspect, CardResource, ResourceCard};
pub fn get_the_power_of_justice() -> Card {
    Card::Resource(ResourceCard {
        id: "core_062",
        name: "The Power of Justice",
        aspect: CardAspect::Justice,
        res: vec![CardResource::Wild],
        traits: vec![],
        description: "Max 2 per deck. Double the number of resources this card generates while paying for a Justice (yellow) card.",
        abilities: vec![],
        card_image_path: "embedded://cards/justice/core_062.png",
        card_amount_max: 2,
    })
}
