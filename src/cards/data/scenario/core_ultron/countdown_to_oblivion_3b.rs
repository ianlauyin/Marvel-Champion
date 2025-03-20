use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_139b",
        name: "Countdown to Oblivion - 3B",
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
            ScenarioCardType::MainSchemeB {
                next_stage_id: None,
            },
            CardScheme::main_scheme(Count::Constant(0), Count::PerPlayer(5), Count::PerPlayer(1)),
            ConstantAbilities::single(Ability::new(constant_ability)),
        ))
        .id()
}

fn constant_ability(world: &mut World) {
    println!("constant_ability");
}
