use crate::features::cards::{Card, CardAspect, CardResource, EventCard, Identity::CoreSheHulk};

pub fn get_split_personality() -> Card {
    Card::Event(EventCard {
        id: "core_025",
        name: "Split Personality",
        description: "Action: Change your form (flip your identity card). Then, draw up to your printed hand size.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_025.png",
        keywords: vec![],
        traits: vec![],
        aspect: CardAspect::IdentitySpecific(CoreSheHulk),
        cost: 3,
        res: vec![CardResource::Energy],
        card_amount_max: 1,
    })
}
