use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_118",
        name: "Sonic Converter",
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
            EncounterCardType::Attachment,
            CardBoost::new(3),
            CardTraits::single(CardTrait::Weapon),
            StatsModifier::new(0, 1, 0),
            ForcedResponseAbilities::single(Ability::new(forced_response_ability)),
            InstantAbilities::single(Ability::hero(instant_ability)),
        ))
        .id()
}

fn forced_response_ability(world: &mut World) {
    println!("forced_response_ability");
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
