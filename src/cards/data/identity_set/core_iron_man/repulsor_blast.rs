use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_031",
        name: "Repulsor Blast",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::IdentitySet(IdentitySet::CoreIronMan).into(),
        is_vertical: true,
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            PlayerCardType::Event,
            CardCost::constant(1),
            CardTraits::new(vec![CardTrait::Attack, CardTrait::Superpower]),
            CardResources::physical(),
            CardFormLimit::hero(),
        ))
        .id()
}
