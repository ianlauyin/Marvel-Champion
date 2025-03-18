use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CardKeywords(Vec<CardKeyword>);

impl From<CardKeyword> for CardKeywords {
    fn from(value: CardKeyword) -> Self {
        Self(vec![value])
    }
}

pub enum CardKeyword {
    Retaliate(u8),
    Quickstrike,
    Use(u8, Counter),
    Toughness,
    Surge,
    Guard,
    Permanent,
}

pub enum Counter {
    Attack,
    Web,
    Medical,
    Snoop,
}
