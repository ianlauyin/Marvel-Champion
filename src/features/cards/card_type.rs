use bevy::prelude::World;

#[derive(Clone)]
pub enum CardResource {
    Wild,
    Energy,
    Mental,
    Physical,
}

#[derive(Clone)]
pub enum Identity {
    CoreSpiderMan,
}

impl Identity {
    pub fn to_string(&self) -> String {
        match *self {
            Identity::CoreSpiderMan => "Core - Spider man".to_string(),
        }
    }
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
        match *self {
            CardAspect::Justice => "Justice".to_string(),
            CardAspect::Aggression => "Aggression".to_string(),
            CardAspect::Protection => "Protection".to_string(),
            CardAspect::Leadership => "Leadership".to_string(),
            CardAspect::Pool => "Pool".to_string(),
            CardAspect::IdentitySpecific(ref identity) => identity.to_string(),
            CardAspect::Basic => "Basic".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum CardTrait {
    // Charater-related
    Avenger,
    Criminal,
    HeroForHire,
    Genius,
    Persona,
    // Event-related
    Defense,
    Skill,
    Superpower,
    Aerial,
    Attack,
    // Upgrade-related
    Item,
    Tech,
    Condition,
}

impl CardTrait {
    pub fn to_string(&self) -> String {
        match *self {
            // Charater-related
            CardTrait::Avenger => "Avenger".to_string(),
            CardTrait::Criminal => "Criminal".to_string(),
            CardTrait::HeroForHire => "HeroForHire".to_string(),
            CardTrait::Genius => "Genius".to_string(),
            CardTrait::Persona => "Persona".to_string(),
            // Event-related
            CardTrait::Defense => "Defense".to_string(),
            CardTrait::Skill => "Skill".to_string(),
            CardTrait::Superpower => "Superpower".to_string(),
            CardTrait::Aerial => "Aerial".to_string(),
            CardTrait::Attack => "Attack".to_string(),
            // Upgrade-related
            CardTrait::Item => "Item".to_string(),
            CardTrait::Tech => "Tech".to_string(),
            CardTrait::Condition => "Condition".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum Counter {
    Web,
}

impl Counter {
    pub fn to_string(&self) -> String {
        match *self {
            Counter::Web => "web".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum Keyword {
    Retaliate,
    Quickstrike,
    Use(u8, Counter),
}
impl Keyword {
    pub fn to_string(&self) -> String {
        match self {
            Keyword::Retaliate => "Retaliate".to_string(),
            Keyword::Quickstrike => "Quickstrike".to_string(),
            Keyword::Use(count, counter) => {
                format!("{} {} counters", count, counter.to_string())
            }
        }
    }
}

#[derive(Clone)]
pub enum CardIcon {
    Acceleration,
}

impl CardIcon {
    pub fn to_string(&self) -> String {
        match *self {
            CardIcon::Acceleration => "Acceleration".to_string(),
        }
    }
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
