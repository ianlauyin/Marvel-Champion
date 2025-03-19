use bevy::prelude::Component;

#[derive(Component)]
pub enum EncounterCardType {
    Attachment,
    Environment,
    Minion,
    Obligation,
    SideScheme,
    Treachery,
}
