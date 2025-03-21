use bevy::prelude::Component;

#[derive(Component)]
pub enum CollectionMenuButton {
    Aspect,
    IdentitySet,
    ModularSet,
    Scenario,
    StandardSet,
    ExpertSet,
}

impl CollectionMenuButton {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::Aspect,
            Self::IdentitySet,
            Self::ModularSet,
            Self::Scenario,
            Self::StandardSet,
            Self::ExpertSet,
        ]
    }

    pub fn get_text(&self) -> String {
        match self {
            Self::Aspect => "Aspect".to_string(),
            Self::IdentitySet => "Identity Set".to_string(),
            Self::ModularSet => "Modular Set".to_string(),
            Self::Scenario => "Scenario".to_string(),
            Self::StandardSet => "Standard Set".to_string(),
            Self::ExpertSet => "Expert Set".to_string(),
        }
    }
}
