use bevy::prelude::World;

use super::Identity;

#[derive(Clone)]
pub enum CardResource {
    Wild,
    Energy,
    Mental,
    Physical,
}

#[derive(Clone)]
pub enum CardAspect {
    Justice,
    Aggression,
    Protection,
    Leadership,
    Pool,
    IdentitySpecific(Identity),
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

#[derive(Clone)]
pub enum CardTrait {
    // Charater-related
    Avenger,
    Assassin,
    Android,
    Attorney,
    Brute,
    Criminal,
    Cyborg,
    Defender,
    Drone,
    Elite,
    HeroForHire,
    Gamma,
    Genius,
    Hydra,
    Kree,
    King,
    Persona,
    SHIELD,
    Spy,
    Soldier,
    Mercenary,
    MasterOfEvil,
    // Event-related
    Aerial,
    Attack,
    Defense,
    Skill,
    Superpower,
    Tactic,
    Thwart,
    // Upgrade-related
    Armor,
    Item,
    Tech,
    Condition,
    Weapon,
    // Support-related
    Location,
    // Other
    BlackPanther,
    Wakanda,
}

#[derive(Clone)]
pub enum Counter {
    Attack,
    Web,
    Medical,
    Snoop,
}

#[derive(Clone)]
pub enum Keyword {
    Retaliate(u8),
    Quickstrike,
    Use(u8, Counter),
    Toughness,
    Surge,
    Guard,
}

#[derive(Clone)]
pub enum CardIcon {
    Acceleration,
    Amplify,
    Crisis,
    Hazard,
}

#[derive(Clone)]
pub enum CardAbility {
    Instant(fn(&mut World)),
    ForcedInterrupt(fn(&mut World)),
    Interrupt(fn(&mut World)),
    Boost(fn(&mut World)),
    WhenDefeated(fn(&mut World)),
    WhenRevealed(fn(&mut World)),
    ForcedResponse(fn(&mut World)),
    Response(fn(&mut World)),
}

#[derive(Clone)]
pub enum Count {
    PerPlayer(u8),
    Constant(u8),
}

impl Count {
    pub fn to_actual(&self, player_number: u8) -> u8 {
        match *self {
            Count::PerPlayer(n) => n * player_number,
            Count::Constant(n) => n,
        }
    }
}
