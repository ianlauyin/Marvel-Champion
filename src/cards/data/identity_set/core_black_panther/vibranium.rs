use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_044",
        name: "Vibranium",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::IdentitySet(IdentitySet::CoreBlackPanther).into(),
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
            CardResources::new(vec![CardResource::Wild, CardResource::Wild]),
        ))
        .id()
}
