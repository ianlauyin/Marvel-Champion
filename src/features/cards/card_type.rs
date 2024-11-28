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

#[derive(Clone)]
pub enum CardTrait {
    // Charater-related
    Avenger,
    Criminal,
    Defender,
    HeroForHire,
    Genius,
    Persona,
    SHIELD,
    Spy,
    Gamma,
    Android,
    Elite,
    Kree,
    Soldier,
    Brute,
    Attorney,
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
    // Support-related
    Location,
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
    Retaliate,
    Quickstrike,
    Use(u8, Counter),
    Toughness,
    Surge,
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
