use bevy::prelude::{System, World};

use super::{Card, CardDatas, Identity};

#[derive(Clone)]
pub enum CardResource {
    Wild,
    Energy,
    Mental,
    Physical,
}

#[derive(Clone, PartialEq, Eq, Hash)]
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

    pub fn get_cards(&self) -> Vec<Card> {
        match self {
            CardAspect::Justice => CardDatas::get_justice_cards(),
            CardAspect::Aggression => CardDatas::get_aggression_cards(),
            CardAspect::Protection => CardDatas::get_protection_cards(),
            CardAspect::Leadership => CardDatas::get_leadership_cards(),
            CardAspect::Pool => CardDatas::get_pool_cards(),
            CardAspect::IdentitySpecific(identity) => identity.get_cards(),
            CardAspect::Basic => CardDatas::get_basic_cards(),
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

#[derive(Clone, PartialEq, Eq)]
pub enum Counter {
    Attack,
    Web,
    Medical,
    Snoop,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Keyword {
    Retaliate(u8),
    Quickstrike,
    Use(u8, Counter),
    Toughness,
    Surge,
    Guard,
    Permanent,
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

#[derive(Clone, PartialEq, Eq)]
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
