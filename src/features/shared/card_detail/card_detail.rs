use bevy::{prelude::*, ui::FocusPolicy};

use crate::features::{cards::Card, shared::ButtonBuilder};

use super::{
    ally_detail::spawn_ally_detail, alter_ego_detail::spawn_alter_ego_detail,
    attachment_detail::spawn_attachment_detail, environment_detail::spawn_environment_detail,
    event_detail::spawn_event_detail, hero_detail::spawn_hero_detail,
    main_scheme_detail::spawn_main_scheme_detail, minion_detail::spawn_minion_detail,
    obligation_detail::spawn_obligation_detail, resource_detail::spawn_resource_detail,
    side_scheme_detail::spawn_side_scheme_detail, support_detail::spawn_support_detail,
    treachery_detail::spawn_treachery_detail, upgrade_detail::spawn_upgrade_detail,
    villain_detail::spawn_villain_detail,
};

pub struct CardDetailFramePlugin;

impl Plugin for CardDetailFramePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_escape);
    }
}

#[derive(Component)]
struct EscapeButton;

pub fn spawn_card_detail(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    card: Card,
    z_index: ZIndex,
) {
    commands
        .spawn(NodeBundle {
            focus_policy: FocusPolicy::Block,
            style: Style {
                width: Val::Percent(90.),
                height: Val::Percent(90.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                display: Display::Flex,
                ..default()
            },
            border_radius: BorderRadius::all(Val::Px(10.)),
            z_index,
            background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
            ..default()
        })
        .with_children(|container| {
            spawn_escape_button(container);
            spawn_content(container, asset_server, card);
        });
}

fn spawn_escape_button(children_builder: &mut ChildBuilder) {
    children_builder
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                right: Val::Px(5.),
                ..default()
            },
            ..default()
        })
        .with_children(|button_container| {
            ButtonBuilder {
                text: String::from("X"),
                text_color: Color::srgb(0.494, 0.494, 0.494),
                size: (Val::Px(30.), Val::Px(30.)),
                with_border: false,
                color: Color::srgb(0.239, 0.239, 0.239),
                border_radius: BorderRadius::all(Val::Percent(50.)),
                ..default()
            }
            .spawn(button_container)
            .insert(EscapeButton);
        });
}

fn spawn_content(container: &mut ChildBuilder, asset_server: Res<AssetServer>, card: Card) {
    match card {
        Card::Attachment(attachment_card) => spawn_attachment_detail(container, attachment_card),
        Card::Environment(environment_card) => {
            spawn_environment_detail(container, environment_card)
        }
        Card::MainScheme(main_scheme_card) => spawn_main_scheme_detail(container, main_scheme_card),
        Card::Minion(minion_card) => spawn_minion_detail(container, minion_card),
        Card::Obligation(obligation_card) => spawn_obligation_detail(container, obligation_card),
        Card::SideScheme(side_scheme_card) => spawn_side_scheme_detail(container, side_scheme_card),
        Card::Treachery(treachery_card) => spawn_treachery_detail(container, treachery_card),
        Card::Villain(villain_card) => spawn_villain_detail(container, villain_card),
        Card::Ally(ally_card) => spawn_ally_detail(container, ally_card),
        Card::AlterEgo(alter_ego_card) => spawn_alter_ego_detail(container, alter_ego_card),
        Card::Event(event_card) => spawn_event_detail(container, event_card),
        Card::Hero(hero_card) => spawn_hero_detail(container, asset_server, hero_card),
        Card::Resource(resource_card) => spawn_resource_detail(container, resource_card),
        Card::Support(support_card) => spawn_support_detail(container, support_card),
        Card::Upgrade(upgrade_card) => spawn_upgrade_detail(container, upgrade_card),
    };
}

fn on_escape(
    mut commands: Commands,
    escape_button_q: Query<(&Interaction, Entity), With<EscapeButton>>,
    parent_q: Query<&Parent>,
) {
    for (interaction, escape_button) in escape_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail = parent_q.iter_ancestors(escape_button).last().unwrap();
            commands.entity(card_detail).despawn_recursive();
            return;
        }
    }
}
