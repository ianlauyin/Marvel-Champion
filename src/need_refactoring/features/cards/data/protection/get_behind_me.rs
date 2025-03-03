use crate::features::cards::{Card, CardAspect, CardResource, EventCard};
pub fn get_get_behind_me() -> Card {
    Card::Event(EventCard {
        id: "core_078",
        name: "Get Behind Me!",
        cost: 1,
        keywords: vec![],
        aspect: CardAspect::Protection,
        res: vec![CardResource::Mental],
        traits: vec![],
        description: "Hero Interrupt: When a treachery card is revealed from the encounter deck, cancel its \"When Revealed\" effects. The villain attacks you instead.",
        abilities: vec![],
        card_image_path: "embedded://cards/protection/core_078.png",
        card_amount_max: 3,
    })
}
