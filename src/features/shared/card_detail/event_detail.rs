use bevy::prelude::*;

use crate::{
    constants::DETAIL_CARD_SIZE,
    features::{cards::EventCard, shared::text_with_background::TextWithBackgroundBuilder},
};

pub fn spawn_event_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    event: EventCard<'static>,
) {
    children_builder
        .spawn({
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: UiRect::all(Val::Px(30.)),
                    column_gap: Val::Px(30.),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }
        })
        .with_children(|container| {
            spawn_image(container, asset_server.load(event.card_image_path));
            spawn_detail(container, event.clone());
            spawn_footer(container, event.id);
        });
}

fn spawn_image(container: &mut ChildBuilder, image: Handle<Image>) {
    container.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(DETAIL_CARD_SIZE.x),
                height: Val::Px(DETAIL_CARD_SIZE.y),
                ..default()
            },
            ..default()
        },
        UiImage::new(image),
    ));
}

fn spawn_detail(container: &mut ChildBuilder, event: EventCard<'static>) {
    container
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(70.),
                height: Val::Percent(90.),
                display: Display::Grid,
                grid_template_columns: vec![RepeatedGridTrack::auto(4)],
                grid_template_rows: vec![RepeatedGridTrack::auto(10)],
                row_gap: Val::Px(15.),
                column_gap: Val::Px(15.),
                ..default()
            },
            ..default()
        })
        .with_children(|detail_container| {
            let mut text_builder = TextWithBackgroundBuilder {
                text: event.name.to_string(),
                style: Style {
                    grid_column: GridPlacement::span(4),
                    ..default()
                },
                ..default()
            };
            text_builder.spawn(detail_container);

            text_builder.text = event.aspect.to_string();
            text_builder.spawn(detail_container);

            let traits_text: Vec<String> =
                event.traits.iter().map(|item| item.to_string()).collect();
            text_builder.text = format!("Traits: {}", traits_text.join(", "));
            text_builder.spawn(detail_container);

            let keyword_text: Vec<String> =
                event.keywords.iter().map(|item| item.to_string()).collect();
            text_builder.text = format!("Keywords: {}", keyword_text.join(", "));
            text_builder.spawn(detail_container);

            text_builder.text = event.description.to_string();
            text_builder.style.grid_row = GridPlacement::span(6);
            text_builder.style.display = Display::Flex;
            text_builder.spawn(detail_container);
        });
}

fn spawn_footer(container: &mut ChildBuilder, event_id: &str) {
    TextWithBackgroundBuilder {
        text: event_id.to_string(),
        style: Style {
            position_type: PositionType::Absolute,
            right: Val::Px(30.),
            bottom: Val::Px(10.),
            ..default()
        },
        ..default()
    }
    .spawn(container);
}
