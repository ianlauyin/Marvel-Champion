use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_079",
        name: "The Power of Protection",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::Aspect(Aspect::Protection).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands.spawn((
        get_info(),
        PlayerCardType::Resource,
        CardResources::wild(),
        ResourceModifier::single(Ability::new(resource_modifier)),
    )).id()
}

fn resource_modifier(world: &mut World) {
    println!("resource_modifier");
}
