use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreBlackPanther, UpgradeCard,
};

pub fn get_energy_daggers() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_046",
        name: "Energy Daggers",
        description: "Special: Choose a player. Deal 1 damage to the villain and to each enemy engaged with that player (2 damage instead if this is the final step of this sequence).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_black_panther/core_046.png",
        traits: vec![CardTrait::BlackPanther, CardTrait::Weapon],
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        res: vec![CardResource::Mental],
        card_amount_max: 1,
        unique: false,
        cost: 2,
        card_icons: vec![],
        keywords: vec![],
    })
}
