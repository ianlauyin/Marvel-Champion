use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_002",
        name: "Black Cat",
        sub_name: Some("Felicia Hardy"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreSpiderMan).into(),
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
            CardCost::constant(2),
            CardResources::energy(),
            CardCharacter::ally(2, 1, 1, 1, 0),
            CardTraits::single(CardTrait::HeroForHire),
        ))
        .id()
}
