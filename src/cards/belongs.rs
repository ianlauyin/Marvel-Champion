use crate::cards::{BasicSet, IdentitySet, ModularSet, Scenario};

use super::set::Aspect;

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Belong {
    Aspect(Aspect),
    IdentitySet(IdentitySet),
    ModularSet(ModularSet),
    Scenario(Scenario),
    BasicSet(BasicSet),
}

impl Belong {
    pub fn to_string(&self) -> String {
        match self {
            Self::Aspect(aspect) => aspect.to_string(),
            Self::IdentitySet(identity_set) => identity_set.to_string(),
            Self::ModularSet(modular_set) => modular_set.to_string(),
            Self::BasicSet(basic_set) => basic_set.to_string(),
            Self::Scenario(scenario) => scenario.to_string(),
        }
    }

    pub fn get_key(&self) -> String {
        match self {
            Self::Aspect(aspect) => format!("aspect/{}", aspect.get_key()),
            Self::IdentitySet(identity_set) => format!("identity_set/{}", identity_set.get_key()),
            Self::ModularSet(modular_set) => format!("modular_set/{}", modular_set.get_key()),
            Self::BasicSet(basic_set) => format!("basic_set/{}", basic_set.get_key()),
            Self::Scenario(scenario) => format!("scenario/{}", scenario.get_key()),
        }
    }
}
