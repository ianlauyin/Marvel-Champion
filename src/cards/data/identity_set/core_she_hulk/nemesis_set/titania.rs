use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_162",
        name: "Titania",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreSheHulk).into(),
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
            CardBoost::amount(2),
            CardCharacter::minion(Count::Constant(6), 1, 0),
            CardTraits::new(vec![CardTrait::Brute, CardTrait::Elite]),
        ))
        .id()
}
