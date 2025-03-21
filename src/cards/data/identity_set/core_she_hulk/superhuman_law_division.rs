use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_026",
        name: "Superhuman Law Division",
        sub_name: None,
        unique: false,
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
            PlayerCardType::Support,
            CardTraits::single(CardTrait::Location),
            CardCost::constant(1),
            CardResources::physical(),
        ))
        .id()
}
