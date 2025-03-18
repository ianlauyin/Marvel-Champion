pub enum IdentitySet {
    CoreSpiderMan,
    CoreCaptainMarvel,
    CoreSheHulk,
    CoreIronMan,
    CoreBlackPanther,
}

impl IdentitySet {
    pub fn get_all() -> Vec<Self> {
        vec![
            IdentitySet::CoreSpiderMan,
            IdentitySet::CoreCaptainMarvel,
            IdentitySet::CoreSheHulk,
            IdentitySet::CoreIronMan,
            IdentitySet::CoreBlackPanther,
        ]
    }

    // pub fn get_all_cards() -> Vec<Card> {
    //     let mut cards = Vec::new();
    //     for identity in Identity::get_all() {
    //         cards.push(identity.get_cards());
    //     }
    //     cards.concat()
    // }

    pub fn to_string(&self) -> String {
        let str = match *self {
            IdentitySet::CoreSpiderMan => "Core - Spider Man",
            IdentitySet::CoreCaptainMarvel => "Core - Captain Marvel",
            IdentitySet::CoreSheHulk => "Core - She Hulk",
            IdentitySet::CoreIronMan => "Core - Iron Man",
            IdentitySet::CoreBlackPanther => "Core - Black Panther",
        };
        str.to_string()
    }

    // pub fn get_validator(&self) -> DeckValidator {
    //     match self {
    //         _ => DeckValidator::default(self.clone()),
    //     }
    // }

    // pub fn get_key(&self) -> String {
    //     let key = match *self {
    //         Identity::CoreSpiderMan => "core_spider_man",
    //         Identity::CoreCaptainMarvel => "core_captain_marvel",
    //         Identity::CoreSheHulk => "core_she_hulk",
    //         Identity::CoreIronMan => "core_iron_man",
    //         Identity::CoreBlackPanther => "core_black_panther",
    //     };
    //     key.to_string()
    // }

    // pub fn get_title_image_path(&self) -> String {
    //     let prefix = "embedded://identity/";
    //     let postfix = ".png";
    //     let name = self.get_key();
    //     format!("{prefix}{name}{postfix}")
    // }

    // pub fn get_alter_ego(&self) -> Card {
    //     match *self {
    //         Identity::CoreSpiderMan => core_spider_man::get_alter_ego(),
    //         Identity::CoreCaptainMarvel => core_captain_marvel::get_alter_ego(),
    //         Identity::CoreSheHulk => core_she_hulk::get_alter_ego(),
    //         Identity::CoreIronMan => core_iron_man::get_alter_ego(),
    //         Identity::CoreBlackPanther => core_black_panther::get_alter_ego(),
    //     }
    // }

    // pub fn get_hero(&self) -> Vec<Card> {
    //     match *self {
    //         Identity::CoreSpiderMan => core_spider_man::get_hero(),
    //         Identity::CoreCaptainMarvel => core_captain_marvel::get_hero(),
    //         Identity::CoreSheHulk => core_she_hulk::get_hero(),
    //         Identity::CoreIronMan => core_iron_man::get_hero(),
    //         Identity::CoreBlackPanther => core_black_panther::get_hero(),
    //     }
    // }

    // pub fn get_health(&self) -> u8 {
    //     let Card::AlterEgo(alter_ego_card) = self.get_alter_ego() else {
    //         return 0;
    //     };
    //     alter_ego_card.initial_hit_points
    // }

    // pub fn get_identity_cards(&self) -> Vec<Card> {
    //     match *self {
    //         Identity::CoreSpiderMan => core_spider_man::get_identity_cards(),
    //         Identity::CoreCaptainMarvel => core_captain_marvel::get_identity_cards(),
    //         Identity::CoreSheHulk => core_she_hulk::get_identity_cards(),
    //         Identity::CoreIronMan => core_iron_man::get_identity_cards(),
    //         Identity::CoreBlackPanther => core_black_panther::get_identity_cards(),
    //     }
    // }

    // pub fn get_cards(&self) -> Vec<Card> {
    //     match *self {
    //         Identity::CoreSpiderMan => core_spider_man::get_all(),
    //         Identity::CoreCaptainMarvel => core_captain_marvel::get_all(),
    //         Identity::CoreSheHulk => core_she_hulk::get_all(),
    //         Identity::CoreIronMan => core_iron_man::get_all(),
    //         Identity::CoreBlackPanther => core_black_panther::get_all(),
    //     }
    // }

    // pub fn get_player_cards(&self) -> Vec<Card> {
    //     match *self {
    //         Identity::CoreSpiderMan => core_spider_man::get_player_cards(),
    //         Identity::CoreCaptainMarvel => core_captain_marvel::get_player_cards(),
    //         Identity::CoreSheHulk => core_she_hulk::get_player_cards(),
    //         Identity::CoreIronMan => core_iron_man::get_player_cards(),
    //         Identity::CoreBlackPanther => core_black_panther::get_player_cards(),
    //     }
    // }

    // pub fn get_obligation(&self) -> Card {
    //     match *self {
    //         Identity::CoreSpiderMan => core_spider_man::get_obligation(),
    //         Identity::CoreCaptainMarvel => core_captain_marvel::get_obligation(),
    //         Identity::CoreSheHulk => core_she_hulk::get_obligation(),
    //         Identity::CoreIronMan => core_iron_man::get_obligation(),
    //         Identity::CoreBlackPanther => core_black_panther::get_obligation(),
    //     }
    // }

    // pub fn get_nemesis_set(&self) -> Vec<Card> {
    //     match *self {
    //         Identity::CoreSpiderMan => core_spider_man::get_nemesis_set(),
    //         Identity::CoreCaptainMarvel => core_captain_marvel::get_nemesis_set(),
    //         Identity::CoreSheHulk => core_she_hulk::get_nemesis_set(),
    //         Identity::CoreIronMan => core_iron_man::get_nemesis_set(),
    //         Identity::CoreBlackPanther => core_black_panther::get_nemesis_set(),
    //     }
    // }
}
