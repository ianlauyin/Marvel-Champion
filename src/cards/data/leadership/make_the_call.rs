use crate::features::cards::{Card, CardAspect, CardResource, CardTrait, EventCard};
pub fn get_make_the_call() -> Card {
    Card::Event(EventCard {
        id: "core_071",
        name: "Make the Call",
        cost: 0,
        keywords: vec![],
        aspect: CardAspect::Leadership,
        res: vec![CardResource::Mental],
        traits: vec![CardTrait::Tactic],
        description: "Action: Pay the printed cost of an ally in any player's discard pile -> put that ally into play under your control.",
        abilities: vec![],
        card_image_path: "embedded://cards/leadership/core_071.png",
        card_amount_max: 3,
    })
}
