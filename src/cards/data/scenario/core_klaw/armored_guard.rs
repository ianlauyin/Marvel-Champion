use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_120",
        name: "Armored Guard",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
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
            EncounterCardType::Minion,
            CardBoost::amount(1),
            CardKeywords::new(vec![CardKeyword::Guard, CardKeyword::Toughness]),
            CardTraits::single(CardTrait::Mercenary),
            CardCharacter::minion(Count::Constant(3), 0, 1),
        ))
        .id()
}
