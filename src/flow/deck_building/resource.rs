use bevy::prelude::*;

use crate::{cards::IdentitySet, flow::state::AppState, util::Deck};

#[derive(Resource, Default)]
pub struct DeckBuildingResource {
    identity: Option<IdentitySet>,
    deck: Option<Deck>,
}

#[derive(PartialEq)]
pub enum DeckBuildingState {
    HeroMenu,
    DeckMenu,
    DeckEditor,
}

impl DeckBuildingResource {
    pub fn get_state(&self) -> DeckBuildingState {
        if self.identity.is_none() {
            DeckBuildingState::HeroMenu
        } else if self.deck.is_none() {
            DeckBuildingState::DeckMenu
        } else {
            DeckBuildingState::DeckEditor
        }
    }

    pub fn set_identity(&mut self, identity: IdentitySet) {
        self.identity = Some(identity);
    }

    pub fn clear_identity(&mut self) {
        self.identity = None;
    }

    pub fn get_identity(&self) -> Option<IdentitySet> {
        self.identity.clone()
    }

    pub fn get_deck(&self) -> Option<Deck> {
        self.deck.clone()
    }

    pub fn set_deck(&mut self, deck: Deck) {
        self.deck = Some(deck);
    }

    pub fn add_card(&mut self, card_id: &str) {
        self.deck.as_mut().unwrap().push_card_id(card_id);
    }

    pub fn remove_card(&mut self, card_id: &str) {
        self.deck.as_mut().unwrap().remove_card_id(card_id);
    }

    pub fn clear_deck(&mut self) {
        self.deck = None;
    }
}

pub struct DeckBuildingResourcePlugin;

const CURRENT_STATE: AppState = AppState::DeckBuilding;

impl Plugin for DeckBuildingResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), init_resources)
            .add_systems(OnExit(CURRENT_STATE), cleanup_resources);
    }
}

fn init_resources(mut commands: Commands) {
    commands.insert_resource(DeckBuildingResource::default());
}

fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<DeckBuildingResource>();
}
