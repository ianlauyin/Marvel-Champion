use bevy::prelude::*;

use crate::{
    constants::{DETAIL_CARD_SIZE, DETAIL_ICON_SIZE},
    features::{cards::AllyCard, shared::text_with_background::TextWithBackgroundBuilder},
};

pub fn spawn_ally_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    ally: AllyCard<'static>,
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
            spawn_image(container, asset_server.load(ally.card_image_path));
            spawn_detail(container, asset_server, ally.clone());
            spawn_footer(container, ally.id);
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

fn spawn_detail(
    container: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    ally: AllyCard<'static>,
) {
    container
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(65.),
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
                text: format!("{} {}", if ally.unique { "+" } else { "" }, ally.name),
                style: Style {
                    grid_column: GridPlacement::span(2),
                    ..default()
                },
                ..default()
            };
            text_builder.spawn(detail_container);

            text_builder.text = ally.sub_name.to_string();
            text_builder.spawn(detail_container);

            text_builder.text = format!("HP: {}", ally.initial_hit_points);
            text_builder.style.grid_column = GridPlacement::span(1);
            text_builder.spawn(detail_container);

            text_builder.text = format!("THW: {}", ally.thw);
            text_builder.spawn(detail_container);

            text_builder.text = format!("THW Consequential Damage: {}", ally.thw_con_dmg);
            text_builder.style.grid_column = GridPlacement::span(2);
            text_builder.spawn(detail_container);

            text_builder.text = ally.aspect.to_string();
            text_builder.style.grid_column = GridPlacement::span(1);
            text_builder.spawn(detail_container);

            text_builder.text = format!("ATK: {}", ally.atk);
            text_builder.spawn(detail_container);

            text_builder.text = format!("ATK Consequential Damage: {}", ally.atk_con_dmg);
            text_builder.style.grid_column = GridPlacement::span(2);
            text_builder.spawn(detail_container);

            let traits_text: Vec<String> =
                ally.traits.iter().map(|item| item.to_string()).collect();
            text_builder.text = format!("Traits: {}", traits_text.join(", "));
            text_builder.style.grid_column = GridPlacement::span(4);
            text_builder.spawn(detail_container);

            let keyword_text: Vec<String> =
                ally.keywords.iter().map(|item| item.to_string()).collect();
            text_builder.text = format!("Keywords: {}", keyword_text.join(", "));
            text_builder.spawn(detail_container);

            text_builder.text = ally.description.to_string();
            text_builder.style.grid_row = GridPlacement::span(5);
            text_builder.style.display = Display::Flex;
            text_builder
                .spawn(detail_container)
                .with_children(|desciprtion_container| {
                    desciprtion_container
                        .spawn((NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                right: Val::Px(5.),
                                bottom: Val::Px(5.),
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|icons_container| {
                            for icon in ally.card_icons {
                                let icon_path = icon.get_icon_path();
                                icons_container.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(DETAIL_ICON_SIZE.x),
                                            height: Val::Px(DETAIL_ICON_SIZE.y),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    UiImage::new(asset_server.load(icon_path)),
                                ));
                            }
                        });
                });
        });
}

fn spawn_footer(container: &mut ChildBuilder, ally_id: &str) {
    TextWithBackgroundBuilder {
        text: ally_id.to_string(),
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
