use crate::cards::{IdentitySet, ModularSet, Scenario, StandardSet};

use super::set::{Aspect, ExpertSet, SetTrait};

#[derive(Clone)]
pub struct Belongs {
    main: Belong,
    sub: Vec<Belong>,
}

impl Default for Belongs {
    fn default() -> Self {
        Self {
            main: Belong::Aspect(Aspect::Basic),
            sub: vec![],
        }
    }
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

    pub fn get_aspect(&self) -> Option<Aspect> {
        if let Belong::Aspect(aspect) = &self.main {
            return Some(aspect.clone());
        }
        for sub in &self.sub {
            if let Belong::Aspect(aspect) = sub {
                return Some(aspect.clone());
            }
        }
        None
    }
}

#[derive(Clone)]
pub enum Belong {
    Aspect(Aspect),
    IdentitySet(IdentitySet),
    ModularSet(ModularSet),
    Scenario(Scenario),
    StandardSet(StandardSet),
    ExpertSet(ExpertSet),
}

impl Belong {
    pub fn get_key(&self) -> String {
        match self {
            Self::Aspect(aspect) => format!("aspect/{}", aspect.get_key()),
            Self::IdentitySet(identity_set) => format!("identity_set/{}", identity_set.get_key()),
            Self::ModularSet(modular_set) => format!("modular_set/{}", modular_set.get_key()),
            Self::StandardSet(standard_set) => format!("standard_set/{}", standard_set.get_key()),
            Self::ExpertSet(expert_set) => format!("expert_set/{}", expert_set.get_key()),
            Self::Scenario(scenario) => format!("scenario/{}", scenario.get_key()),
        }
    }
}
