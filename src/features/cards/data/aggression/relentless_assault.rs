use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, CardTrait, EventCard},
};
pub fn get_relentless_assault() -> Card {
    Card::Event(EventCard {
        id: "core_053",
        name: "Relentless Assault",
        aspect: CardAspect::Aggression,
        cost: 2,
        res: vec![CardResource::Energy],
        keywords: vec![],
        traits: vec![CardTrait::Attack],
        description: "Hero Action (attack): Deal 5 damage to a minion. If you paid for this card using a Physical resource, this attack gains overkill. (Excess damage from this attack is dealt to the villain.)",
        abilities: vec![],
        card_image_path: "embedded://cards/aggression/core_053.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 3,
    })
}
