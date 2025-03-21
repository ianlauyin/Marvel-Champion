use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_091",
        name: "Avengers Mansion",
        sub_name: None,
        unique: false,
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
            PlayerCardType::Support,
            CardCost::constant(4),
            CardResources::mental(),
            CardTraits::new(vec![CardTrait::Avenger, CardTrait::Location]),
        ))
        .id()
}
