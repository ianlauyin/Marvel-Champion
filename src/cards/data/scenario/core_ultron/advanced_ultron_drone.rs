use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_143",
        name: "Advanced Ultron Drone",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
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
            EncounterCardType::Minion,
            CardTraits::single(CardTrait::Drone),
            CardKeywords::single(CardKeyword::Guard),
            ForcedInterruptAbilities::single(Ability::new(forced_interrupt_ability)),
            CardBoost::new(2),
            CardCharacter::minion(Count::Constant(4), 1, 1),
        ))
        .id()
}

fn forced_interrupt_ability(world: &mut World) {
    println!("forced_interrupt_ability");
}
