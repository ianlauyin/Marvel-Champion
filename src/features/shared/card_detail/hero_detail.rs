use bevy::prelude::*;

use crate::{
    constants::{DETAIL_CARD_SIZE, DETAIL_ICON_SIZE},
    features::{cards::HeroCard, shared::text_with_background::TextWithBackgroundBuilder},
};

pub fn spawn_hero_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    hero: HeroCard<'static>,
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
            spawn_image(container, asset_server.load(hero.card_image_path));
            spawn_detail(container, asset_server, hero.clone());
            spawn_footer(container, hero.id, hero.flip_target_id);
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
    hero: HeroCard<'static>,
) {
    container
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(70.),
                height: Val::Percent(90.),
                display: Display::Grid,
                grid_template_columns: vec![RepeatedGridTrack::auto(4)],
                grid_template_rows: vec![RepeatedGridTrack::auto(10)],
                row_gap: Val::Px(30.),
                column_gap: Val::Px(30.),
                ..default()
            },
            ..default()
        })
        .with_children(|detail_container| {
            let mut text_builder = TextWithBackgroundBuilder {
                text: hero.name.to_string(),
                style: Style {
                    grid_column: GridPlacement::span(3),
                    ..default()
                },
                ..default()
            };
            text_builder.spawn(detail_container);

            text_builder.text = format!("Hand Size: {}", hero.hand_size);
            text_builder.style.grid_column = GridPlacement::span(1);
            text_builder.spawn(detail_container);

            text_builder.text = format!("HP: {}", hero.initial_hit_points);
            text_builder.spawn(detail_container);

            text_builder.text = format!("THW: {}", hero.thw);
            text_builder.spawn(detail_container);

            text_builder.text = format!("ATK: {}", hero.atk);
            text_builder.spawn(detail_container);

            text_builder.text = format!("DEF: {}", hero.def);
            text_builder.spawn(detail_container);

            let traits_text: Vec<String> =
                hero.traits.iter().map(|item| item.to_string()).collect();
            text_builder.text = format!("Traits: {}", traits_text.join(", "));
            text_builder.style.grid_column = GridPlacement::span(4);
            text_builder.spawn(detail_container);

            let keyword_text: Vec<String> =
                hero.keywords.iter().map(|item| item.to_string()).collect();
            text_builder.text = format!("Keywords: {}", keyword_text.join(", "));
            text_builder.spawn(detail_container);

            text_builder.text = hero.description.to_string();
            text_builder.style.grid_row = GridPlacement::span(6);
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
                            for icon in hero.card_icons {
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

fn spawn_footer(container: &mut ChildBuilder, hero_id: &str, flip_target_id: Vec<&str>) {
    container
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(30.),
                bottom: Val::Px(10.),
                display: Display::Flex,
                column_gap: Val::Px(10.),
                ..default()
            },
            ..default()
        })
        .with_children(|footer_container| {
            let mut text_builder = TextWithBackgroundBuilder {
                text: format!("Flip Targets: {}", flip_target_id.join(", ")),
                ..default()
            };
            text_builder.spawn(footer_container);

            text_builder.text = hero_id.to_string();
            text_builder.spawn(footer_container);
        });
}
