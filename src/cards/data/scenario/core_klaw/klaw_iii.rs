use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_115",
        name: "Klaw (III)",
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
                next_villain_id: None,
            },
            CardCharacter::villain(Count::PerPlayer(22), 3, 2),
            CardTraits::single(CardTrait::MastersOfEvil),
            CardKeywords::single(CardKeyword::Toughness),
        ))
        .id()
}
