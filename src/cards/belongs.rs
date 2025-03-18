use bevy::ecs::world::World;

use crate::{
    cards::{BasicSet, IdentitySet, ModularSet, Scenario},
    component::card::CardBasic,
};

pub struct Belongs {
    main: Belong,
    sub: Vec<Belong>,
}

impl From<Belong> for Belongs {
    fn from(belong: Belong) -> Self {
        Self {
            main: belong,
            sub: vec![],
        }
    }
}

impl From<Vec<Belong>> for Belongs {
    fn from(belongs: Vec<Belong>) -> Self {
        let mut belong_iter = belongs.into_iter();
        if let Some(main) = belong_iter.next() {
            Self {
                main,
                sub: belong_iter.collect(),
            }
        } else {
            panic!("Belongs must have at least one belong");
        }
    }
}

impl Belongs {
    pub fn get_key(&self) -> String {
        self.main.get_key()
    }
}

pub enum Belong {
    Basic,
    Justice,
    Aggression,
    Protection,
    Leadership,
    Pool,
    IdentitySet(IdentitySet),
    ModularSet(ModularSet),
    Scenario(Scenario),
    BasicSet(BasicSet),
}

impl Belong {
    pub fn to_string(&self) -> String {
        match self {
            Self::Basic => "Basic".to_string(),
            Self::Justice => "Justice".to_string(),
            Self::Aggression => "Aggression".to_string(),
            Self::Protection => "Protection".to_string(),
            Self::Leadership => "Leadership".to_string(),
            Self::Pool => "Pool".to_string(),
            Self::IdentitySet(identity_set) => identity_set.to_string(),
            Self::ModularSet(modular_set) => modular_set.to_string(),
            Self::BasicSet(basic_set) => basic_set.to_string(),
            Self::Scenario(scenario) => scenario.to_string(),
        }
    }

    pub fn get_key(&self) -> String {
        match self {
            Self::Basic => "basic".to_string(),
            Self::Justice => "justice".to_string(),
            Self::Aggression => "aggression".to_string(),
            Self::Protection => "protection".to_string(),
            Self::Leadership => "leadership".to_string(),
            Self::Pool => "pool".to_string(),
            Self::IdentitySet(identity_set) => format!("identity_set/{}", identity_set.get_key()),
            Self::ModularSet(modular_set) => format!("modular_set/{}", modular_set.get_key()),
            Self::BasicSet(basic_set) => format!("basic_set/{}", basic_set.get_key()),
            Self::Scenario(scenario) => format!("scenario/{}", scenario.get_key()),
        }
    }

    pub fn get_card_infos(&self) -> Vec<CardBasic> {
        todo!()
    }

    pub fn get_cards(&self) -> Vec<(CardBasic, fn(&mut World))> {
        todo!()
    }
}
