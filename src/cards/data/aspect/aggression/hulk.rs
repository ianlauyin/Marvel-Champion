use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_050",
        name: "Hulk",
        sub_name: Some("Bruce Banner"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Aggression).into(),
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
            CardCost::constant(2),
            CardResources::energy(),
            CardTraits::new(vec![CardTrait::Avenger, CardTrait::Gamma]),
            CardCharacter::ally(5, 0, 0, 3, 1),
        ))
        .id()
}
