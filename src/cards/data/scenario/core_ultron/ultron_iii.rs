use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_136",
        name: "Ultron (III)",
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
                hit_points: Count::PerPlayer(27),
                sch: 2,
                atk: 4,
                next_villain_id: None,
            },
            CardTraits::single(CardTrait::Android),
            WhenRevealedAbilities::single(Ability::new(when_revealed_ability)),
            ConstantAbilities::single(Ability::new(constant_ability)),
        ))
        .id()
}

fn when_revealed_ability(world: &mut World) {
    println!("when_revealed_ability");
}

fn constant_ability(world: &mut World) {
    println!("constant_ability");
}
