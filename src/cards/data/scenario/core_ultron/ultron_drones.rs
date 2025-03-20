use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_140",
        name: "Ultron Drones",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
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
            EncounterCardType::Environment,
            ConstantAbilities::single(Ability::new(constant_ability)),
            ForcedResponseAbilities::single(Ability::new(forced_response_ability)),
        ))
        .id()
}

fn constant_ability(world: &mut World) {
    println!("constant_ability");
}

fn forced_response_ability(world: &mut World) {
    println!("forced_response_ability");
}
