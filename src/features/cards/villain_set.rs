use super::{data::core_rhino, Card};

#[derive(Clone)]
pub enum VillainSet {
    CoreRhino,
}

impl VillainSet {
    pub fn get_all() -> Vec<Self> {
        vec![VillainSet::CoreRhino]
    }

    pub fn get_all_cards() -> Vec<Card> {
        let mut cards = Vec::new();
        for identity in VillainSet::get_all() {
            cards.push(identity.get_cards());
        }
        cards.concat()
    }

    pub fn to_string(&self) -> String {
        let str = match *self {
            VillainSet::CoreRhino => "Core - Rhino",
        };
        str.to_string()
    }
    pub fn get_title_image_path(&self) -> String {
        let prefix = "embedded://villain/";
        let postfix = ".png";
        let name = match *self {
            VillainSet::CoreRhino => "core_rhino",
        };
        format!("{prefix}{name}{postfix}")
    }

    pub fn get_cards(&self) -> Vec<Card> {
        match *self {
            VillainSet::CoreRhino => core_rhino::get_all(),
        }
    }
}
