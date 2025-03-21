use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_086",
        name: "First Aid",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
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
            PlayerCardType::Event,
            CardCost::constant(1),
            CardResources::mental(),
        ))
        .id()
}
