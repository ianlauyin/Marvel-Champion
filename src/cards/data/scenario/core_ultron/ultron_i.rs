use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_134",
        name: "Ultron (I)",
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
                next_villain_id: Some("core_135"),
            },
            CardCharacter::villain(Count::PerPlayer(17), 1, 2),
            CardTraits::single(CardTrait::Android),
            ForcedResponseAbilities::single(Ability::new(forced_response_ability)),
        ))
        .id()
}

fn forced_response_ability(world: &mut World) {
    println!("forced_response_ability");
}
