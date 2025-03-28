use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_058",
        name: "Daredevil",
        sub_name: Some("Matt Murdock"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Justice).into(),
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
            CardCost::constant(4),
            CardResources::physical(),
            CardTraits::single(CardTrait::Defender),
            CardCharacter::ally(3, 2, 1, 2, 1),
        ))
        .id()
}
