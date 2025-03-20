use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_114",
        name: "Klaw (II)",
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
                next_villain_id: Some("core_115"),
            },
            CardCharacter::villain(Count::PerPlayer(18), 2, 1),
            CardTraits::single(CardTrait::MastersOfEvil),
            WhenRevealedAbilities::single(Ability::new(when_revealed_ability)),
            ForcedInterruptAbilities::single(Ability::new(forced_interrupt_ability)),
        ))
        .id()
}

fn when_revealed_ability(world: &mut World) {
    println!("when_revealed_ability");
}

fn forced_interrupt_ability(world: &mut World) {
    println!("forced_interrupt_ability");
}
