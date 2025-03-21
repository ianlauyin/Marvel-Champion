use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_011",
        name: "Spider-Woman",
        sub_name: Some("Jessica Drew"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreCaptainMarvel).into(),
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
            CardCost::constant(3),
            CardResources::wild(),
            CardCharacter::ally(2, 2, 1, 2, 1),
            CardTraits::new(vec![CardTrait::Avenger, CardTrait::Spy]),
        ))
        .id()
}
