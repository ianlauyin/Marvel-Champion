use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_113",
        name: "Klaw (I)",
        sub_name: None,
        unique: true,
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
            ScenarioCardType::Villain {
                hit_points: Count::PerPlayer(12),
                sch: 2,
                atk: 0,
                next_villain_id: Some("core_114"),
            },
            CardTraits::single(CardTrait::MastersOfEvil),
            ForcedInterruptAbilities::single(Ability::new(forced_interrupt_ability)),
        ))
        .id()
}

fn forced_interrupt_ability(world: &mut World) {
    println!("forced_interrupt_ability");
}
