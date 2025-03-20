use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands, world::World};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_152",
        name: "Vibranium Armor",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::ModularSet(ModularSet::UnderAttack).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            EncounterCardType::Attachment,
            CardBoost::new(1),
            CardTraits::single(CardTrait::Armor),
            ForcedResponseAbilities::single(Ability::new(forced_response)),
            InstantAbilities::single(Ability::hero(instant_ability)),
        ))
        .id()
}

fn forced_response(world: &mut World) {
    println!("forced_response");
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
