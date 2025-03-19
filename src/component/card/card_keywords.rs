use bevy::ecs::component::Component;

use super::CardCounter;

#[derive(Component)]
pub struct CardKeywords(Vec<CardKeyword>);

impl CardKeywords {
    pub fn single(card_keyword: CardKeyword) -> Self {
        Self(vec![card_keyword])
    }
}

pub enum CardKeyword {
    Retaliate(u8),
    Quickstrike,
    Use(CardCounter),
    Toughness,
    Surge,
    Guard,
    Permanent,
}
