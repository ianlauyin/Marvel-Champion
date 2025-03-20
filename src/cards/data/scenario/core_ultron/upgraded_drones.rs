use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_142",
        name: "Upgraded Drones",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::Scenario(Scenario::CoreUltron).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            EncounterCardType::Attachment,
            CardBoost::new(0),
            CardTraits::single(CardTrait::Condition),
            InstantAbilities::single(Ability::hero(instant_ability)),
            ConstantAbilities::single(Ability::new(constant_ability)),
        ))
        .id()
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}

fn constant_ability(world: &mut World) {
    println!("constant_ability");
}
