use bevy::prelude::Component;

use super::builders::*;

#[derive(Component)]
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
