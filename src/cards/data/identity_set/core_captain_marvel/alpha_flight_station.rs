use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_015",
        name: "Alpha Flight Station",
        sub_name: None,
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
            PlayerCardType::Support,
            CardCost::constant(1),
            CardResources::mental(),
            CardTraits::new(vec![CardTrait::Location, CardTrait::SHIELD]),
        ))
        .id()
}
