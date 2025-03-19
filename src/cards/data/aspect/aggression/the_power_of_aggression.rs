use bevy::ecs::{system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_055",
        name: "The Power of Aggression",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::Aspect(Aspect::Aggression).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        PlayerCardType::Resource,
        CardResources::wild(),
        ResourceModifier::single(Ability::new(resource_modifier)),
    ));
}

fn resource_modifier(world: &mut World) {
    println!("instant_ability");
}
