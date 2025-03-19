use bevy::ecs::{system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_059",
        name: "Jessica Jones",
        sub_name: Some("Jessica Jones"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Justice).into(),
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
        CardResources::energy(),
        CardTraits::single(CardTrait::Defender),
        CardCharacter::ally(3, 1, 1, 2, 1),
        ConstantAbilities::single(Ability::new(constant_ability)),
    ));
}

fn constant_ability(world: &mut World) {
    println!("constant_ability");
}