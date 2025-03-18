use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreCaptainMarvel, UpgradeCard,
};
pub fn get_energy_channel() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_018",
        name: "Energy Channel",
        unique: false,
        aspect: CardAspect::IdentitySpecific(CoreCaptainMarvel),
        cost: 0,
        res: vec![CardResource::Mental],
        traits: vec![CardTrait::Superpower],
        keywords: vec![],
        card_icons: vec![],
        description: "Max 1 per player. Action: Spend X Energy resources -> put X energy counters here. Hero Action (attack): Discard Energy Channel → deal 2 damage to an enemy (to a maximum of 10) for each energy counter here.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_captain_marvel/core_018.png",
        card_amount_max: 2,
    })
}
