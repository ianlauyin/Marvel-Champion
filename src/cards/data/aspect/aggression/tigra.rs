use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_051",
        name: "Tigra",
        sub_name: Some("Greer Grant Nelson"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Aggression).into(),
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
            CardCost::constant(3),
            CardResources::mental(),
            CardTraits::new(vec![CardTrait::Avenger]),
            CardCharacter::ally(3, 1, 1, 2, 1),
        ))
        .id()
}
