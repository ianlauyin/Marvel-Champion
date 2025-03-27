use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_040a",
        name: "Black Panther",
        sub_name: Some("T'Challa"),
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
            IdentityCardType::Hero {
                hand_size: 5,
                flip_target_id: vec!["core_040b"],
            },
            CardCharacter::hero(11, 2, 2, 2),
            CardTraits::new(vec![CardTrait::Avenger, CardTrait::Wakanda]),
            CardKeywords::single(CardKeyword::Retaliate(1)),
        ))
        .id()
}
