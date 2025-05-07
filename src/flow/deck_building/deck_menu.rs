use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{
    node_ui::{ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer, ScrollingList},
    util::{Deck, DecksStorageUtil, SystemUtil},
};

use super::{resource::DeckBuildingResource, state::DeckBuildingState};

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckMenu;

pub struct DeckMenuPlugin;

impl Plugin for DeckMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_deck_menu)
            .add_systems(
                Update,
                (handle_header_button_click, handle_deck_menu_button_click)
                    .run_if(in_state(CURRENT_STATE)),
            )
            .add_systems(OnExit(CURRENT_STATE), SystemUtil::cleanup_all::<DeckMenu>);
    }
}

#[derive(Component)]
struct DeckMenu;

#[derive(Component)]
struct DeckMenuButton(Deck);

fn spawn_deck_menu(mut commands: Commands, res: Res<DeckBuildingResource>, pkv: ResMut<PkvStore>) {
    commands
        .spawn((MainContainer::default(), DeckMenu))
        .with_children(|container| {
            container.spawn(ContainerHeader::with_leading_button("<"));
            container
                .spawn(ScrollingList::Grid {
                    column: 3,
                    spacing: 50.,
                })
                .with_children(|scrolling_list| {
                    let Some(identity) = res.get_identity() else {
                        warn!("No identity found in deck building resource");
                        return;
                    };
                    for deck in DecksStorageUtil::init(&identity, pkv).get_decks() {
                        scrolling_list
                            .spawn((CustomButton::large(deck.get_name()), DeckMenuButton(deck)));
                    }
                    scrolling_list.spawn((CustomButton::large("+"), DeckMenuButton(Deck::new())));
                });
        });
}

fn handle_deck_menu_button_click(
    deck_menu_button_q: Query<(&Interaction, &DeckMenuButton), Changed<Interaction>>,
    mut res: ResMut<DeckBuildingResource>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
) {
    SystemUtil::handle_component_click(deck_menu_button_q, |deck_menu_button| {
        res.set_deck(deck_menu_button.0.clone());
        next_state.set(DeckBuildingState::DeckEditor);
    });
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    menu_q: Query<&Children, With<DeckMenu>>,
    mut res: ResMut<DeckBuildingResource>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
) {
    for event in event_reader.read() {
        for menu_children in menu_q.iter() {
            match event {
                ContainerHeaderEvent::LeadingButtonPressed(header_entity) => {
                    if menu_children.contains(header_entity) {
                        res.clear_identity();
                        next_state.set(DeckBuildingState::HeroMenu);
                    }
                }
                _ => {}
            }
        }
    }
}
