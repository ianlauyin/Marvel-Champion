use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_100",
        name: "Enhanced Ivory Horn",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
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
            CardTraits::single(CardTrait::Weapon),
            StatsModifier::new(0, 1, 0),
            InstantAbilities::single(Ability::hero(instant_ability)),
            ForcedInterruptAbilities::single(Ability::new(forced_interrupt_ability)),
        ))
        .id()
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}

fn forced_interrupt_ability(world: &mut World) {
    println!("forced_interrupt_ability");
}
