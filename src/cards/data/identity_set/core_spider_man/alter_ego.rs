use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_001b",
        name: "Peter Parker",
        sub_name: Some("Spider-Man"),
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
            IdentityCardType::AlterEgo {
                flip_target_id: vec!["core_001a"],
                hand_size: 6,
            },
            CardTraits::single(CardTrait::Genius),
            CardCharacter::alter_ego(10, 3),
        ))
        .id()
}
