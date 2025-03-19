use bevy::ecs::{system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_075",
        name: "Black Widow",
        sub_name: Some("Natasha Romanoff"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Protection).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        PlayerCardType::Ally,
        CardCost::constant(3),
        CardResources::physical(),
        CardTraits::new(vec![CardTrait::SHIELD, CardTrait::Spy]),
        CardCharacter::ally(2, 2, 1, 1, 1),
        InstantAbilities::single(Ability::new(instant_ability)),
    ));
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
