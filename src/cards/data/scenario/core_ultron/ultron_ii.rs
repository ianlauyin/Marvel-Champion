use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_135",
        name: "Ultron (II)",
        sub_name: None,
        unique: true,
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
            ScenarioCardType::Villain {
                next_villain_id: None,
            },
            CardCharacter::villain(Count::PerPlayer(22), 2, 2),
            CardTraits::single(CardTrait::Android),
            ForcedInterruptAbilities::single(Ability::new(forced_interrupt_ability)),
        ))
        .id()
}

fn forced_interrupt_ability(world: &mut World) {
    println!("forced_interrupt_ability");
}
