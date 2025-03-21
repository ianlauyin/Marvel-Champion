use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_039",
        name: "Rocket Boots",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::IdentitySet(IdentitySet::CoreIronMan).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            PlayerCardType::Upgrade,
            CardCost::constant(1),
            CardTraits::new(vec![CardTrait::Armor, CardTrait::Tech]),
            CardResources::mental(),
        ))
        .id()
}
