use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

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
            CardBoost::amount(1),
            CardTraits::single(CardTrait::Armor),
        ))
        .id()
}
