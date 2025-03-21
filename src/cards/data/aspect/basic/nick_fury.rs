use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_084",
        name: "Nick Fury",
        sub_name: Some("Nick Fury"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Basic).into(),
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
            CardCost::constant(4),
            CardResources::mental(),
            CardTraits::new(vec![CardTrait::SHIELD, CardTrait::Spy]),
            CardCharacter::ally(3, 2, 1, 2, 1),
        ))
        .id()
}
