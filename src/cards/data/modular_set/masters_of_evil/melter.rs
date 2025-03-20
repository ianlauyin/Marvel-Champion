use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_132",
        name: "Melter",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
        belongs: Belong::ModularSet(ModularSet::MastersOfEvil).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            EncounterCardType::Minion,
            CardBoost::new(0),
            CardTraits::single(CardTrait::MastersOfEvil),
            CardCharacter::minion(Count::Constant(5), 1, 3),
            ConstantAbilities::single(Ability::new(constant_ability)),
            BoostAbilities::single(Ability::new(boost_ability)),
        ))
        .id()
}

fn constant_ability(world: &mut World) {
    println!("constant_ability");
}

fn boost_ability(world: &mut World) {
    println!("boost_ability");
}
