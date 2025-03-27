use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_030",
        name: "War Machine",
        sub_name: Some("James Rhodes"),
        unique: true,
        card_amount_max: 1,
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
            PlayerCardType::Ally,
            CardTraits::new(vec![CardTrait::SHIELD, CardTrait::Soldier]),
            CardCharacter::ally(4, 1, 1, 2, 1),
            CardResources::wild(),
            CardCost::constant(4),
        ))
        .id()
}
