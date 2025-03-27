use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_121",
        name: "Weapons Runner",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::Scenario(Scenario::CoreKlaw).into(),
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
            CardBoost::amount(0),
            CardCharacter::minion(Count::Constant(2), 1, 1),
            CardKeywords::single(CardKeyword::Surge),
            CardTraits::single(CardTrait::Mercenary),
        ))
        .id()
}
