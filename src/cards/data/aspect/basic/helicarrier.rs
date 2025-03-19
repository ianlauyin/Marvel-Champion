use bevy::ecs::{system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_092",
        name: "Helicarrier",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Basic).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        PlayerCardType::Support,
        CardCost::constant(3),
        CardResources::physical(),
        CardTraits::new(vec![CardTrait::SHIELD, CardTrait::Location]),
        InstantAbilities::single(Ability::new(instant_ability)),
    ));
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
