use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_101",
        name: "Hydra Mercenary",
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
            EncounterCardType::Minion,
            CardBoost::new(1),
            CardCharacter::minion(3, 0, 1),
            CardKeywords::single(CardKeyword::Guard),
            CardTraits::single(CardTrait::Hydra),
        ))
        .id()
}
