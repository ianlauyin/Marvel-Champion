use bevy::ecs::{system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_054",
        name: "Uppercut",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Aggression).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        PlayerCardType::Event,
        CardCost::constant(3),
        CardResources::physical(),
        CardTraits::single(CardTrait::Attack),
        CardFormLimit::hero(),
        InstantAbilities::single(Ability::hero(instant_ability)),
    ));
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
