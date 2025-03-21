use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_029a",
        name: "Iron Man",
        sub_name: Some("Tony Stark"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreIronMan).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            IdentityCardType::Hero {
                flip_target_id: vec!["core_029b"],
                hand_size: 1,
            },
            CardBoost::amount(0),
            CardTraits::single(CardTrait::Avenger),
            CardCharacter::hero(9, 2, 1, 1),
        ))
        .id()
}
