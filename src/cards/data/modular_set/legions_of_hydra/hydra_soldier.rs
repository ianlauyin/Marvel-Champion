use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_182",
        name: "Hydra Soldier",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::ModularSet(ModularSet::LegionsOfHydra).into(),
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
            CardKeywords::single(CardKeyword::Guard),
            CardTraits::single(CardTrait::Hydra),
            CardCharacter::minion(4, 1, 2),
            WhenDefeatedAbilities::single(Ability::new(when_defeated_ability)),
        ))
        .id()
}

fn when_defeated_ability(world: &mut World) {
    println!("when_defeated_ability");
}
