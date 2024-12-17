use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::features::shared::{previous_button::PreviousButton, CardListBuilder};

use super::spawn_list;

#[derive(Default, Clone)]
pub struct ListItem {
    pub text: String,
    pub color: Color,
    pub image: ImageNode,
}

pub enum DisplayMethod {
    ButtonList,
    CardList,
}

/// Reminder: Add PreviousButtonPlugin::<State>::default() in state plugin
pub struct MenuBuilder<T: Component, S: States + FreelyMutableState, B: Component> {
    pub component: T,
    pub previous_state: S,
    pub list_items: Vec<(B, ListItem)>,
    pub display_method: DisplayMethod,
}

impl<T: Component + Clone, S: States + FreelyMutableState, B: Component + Clone>
    MenuBuilder<T, S, B>
{
    pub fn spawn(&self, mut commands: Commands) -> Entity {
        let header = spawn_header(commands.reborrow(), self.previous_state.clone());
        let list = match self.display_method {
            DisplayMethod::ButtonList => spawn_list(commands.reborrow(), self.list_items.clone()),
            DisplayMethod::CardList => CardListBuilder {
                button_map: self.list_items.clone(),
                card_size: (Val::Px(128.), Val::Px(178.)),
                height: Val::Percent(90.),
                columns: 8,
            }
            .spawn(commands.reborrow()),
        };
        let mut menu_frame = commands.spawn((
            self.component.clone(),
            Node {
                width: Val::Percent(90.),
                height: Val::Percent(90.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_y(),
                ..default()
            },
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
        ));
        menu_frame.add_children(&[header, list]).id()
    }
}

fn spawn_header<S: States + FreelyMutableState>(
    mut commands: Commands,
    previous_state: S,
) -> Entity {
    commands
        .spawn(Node {
            height: Val::Percent(10.),
            padding: UiRect {
                left: Val::Px(10.),
                top: Val::Px(10.),
                bottom: Val::Px(10.),
                ..default()
            },
            ..default()
        })
        .with_children(|header| {
            header.spawn(PreviousButton(previous_state));
        })
        .id()
}
