use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_181",
        name: "Madame Hydra",
        sub_name: None,
        unique: true,
        card_amount_max: 2,
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
            CardBoost::new(2),
            CardTraits::single(CardTrait::Hydra),
            CardCharacter::minion(6, 2, 2),
            ConstantAbilities::single(Ability::new(constant_ability)),
            ForcedResponseAbilities::single(Ability::new(forced_response_ability)),
        ))
        .id()
}

fn constant_ability(world: &mut World) {
    println!("constant_ability");
}

fn forced_response_ability(world: &mut World) {
    println!("forced_response_ability");
}
