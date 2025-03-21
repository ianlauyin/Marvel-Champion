use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_020",
        name: "Hellcat",
        sub_name: Some("Patsy Walker"),
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
            PlayerCardType::Ally,
            CardTraits::single(CardTrait::Avenger),
            CardCharacter::ally(3, 2, 1, 1, 1),
            CardResources::wild(),
            CardCost::constant(3),
        ))
        .id()
}
