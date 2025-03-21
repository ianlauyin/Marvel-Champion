use bevy::prelude::Component;

#[derive(Component)]
pub struct CardIcons(Vec<CardIcon>);

impl CardIcons {
    pub fn acceleration() -> Self {
        Self(vec![CardIcon::Acceleration])
    }

    pub fn amplify() -> Self {
        Self(vec![CardIcon::Amplify])
    }

    pub fn crisis() -> Self {
        Self(vec![CardIcon::Crisis])
    }

    pub fn hazard() -> Self {
        Self(vec![CardIcon::Hazard])
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
