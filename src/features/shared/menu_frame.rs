use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::features::shared::previous_button::PreviousButton;

/// Reminder: Add PreviousButtonPlugin::<State>::default() in state plugin
pub struct MenuBuilder<T: Component, S: States + FreelyMutableState> {
    pub component: T,
    pub previous_state: S,
    pub content_child: Entity,
}

impl<T: Component + Clone, S: States + FreelyMutableState> MenuBuilder<T, S> {
    pub fn spawn(&self, mut commands: Commands) -> Entity {
        let header = spawn_header(commands.reborrow(), self.previous_state.clone());
        // let list = match self.display_method {
        //     DisplayMethod::ButtonList => spawn_list(commands.reborrow(), self.list_items.clone()),
        // DisplayMethod::CardList => CardListBuilder {
        //     button_map: self.list_items.clone(),
        //     card_size: (Val::Px(128.), Val::Px(178.)),
        //     height: Val::Percent(90.),
        //     columns: 8,
        // }
        // .spawn(commands.reborrow()),
        //     DisplayMethod::TextList => {
        //         spawn_double_list(commands.reborrow(), self.list_items.clone())
        //     }
        // };
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

        menu_frame.add_children(&[header, self.content_child]).id()
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
