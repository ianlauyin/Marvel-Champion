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

    pub fn get_text(&self) -> &str {
        match self {
            Self::Aspect => "Aspect",
            Self::IdentitySet => "Identity Set",
            Self::ModularSet => "Modular Set",
            Self::Scenario => "Scenario",
            Self::StandardSet => "Standard Set",
            Self::ExpertSet => "Expert Set",
        }
    }
}
