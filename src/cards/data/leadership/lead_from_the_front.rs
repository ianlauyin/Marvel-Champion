use crate::features::cards::{Card, CardAspect, CardResource, CardTrait, EventCard};
pub fn get_lead_from_the_front() -> Card {
    Card::Event(EventCard {
        id: "core_070",
        name: "Lead from the Front",
        cost: 2,
        keywords: vec![],
        aspect: CardAspect::Leadership,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Tactic],
        description: "Hero Action: Choose a player. Each character that player controls gets +1 THW and +1 ATK until the end of the phase.",
        abilities: vec![],
        card_image_path: "embedded://cards/leadership/core_070.png",
        card_amount_max: 3,
    })
}
