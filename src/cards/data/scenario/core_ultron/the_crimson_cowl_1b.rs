use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_137b",
        name: "The Crimson Cowl - 1B",
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
                next_stage_id: Some("core_138a"),
            },
            CardScheme::main_scheme(Count::Constant(0), Count::PerPlayer(3), Count::PerPlayer(1)),
            WhenRevealedAbilities::single(Ability::new(when_revealed_ability)),
        ))
        .id()
}

fn when_revealed_ability(world: &mut World) {
    println!("when_revealed_ability");
}
