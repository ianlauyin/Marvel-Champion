use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_078",
        name: "Get Behind Me!",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Protection).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands.spawn((
        get_info(),
        PlayerCardType::Event,
        CardCost::constant(1),
        CardResources::mental(),
        CardFormLimit::hero(),
        InterruptAbilities::single(Ability::hero(interrupt_ability)),
    )).id()
}

fn interrupt_ability(world: &mut World) {
    println!("interrupt_ability");
}

// pub fn get_get_behind_me() -> Card {
//     Card::Event(EventCard {
//         id: "core_078",
//         name: "Get Behind Me!",
//         cost: 1,
//         keywords: vec![],
//         aspect: CardAspect::Protection,
//         res: vec![CardResource::Mental],
//         traits: vec![],
//         description: "Hero Interrupt: When a treachery card is revealed from the encounter deck, cancel its \"When Revealed\" effects. The villain attacks you instead.",
//         abilities: vec![],
//         card_image_path: "embedded://cards/protection/core_078.png",
//         card_amount_max: 3,
//     })
// }
