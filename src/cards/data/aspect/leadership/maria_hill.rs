use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_067",
        name: "Maria Hill",
        sub_name: Some("Maria Hill"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Leadership).into(),
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
            CardCost::constant(2),
            CardResources::mental(),
            CardTraits::single(CardTrait::SHIELD),
            CardCharacter::ally(2, 2, 1, 1, 1),
        ))
        .id()
}
