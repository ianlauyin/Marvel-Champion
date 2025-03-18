use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CardTraits(Vec<CardTrait>);

impl CardTraits {
    pub fn new(traits: Vec<CardTrait>) -> Self {
        Self(traits)
    }
}

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
