use bevy::prelude::Component;

#[derive(Component)]
pub struct CardIcons(Vec<CardIcon>);

impl CardIcons {
    pub fn single(icon: CardIcon) -> Self {
        Self(vec![icon])
    }

    pub fn new(icons: Vec<CardIcon>) -> Self {
        Self(icons)
    }
}

pub enum CardIcon {
    Acceleration,
    Amplify,
    Crisis,
    Hazard,
}
