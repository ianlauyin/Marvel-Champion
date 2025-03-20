use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_094",
        name: "Rhino (I)",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
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
            ScenarioCardType::Villain {
                next_villain_id: Some("core_095"),
            },
            CardCharacter::villain(Count::PerPlayer(14), 1, 2),
            CardTraits::new(vec![CardTrait::Brute, CardTrait::Criminal]),
        ))
        .id()
}
