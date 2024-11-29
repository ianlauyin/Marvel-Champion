use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreBlackPanther, SupportCard,
};

pub fn get_the_golden_city() -> Card {
    Card::Support(SupportCard {
        id: "core_045",
        name: "The Golden City",
        description: "Alter-Ego Action: Exhaust The Golden City -> draw 2 cards",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_black_panther/core_045.png",
        traits: vec![CardTrait::Location, CardTrait::Wakanda],
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        res: vec![CardResource::Energy],
        card_amount_max: 1,
        unique: true,
        cost: 2,
        card_icons: vec![],
        keywords: vec![],
    })
}
