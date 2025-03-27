use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    constant::WINDOW_RESOLUTION,
    resource::AssetLoader,
    util::{MouseControl, MouseControlEvent, UiUtils},
};

use super::{Card, ContainerHeader, ContainerHeaderEvent};

#[derive(Component)]
#[require(MouseControl)]
pub struct CardDetailButton {
    image_key: String,
    is_vertical: bool,
}

impl CardDetailButton {
    pub fn new(image_key: String, is_vertical: bool) -> Self {
        Self {
            image_key,
            is_vertical,
        }
    }
}
pub struct CardDetailPlugin;

impl Plugin for CardDetailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_header_button_click, on_drag))
            .add_observer(listen_card_detail_button_click);
    }
}

#[derive(Component)]
#[require(Interaction)]
struct CardDetail;

fn listen_card_detail_button_click(
    trigger: Trigger<MouseControlEvent>,
    card_detail_button_q: Query<&CardDetailButton>,
    commands: Commands,
    asset_loader: Res<AssetLoader>,
    z_index_q: Query<&ZIndex>,
) {
    match trigger.event() {
        MouseControlEvent::ShortClick(entity) => {
            if let Ok(card_detail_button) = card_detail_button_q.get(*entity) {
                spawn_card_detail(
                    commands,
                    Card::large(
                        asset_loader.get(&card_detail_button.image_key).clone(),
                        card_detail_button.is_vertical,
                    ),
                    UiUtils::get_largest_z_index(z_index_q),
                );
            }
        }
        _ => {}
    }
}

fn spawn_card_detail(mut commands: Commands, card: Card, z_index: ZIndex) {
    commands
        .spawn((
            CardDetail,
            z_index,
            Node {
                width: Val::Px(600.),
                height: Val::Px(600.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                left: Val::Px(WINDOW_RESOLUTION.x / 2. - 300.),
                top: Val::Px(WINDOW_RESOLUTION.y / 2. - 300.),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            FocusPolicy::Block,
            BorderColor::from(Color::WHITE),
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
        ))
        .with_children(|container| {
            container.spawn(ContainerHeader::with_leading_button("X"));
            container.spawn(card);
        });
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    mut commands: Commands,
    menu_q: Query<(Entity, &Children), With<CardDetail>>,
) {
    for event in event_reader.read() {
        for (entity, card_detail_children) in menu_q.iter() {
            match event {
                ContainerHeaderEvent::LeadingButtonPressed(header_entity) => {
                    if card_detail_children.contains(header_entity) {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

fn on_drag(
    mut cursor_ev: EventReader<CursorMoved>,
    mut card_detail_q: Query<(&Interaction, &mut Node), With<CardDetail>>,
) {
    for (inteaction, mut node) in card_detail_q.iter_mut() {
        if *inteaction == Interaction::Pressed {
            for cursor in cursor_ev.read() {
                if let (Some(delta), Val::Px(y), Val::Px(x)) = (cursor.delta, node.top, node.left) {
                    node.top = Val::Px(y + delta.y);
                    node.left = Val::Px(x + delta.x);
                };
            }
            return;
        }
    }
}
