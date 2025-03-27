use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_019a",
        name: "She-Hulk",
        sub_name: Some("Jennifer Walters"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreSheHulk).into(),
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
                flip_target_id: vec!["core_019b"],
                hand_size: 4,
            },
            CardTraits::new(vec![CardTrait::Avenger, CardTrait::Gamma]),
            CardCharacter::hero(15, 1, 3, 2),
        ))
        .id()
}
