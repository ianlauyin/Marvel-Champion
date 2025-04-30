use bevy::prelude::*;

use crate::{
    component::Card,
    node_ui::{
        CardDetailButton, CardNode, ContainerHeader, ContainerHeaderEvent, MainContainer,
        ScrollingList,
    },
    resource::AssetLoader,
};

use super::super::CURRENT_STATE;

#[derive(Component)]
pub struct CollectionCardList(Vec<Card<'static>>);

impl CollectionCardList {
    pub fn new(cards: Vec<Card<'static>>) -> Self {
        Self(cards)
    }
}

pub struct CardListPlugin;

impl Plugin for CardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_header_interaction.run_if(in_state(CURRENT_STATE)),
        )
        .add_observer(on_added);
    }
}

fn on_added(
    trigger: Trigger<OnAdd, CollectionCardList>,
    mut commands: Commands,
    card_list_q: Query<&CollectionCardList>,
    asset_loader: Res<AssetLoader>,
) -> Result<(), BevyError> {
    let main_container = commands
        .entity(trigger.target())
        .insert((
            MainContainer::default(),
            children![ContainerHeader::with_leading_button("X")],
        ))
        .id();

    let content_container = commands
        .spawn((
            ScrollingList::Grid {
                column: 8,
                spacing: 20.,
            },
            ChildOf(main_container),
        ))
        .id();

    let card_list = card_list_q.get(trigger.target())?;
    for card in card_list.0.clone() {
        commands.spawn((
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                CardNode::medium(asset_loader.get(&card.get_key()).clone()),
                CardDetailButton,
                card.clone(),
            ],
            ChildOf(content_container),
        ));
    }
    Ok(())
}

fn handle_header_interaction(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    mut commands: Commands,
    menu_q: Query<(Entity, &Children), With<CollectionCardList>>,
) {
    for event in event_reader.read() {
        for (entity, card_list_children) in menu_q.iter() {
            if let ContainerHeaderEvent::LeadingButtonPressed(header_entity) = event {
                if card_list_children.contains(header_entity) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
