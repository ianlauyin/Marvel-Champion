use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_117b",
        name: "Secret Rendezvous - 2B",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::Scenario(Scenario::CoreKlaw).into(),
        is_vertical: false,
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
            CardScheme::main_scheme(Count::Constant(0), Count::PerPlayer(8), Count::PerPlayer(1)),
        ))
        .id()
}
