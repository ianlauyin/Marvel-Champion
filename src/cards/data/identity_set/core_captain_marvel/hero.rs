use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_010a",
        name: "Captain Marvel",
        sub_name: Some("Hero"),
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
            IdentityCardType::Hero {
                flip_target_id: vec!["core_010b"],
                hand_size: 5,
            },
            CardCharacter::hero(12, 2, 2, 1),
            CardTraits::new(vec![CardTrait::Avenger, CardTrait::Soldier]),
        ))
        .id()
}
