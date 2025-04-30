use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{
    cards::IdentitySet,
    flow::game::resource::PlayersInfo,
    node_ui::{ContainerHeader, ContainerHeaderEvent, CustomButton, MainContainer, ScrollingList},
    util::{Deck, DecksStorageUtil, SystemUtil},
};

use super::state::PreGameState;

#[derive(Component)]
pub struct DeckMenu(pub IdentitySet);

pub struct DeckMenuPlugin;

impl Plugin for DeckMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_deck_menu_button_click, handle_header_button_click)
                .run_if(in_state(PreGameState::HeroMenu)),
        )
        .add_observer(on_deck_menu_added);
    }
}

#[derive(Component)]
struct DeckMenuButton(Option<Deck>);

fn on_deck_menu_added(
    trigger: Trigger<OnAdd, DeckMenu>,
    mut commands: Commands,
    deck_menu_q: Query<&DeckMenu>,
    players_info: Res<PlayersInfo>,
    pkv: ResMut<PkvStore>,
) {
    let deck_menu = deck_menu_q.get(trigger.target()).unwrap();
    commands
        .entity(trigger.target())
        .insert(MainContainer::default())
        .with_children(|container| {
            container.spawn(ContainerHeader::with_leading_button("X"));
            container
                .spawn(ScrollingList::Grid {
                    column: 3,
                    spacing: 50.,
                })
                .with_children(|scrolling_list| {
                    for deck in get_decks(&deck_menu.0, pkv) {
                        scrolling_list.spawn((
                            CustomButton::large(deck.get_name()),
                            DeckMenuButton(Some(deck)),
                        ));
                    }
                    if players_info.contains_identity(&deck_menu.0) {
                        scrolling_list
                            .spawn((CustomButton::large("Remove Player"), DeckMenuButton(None)));
                    }
                });
        });
}

fn handle_deck_menu_button_click(
    mut commands: Commands,
    deck_menu_q: Query<(Entity, &DeckMenu)>,
    deck_menu_button_q: Query<(&Interaction, &DeckMenuButton), Changed<Interaction>>,
    mut players_info: ResMut<PlayersInfo>,
) {
    SystemUtil::handle_component_click(deck_menu_button_q, |deck_menu_button| {
        let (entity, deck_menu) = deck_menu_q.single().unwrap();
        if let Some(deck) = &deck_menu_button.0 {
            players_info.save_player(&deck_menu.0, deck.get_card_ids());
        } else {
            players_info.remove_player(&deck_menu.0);
        }
        commands.entity(entity).despawn();
    });
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    mut commands: Commands,
    menu_q: Query<(Entity, &Children), With<DeckMenu>>,
) {
    for event in event_reader.read() {
        for (entity, menu_children) in menu_q.iter() {
            match event {
                ContainerHeaderEvent::LeadingButtonPressed(header_entity) => {
                    if menu_children.contains(header_entity) {
                        commands.entity(entity).despawn();
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_decks(identity: &IdentitySet, pkv: ResMut<PkvStore>) -> Vec<Deck> {
    DecksStorageUtil::init(identity, pkv).get_decks()
}
