use bevy::prelude::Component;

#[derive(Component)]
pub enum EncounterCardType {
    Attachment { atk_modifier: u8, sch_modifier: u8 },
    Environment,
    Minion,
    Obligation,
    SideScheme,
    Treachery,
}
