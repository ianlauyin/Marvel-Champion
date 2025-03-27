use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_157",
        name: "Killmonger",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreBlackPanther).into(),
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
            CardBoost::amount(2),
            EncounterCardType::Minion,
            CardTraits::new(vec![
                CardTrait::Assassin,
                CardTrait::Elite,
                CardTrait::Mercenary,
            ]),
            CardCharacter::minion(Count::Constant(5), 2, 2),
        ))
        .id()
}
