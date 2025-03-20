use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_116a",
        name: "Underground Distribution - 1A",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::Scenario(Scenario::CoreKlaw).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            ScenarioCardType::MainSchemeA {
                next_stage_id: "core_116b",
            },
            SetupAbilities::single(Ability::new(setup_ability)),
        ))
        .id()
}

fn setup_ability(world: &mut World) {
    println!("setup_ability");
}
