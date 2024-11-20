use bevy::prelude::Component;

use super::builders::*;

#[derive(Component, Clone)]
pub enum Card {
    // Opponent Card
    Attachment(AttachmentCard<'static>),
    Environment(EnvironmentCard<'static>),
    MainScheme(MainSchemeCard<'static>),
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
            Card::MainScheme(main_scheme_card) => main_scheme_card.card_image_path,
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
            Card::Attachment(attachment_card) => attachment_card.card_back_image_path,
            Card::Environment(environment_card) => environment_card.card_back_image_path,
            Card::MainScheme(main_scheme_card) => main_scheme_card.card_back_image_path,
            Card::Minion(minion_card) => minion_card.card_back_image_path,
            Card::Obligation(obligation_card) => obligation_card.card_back_image_path,
            Card::SideScheme(side_scheme_card) => side_scheme_card.card_back_image_path,
            Card::Treachery(treachery_card) => treachery_card.card_back_image_path,
            Card::Villain(villain_card) => villain_card.card_back_image_path,
            Card::Ally(ally_card) => ally_card.card_back_image_path,
            Card::AlterEgo(alter_ego_card) => alter_ego_card.card_back_image_path,
            Card::Event(event_card) => event_card.card_back_image_path,
            Card::Hero(hero_card) => hero_card.card_back_image_path,
            Card::Resource(resource_card) => resource_card.card_back_image_path,
            Card::Support(support_card) => support_card.card_back_image_path,
            Card::Upgrade(upgrade_card) => upgrade_card.card_back_image_path,
        };
        image_path.to_string()
    }
}
