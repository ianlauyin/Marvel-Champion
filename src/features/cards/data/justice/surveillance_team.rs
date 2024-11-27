use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, CardTrait, SupportCard},
};
pub fn get_surveillance_team() -> Card {
    Card::Support(SupportCard {
        id: "core_064",
        name: "Surveillance Team",
        unique: false,
        cost: 2,
        card_icons: vec![],
        aspect: CardAspect::Justice,
        res: vec![CardResource::Mental],
        traits: vec![CardTrait::SHIELD],
        description: "Uses (3 snoop counters). (Enters play with 3 counters. When those are gone, discard this card) Action: Exhaust Surveillance Team and remove 1 snoop counter from it -> remove 1 threat from a scheme.",
        abilities: vec![],
        card_image_path: "embedded://cards/justice/core_064.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 3,
    })
}
