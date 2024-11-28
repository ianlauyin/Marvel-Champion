use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreCaptainMarvel, SupportCard,
};
pub fn get_alpha_flight_station() -> Card {
    Card::Support(SupportCard {
        id: "core_015",
        name: "Alpha Flight Station",
        aspect: CardAspect::IdentitySpecific(CoreCaptainMarvel),
        unique: true,
        cost:1,
        res: vec![CardResource::Mental],
        keywords: vec![],
        traits: vec![CardTrait::Location, CardTrait::SHIELD],
        card_icons: vec![],
        description: "Action: Exhaust Alpha Flight Station, choose and discard 1 card from your hand -> draw 1 card (draw 2 cards instead if you are Carol Danvers).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_captain_marvel/core_015.png",
        card_amount_max: 1,
    })
}
