use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_143",
        name: "Advanced Ultron Drone",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::Scenario(Scenario::CoreUltron).into(),
        is_vertical: true,
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
            CardBoost::amount(2),
            CardCharacter::minion(Count::Constant(4), 1, 1),
        ))
        .id()
}
