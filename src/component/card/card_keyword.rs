use bevy::ecs::component::Component;

#[derive(Component)]
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
