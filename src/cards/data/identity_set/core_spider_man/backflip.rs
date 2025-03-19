use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreSpiderMan,
};

pub fn get_backflip() -> Card {
    Card::Event(EventCard {
        id: "core_003",
        name: "Backflip",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        cost: 0,
        res: vec![CardResource::Physical],
        traits: vec![CardTrait::Defense,CardTrait::Skill],
        keywords: vec![],
        description: "Interrupt (defense): When you would take any amount of damage from an attack, prevent all of that damage.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_spider_man/core_003.png",
        card_amount_max: 2,
    })
}
