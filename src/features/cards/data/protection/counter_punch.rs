use crate::features::cards::{Card, CardAspect, CardResource, CardTrait, EventCard};
pub fn get_counter_punch() -> Card {
    Card::Event(EventCard {
        id: "core_077",
        name: "Counter-Punch",
        cost: 0,
        keywords: vec![],
        aspect: CardAspect::Protection,
        res: vec![CardResource::Physical],
        traits: vec![CardTrait::Attack],
        description: "Response (attack): After your hero defends against an enemy attack, deal damage to that enemy equal to your hero's ATK.",
        abilities: vec![],
        card_image_path: "embedded://cards/protection/core_077.png",
        card_amount_max: 3,
    })
}
