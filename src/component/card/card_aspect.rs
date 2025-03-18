use crate::cards::IdentitySet;
use bevy::prelude::Component;

#[derive(Component)]
pub enum CardAspect {
    Justice,
    Aggression,
    Protection,
    Leadership,
    Pool,
    IdentitySpecific(IdentitySet),
    Basic,
}

impl CardAspect {
    pub fn to_string(&self) -> String {
        match self {
            CardAspect::Justice => "Justice".to_string(),
            CardAspect::Aggression => "Aggression".to_string(),
            CardAspect::Protection => "Protection".to_string(),
            CardAspect::Leadership => "Leadership".to_string(),
            CardAspect::Pool => "Pool".to_string(),
            CardAspect::IdentitySpecific(identity) => identity.to_string(),
            CardAspect::Basic => "Basic".to_string(),
        }
    }
}
