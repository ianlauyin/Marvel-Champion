pub enum ModularSet {
    BombScare,
    MastersOfEvil,
    UnderAttack,
    LegionsOfHydra,
    TheDoomsdayChair,
}

impl ModularSet {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::BombScare,
            Self::MastersOfEvil,
            Self::UnderAttack,
            Self::LegionsOfHydra,
            Self::TheDoomsdayChair,
        ]
    }

    // pub fn get_all_cards() -> Vec<Card> {
    //     let mut cards = Vec::new();
    //     for modular_set in ModularSet::get_all() {
    //         cards.push(modular_set.get_cards());
    //     }
    //     cards.concat()
    // }

    pub fn to_string(&self) -> String {
        let str = match *self {
            ModularSet::BombScare => "Bomb Scare",
            ModularSet::MastersOfEvil => "Masters of Evil",
            ModularSet::UnderAttack => "Under Attack",
            ModularSet::LegionsOfHydra => "Legions Of Hydra",
            ModularSet::TheDoomsdayChair => "The Doomsday Chair",
        };
        str.to_string()
    }

    // pub fn get_title_image_path(&self) -> String {
    //     let prefix = "embedded://modular/";
    //     let postfix = ".png";
    //     let name = match *self {
    //         ModularSet::BombScare => "bomb_scare",
    //         ModularSet::MastersOfEvil => "master_of_evil",
    //         ModularSet::UnderAttack => "under_attack",
    //         ModularSet::LegionsOfHydra => "legions_of_hydra",
    //         ModularSet::TheDoomsdayChair => "the_doomsday_chair",
    //     };
    //     format!("{prefix}{name}{postfix}")
    // }

    // pub fn get_cards(&self) -> Vec<Card> {
    //     match *self {
    //         ModularSet::Standard => modular::standard::get_all(),
    //         ModularSet::Expert => modular::expert::get_all(),
    //         ModularSet::BombScare => modular::bomb_scare::get_all(),
    //         ModularSet::MastersOfEvil => modular::master_of_evil::get_all(),
    //         ModularSet::UnderAttack => modular::under_attack::get_all(),
    //         ModularSet::LegionsOfHydra => modular::legions_of_hydra::get_all(),
    //         ModularSet::TheDoomsdayChair => modular::the_doomsday_chair::get_all(),
    //     }
    // }
}
