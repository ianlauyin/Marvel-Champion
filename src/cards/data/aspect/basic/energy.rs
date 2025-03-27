use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_088",
        name: "Energy",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Basic).into(),
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
            PlayerCardType::Resource,
            CardResources::new(vec![CardResource::Energy, CardResource::Energy]),
        ))
        .id()
}
