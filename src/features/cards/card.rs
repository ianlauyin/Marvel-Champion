use bevy::{log::warn, prelude::Component};

use crate::constants::{
    ENCOUNTER_CARD_BACK_ASSET, PLAYER_CARD_BACK_ASSET, VILLAIN_CARD_BACK_ASSET,
};

use super::builders::*;

#[derive(Component, Clone)]
pub enum Card {
    // Opponent Card
    Attachment(AttachmentCard<'static>),
    Environment(EnvironmentCard<'static>),
    MainSchemeA(MainSchemeACard<'static>),
    MainSchemeB(MainSchemeBCard<'static>),
    Minion(MinionCard<'static>),
    Obligation(ObligationCard<'static>),
    SideScheme(SideSchemeCard<'static>),
    Treachery(TreacheryCard<'static>),
    Villain(VillainCard<'static>),
    // Player Card
    Ally(AllyCard<'static>),
    AlterEgo(AlterEgoCard<'static>),
    Event(EventCard<'static>),
    Hero(HeroCard<'static>),
    Resource(ResourceCard<'static>),
    Support(SupportCard<'static>),
    Upgrade(UpgradeCard<'static>),
}

impl Card {
    pub fn get_card_image_path(&self) -> String {
        let image_path = match self {
            Card::Attachment(attachment_card) => attachment_card.card_image_path,
            Card::Environment(environment_card) => environment_card.card_image_path,
            Card::MainSchemeA(main_scheme_card) => main_scheme_card.card_image_path,
            Card::MainSchemeB(main_scheme_card) => main_scheme_card.card_image_path,
            Card::Minion(minion_card) => minion_card.card_image_path,
            Card::Obligation(obligation_card) => obligation_card.card_image_path,
            Card::SideScheme(side_scheme_card) => side_scheme_card.card_image_path,
            Card::Treachery(treachery_card) => treachery_card.card_image_path,
            Card::Villain(villain_card) => villain_card.card_image_path,
            Card::Ally(ally_card) => ally_card.card_image_path,
            Card::AlterEgo(alter_ego_card) => alter_ego_card.card_image_path,
            Card::Event(event_card) => event_card.card_image_path,
            Card::Hero(hero_card) => hero_card.card_image_path,
            Card::Resource(resource_card) => resource_card.card_image_path,
            Card::Support(support_card) => support_card.card_image_path,
            Card::Upgrade(upgrade_card) => upgrade_card.card_image_path,
        };
        image_path.to_string()
    }

    pub fn get_card_back_image_path(&self) -> String {
        let image_path = match self {
            Card::Attachment(_)
            | Card::Environment(_)
            | Card::Minion(_)
            | Card::Obligation(_)
            | Card::SideScheme(_)
            | Card::Treachery(_) => ENCOUNTER_CARD_BACK_ASSET.path,
            Card::Ally(_)
            | Card::Event(_)
            | Card::Resource(_)
            | Card::Support(_)
            | Card::Upgrade(_) => PLAYER_CARD_BACK_ASSET.path,
            Card::Villain(_) => VILLAIN_CARD_BACK_ASSET.path,
            // Hero, AlterEgo, MainScheme should have no card back
            _ => {
                warn!("{} should not have a card back", self.get_name());
                return "".to_string();
            }
        };
        image_path.to_string()
    }

    pub fn get_card_id(&self) -> String {
        let id = match self {
            Card::Attachment(attachment_card) => attachment_card.id,
            Card::Environment(environment_card) => environment_card.id,
            Card::MainSchemeA(main_scheme_card) => main_scheme_card.id,
            Card::MainSchemeB(main_scheme_card) => main_scheme_card.id,
            Card::Minion(minion_card) => minion_card.id,
            Card::Obligation(obligation_card) => obligation_card.id,
            Card::SideScheme(side_scheme_card) => side_scheme_card.id,
            Card::Treachery(treachery_card) => treachery_card.id,
            Card::Villain(villain_card) => villain_card.id,
            Card::Ally(ally_card) => ally_card.id,
            Card::AlterEgo(alter_ego_card) => alter_ego_card.id,
            Card::Event(event_card) => event_card.id,
            Card::Hero(hero_card) => hero_card.id,
            Card::Resource(resource_card) => resource_card.id,
            Card::Support(support_card) => support_card.id,
            Card::Upgrade(upgrade_card) => upgrade_card.id,
        };
        id.to_string()
    }

    pub fn get_name(&self) -> String {
        let id = match self {
            Card::Attachment(attachment_card) => attachment_card.name,
            Card::Environment(environment_card) => environment_card.name,
            Card::MainSchemeA(main_scheme_card) => main_scheme_card.name,
            Card::MainSchemeB(main_scheme_card) => main_scheme_card.name,
            Card::Minion(minion_card) => minion_card.name,
            Card::Obligation(obligation_card) => obligation_card.name,
            Card::SideScheme(side_scheme_card) => side_scheme_card.name,
            Card::Treachery(treachery_card) => treachery_card.name,
            Card::Villain(villain_card) => villain_card.name,
            Card::Ally(ally_card) => ally_card.name,
            Card::AlterEgo(alter_ego_card) => alter_ego_card.name,
            Card::Event(event_card) => event_card.name,
            Card::Hero(hero_card) => hero_card.name,
            Card::Resource(resource_card) => resource_card.name,
            Card::Support(support_card) => support_card.name,
            Card::Upgrade(upgrade_card) => upgrade_card.name,
        };
        id.to_string()
    }
}
