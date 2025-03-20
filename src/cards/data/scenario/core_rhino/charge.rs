use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_099",
        name: "Charge",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::Scenario(Scenario::CoreRhino).into(),
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
            CardBoost::new(2),
            StatsModifier::new(0, 3, 0),
            ForcedInterruptAbilities::single(Ability::new(forced_interrupt_ability)),
        ))
        .id()
}

fn forced_interrupt_ability(world: &mut World) {
    println!("forced_interrupt_ability");
}
