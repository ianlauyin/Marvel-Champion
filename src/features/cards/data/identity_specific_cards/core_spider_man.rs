use crate::features::cards::Identity::CoreSpiderMan;
use crate::features::cards::*;

pub fn get_all_cards(player_number: u8) -> Vec<Card> {
    vec![
        get_nemesis_card(),
        get_nemesis_side_scheme_card(player_number),
        get_sweeping_swoop(),
        get_vultures_plans(),
        get_hero_card(),
        get_alter_ego_card(),
        get_black_cat(),
        get_backflip(),
        get_enhanced_spider_sense(),
        get_swinging_web_kick(),
        get_aunt_may(),
        get_spider_tracer(),
        get_web_shooter(),
        get_webbed_up(),
    ]
}

// Nemesis Cards

fn get_nemesis_card() -> Card {
    Card::Minion(MinionCard {
        id: "core_167",
        name: "Vulture",
        unique: true,
        initial_hit_points: 4,
        keywords: vec![Keyword::Quickstrike],
        traits: vec![CardTrait::Criminal],
        card_icons: vec![],
        sch: 1,
        atk: 3,
        boost: 2,
        description: "Quickstrike. (After this minion engages your hero, it attacks.)",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_167.png",
        card_back_image_path: "embedded://cards/card_backs/encounter_card_back.png",
    })
}

fn get_nemesis_side_scheme_card(player_number: u8) -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_166",
        name: "Highway Robbery",
        boost: 3,
        initial_threat: 3 * player_number,
        card_icons: vec![CardIcon::Acceleration],
        description:
            "When Revealed: Each player places a random card from their hand facedown here.
            When Defeated: Return each facedown card here to its owner's hand.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_166.png",
        card_back_image_path: "embedded://cards/card_backs/encounter_card_back.png",
    })
}

fn get_sweeping_swoop() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_168",
        name: "Sweeping Swoop",
        traits: vec![],
        boost: 0,
        description: "When Revealed: Stun your hero. If Vulture is in play, this card gains surge.
        Boost: If this activation deals damage to a friendly character, stun that character.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_168.png",
        card_back_image_path: "embedded://cards/card_backs/encounter_card_back.png",
    })
}

fn get_vultures_plans() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_169",
        name: "The Vulture's Plans",
        traits: vec![],
        boost: 2,
        description: "When Revealed: Discard 1 card at random from each player's hand. Place 1 threat on the main scheme for each different resource type discarded this way.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_169.png",
        card_back_image_path: "embedded://cards/card_backs/encounter_card_back.png",
    })
}

// Player Cards

fn get_hero_card() -> Card {
    Card::Hero(HeroCard {
        id: "core_1a",
        name: "Spider-Man",
        flip_target_id: vec!["core_1b"],
        initial_hit_points: 10,
        keywords: vec![],
        traits: vec![CardTrait::Avenger],
        card_icons: vec![],
        thw: 1,
        atk: 2,
        def: 3,
        description: "Spider-Sense — Interrupt: When the villain initiates an attack against you, draw 1 card.",
        search_keywords: vec![],
        hand_size: 5,
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_1a.png",
        card_back_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_1b.png",
        nemesis_id: "core_167",
        nemesis_side_scheme_id: "core_166",
        nemesis_card_id: vec!["core_168","core_169"]
    })
}

fn get_alter_ego_card() -> Card {
    Card::AlterEgo(AlterEgoCard {
        id: "core_1b",
        name: "Peter Parker",
        flip_target_id: vec!["core_1a"],
        initial_hit_points: 10,
        keywords: vec![],
        traits: vec![CardTrait::Genius],
        card_icons: vec![],
        rec: 3,
        description: "Scientist — Resource: Generate a  resource. (Limit once per round.)",
        search_keywords: vec![],
        hand_size: 6,
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_1b.png",
        card_back_image_path:
            "embedded://cards/identity_specific_cards/core_spider_man/core_1a.png",
        nemesis_id: "core_167",
        nemesis_side_scheme_id: "core_166",
        nemesis_card_id: vec!["core_168", "core_169"],
    })
}

fn get_black_cat() -> Card {
    Card::Ally(AllyCard {
        id: "core_002",
        name: "Black Cat",
        sub_name: "Felicia Hardy",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        unique: true,
        cost: 2,
        res: vec![CardResource::Energy],
        initial_hit_points: 2,
        keywords: vec![],
        traits: vec![CardTrait::HeroForHire],
        card_icons: vec![],
        thw: 1,
        thw_con_dmg:1,
        atk: 1,
        atk_con_dmg: 0,
        description: "Forced Response: After you play Black Cat, discard the top 2 cards of your deck. Add each card with a printed  resource discarded this way to your hand.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_002.png",
        card_back_image_path: "embedded://cards/card_backs/player_card_back.png",
        card_amount_max: 1,
    })
}

fn get_backflip() -> Card {
    Card::Event(EventCard {
        id: "core_003",
        name: "Backflip",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        cost: 0,
        res: vec![CardResource::Physical],
        traits: vec![CardTrait::Defense,CardTrait::Skill],
        description: "Interrupt (defense): When you would take any amount of damage from an attack, prevent all of that damage.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_003.png",
        card_back_image_path: "embedded://cards/card_backs/player_card_back.png",
        card_amount_max: 2,
    })
}

fn get_enhanced_spider_sense() -> Card {
    Card::Event(EventCard {
        id: "core_004",
        name: "Enhanced Spider-Sense",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        cost: 1,
        res: vec![CardResource::Mental],
        traits: vec![CardTrait::Superpower],
        description: "Hero Interrupt: When a treachery card is revealed from the encounter deck, cancel its \"When Revealed\" effects.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_004.png",
        card_back_image_path: "embedded://cards/card_backs/player_card_back.png",
        card_amount_max: 2,
    })
}

fn get_swinging_web_kick() -> Card {
    Card::Event(EventCard {
        id: "core_005",
        name: "Swinging Web Kick",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        cost: 3,
        res: vec![CardResource::Mental],
        traits: vec![CardTrait::Aerial, CardTrait::Attack, CardTrait::Superpower],
        description: "Hero Action (attack): Deal 8 damage to an enemy.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_005.png",
        card_back_image_path: "embedded://cards/card_backs/player_card_back.png",
        card_amount_max: 3,
    })
}

fn get_aunt_may() -> Card {
    Card::Support(SupportCard {
        id: "core_006",
        name: "Aunt May",
        unique: true,
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        cost: 1,
        res: vec![CardResource::Energy],
        card_icons: vec![],
        traits: vec![CardTrait::Persona],
        description: "Alter-Ego Action: Exhaust Aunt May → heal 4 damage from Peter Parker.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_006.png",
        card_back_image_path: "embedded://cards/card_backs/player_card_back.png",
        card_amount_max: 1,
    })
}

fn get_spider_tracer() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_007",
        name: "Spider-Tracer",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        unique: false,
        cost: 1,
        res: vec![CardResource::Energy],
        card_icons: vec![],
        traits: vec![CardTrait::Item,CardTrait::Tech],
        keywords: vec![],
        description: "Attach to a minion. Forced Interrupt: When attached minion is defeated, remove 3 threat from a scheme.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_007.png",
        card_back_image_path: "embedded://cards/card_backs/player_card_back.png",
        card_amount_max: 2,
    })
}

fn get_web_shooter() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_008",
        name: "Web-Shooter",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        unique: false,
        cost: 1,
        res: vec![CardResource::Physical],
        card_icons: vec![],
        traits: vec![CardTrait::Item,CardTrait::Tech],
        keywords: vec![Keyword::Use(3, Counter::Web)],
        description: "Uses (3 web counters). (Enters play with 3 counters. When those are gone, discard this card)
        Hero Resource: Exhaust Web-Shooter and remove 1 web counter from it → generate a  resource.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_008.png",
        card_back_image_path: "embedded://cards/card_backs/player_card_back.png",
        card_amount_max: 2,
    })
}

fn get_webbed_up() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_009",
        name: "Webbed Up",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        unique: false,
        cost: 4,
        res: vec![CardResource::Physical],
        card_icons: vec![],
        traits: vec![CardTrait::Condition],
        keywords: vec![],
        description: "Hero form only. Attach to an enemy. Max 1 per enemy.
        Forced Interrupt: When attached enemy would attack, discard Webbed Up instead. Then, stun that enemy.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_cards/core_spider_man/core_009.png",
        card_back_image_path: "embedded://cards/card_backs/player_card_back.png",
        card_amount_max: 2,
    })
}
