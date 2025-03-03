use crate::features::cards::{Card, CardAspect, CardResource, EventCard};
pub fn get_great_responsiblity() -> Card {
    Card::Event(EventCard {
        id: "core_061",
        name: "Great Responsibility",
        cost: 0,
        keywords: vec![],
        aspect: CardAspect::Justice,
        res: vec![CardResource::Mental],
        traits: vec![],
        description: "Hero Interrupt: When any amount of threat would be placed on a scheme, you take it as damage instead.",
        abilities: vec![],
        card_image_path: "embedded://cards/justice/core_061.png",
        card_amount_max: 3,
    })
}
