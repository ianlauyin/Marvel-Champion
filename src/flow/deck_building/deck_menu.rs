use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{
    cards::IdentitySet,
    flow::state::AppState,
    node_ui::{
        ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer, ScrollingList,
    },
    util::{Deck, DecksStorageUtil, SystemUtil},
};

use super::resource::{DeckBuildingResource, DeckBuildingState};

pub struct DeckMenuPlugin;

impl Plugin for DeckMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_deck_menu,
                handle_header_button_click,
                handle_deck_menu_button_click,
                despawn_deck_menu,
            )
                .run_if(in_state(AppState::DeckBuilding)),
        );
    }
}

#[derive(Component)]
struct DeckMenu;

#[derive(Component)]
struct DeckMenuButton(Deck);

fn spawn_deck_menu(mut commands: Commands, res: Res<DeckBuildingResource>, pkv: ResMut<PkvStore>) {
    if res.is_changed() && res.get_state() == DeckBuildingState::DeckMenu {
        commands
            .spawn((MainContainer::new(), DeckMenu))
            .with_children(|container| {
                container.spawn(ContainerHeader::with_leading_button("<"));
                container
                    .spawn(ScrollingList::grid(3, 50.))
                    .with_children(|scrolling_list| {
                        let Some(identity) = res.get_identity() else {
                            warn!("No identity found in deck building resource");
                            return;
                        };
                        for deck in get_decks(identity, pkv) {
                            scrolling_list.spawn((
                                CustomButton::large(deck.get_name()),
                                DeckMenuButton(deck),
                            ));
                        }
                        scrolling_list
                            .spawn((CustomButton::large("+"), DeckMenuButton(Deck::new())));
                    });
            });
    }
}

fn handle_deck_menu_button_click(
    deck_menu_button_q: Query<(&Interaction, &DeckMenuButton), Changed<Interaction>>,
    mut res: ResMut<DeckBuildingResource>,
) {
    SystemUtil::handle_button_click(deck_menu_button_q, |deck_menu_button| {
        res.set_deck(deck_menu_button.0.clone());
    });
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    menu_q: Query<&Children, With<DeckMenu>>,
    mut res: ResMut<DeckBuildingResource>,
) {
    for event in event_reader.read() {
        for menu_children in menu_q.iter() {
            match event {
                ContainerHeaderEvent::LeadingButtonPressed(header_entity) => {
                    if menu_children.contains(header_entity) {
                        res.clear_identity();
                    }
                }
            }
        }
    }
}

fn get_decks(identity: IdentitySet, pkv: ResMut<PkvStore>) -> Vec<Deck> {
    DecksStorageUtil::init(&identity, pkv).get_decks()
}

fn despawn_deck_menu(
    commands: Commands,
    deck_menu_q: Query<Entity, With<DeckMenu>>,
    res: Res<DeckBuildingResource>,
) {
    if res.is_changed() && res.get_state() != DeckBuildingState::DeckMenu {
        SystemUtil::cleanup_all(commands, deck_menu_q);
    }
}
