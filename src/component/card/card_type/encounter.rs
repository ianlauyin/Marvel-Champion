use bevy::prelude::Component;

use crate::{cards::IdentitySet, component::card::shared::Count};

#[derive(Component)]
pub enum EncounterCardType {
    Attachment { atk_modifier: u8, sch_modifier: u8 },
    Environment,
    Minion,
    Obligation { belong: Option<IdentitySet> },
    SideScheme,
    Treachery,
}
