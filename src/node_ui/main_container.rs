use bevy::{prelude::*, ui::FocusPolicy};

use crate::util::UiUtils;

#[derive(Component)]
pub struct MainContainer(JustifyContent);

impl MainContainer {
    pub fn default() -> Self {
        Self(JustifyContent::Start)
    }

    pub fn space_around() -> Self {
        Self(JustifyContent::SpaceAround)
    }

    fn node(&self) -> Node {
        Node {
            width: Val::Percent(90.),
            height: Val::Percent(90.),
            padding: UiRect::all(Val::Px(10.)),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            display: Display::Flex,
            column_gap: Val::Px(20.),
            flex_direction: FlexDirection::Column,
            justify_content: self.0,
            align_items: AlignItems::Center,
            overflow: Overflow::scroll_y(),
            ..default()
        }
    }
}

pub struct MainContainerPlugin;

impl Plugin for MainContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_main_container_added);
    }
}

fn on_main_container_added(
    trigger: Trigger<OnAdd, MainContainer>,
    mut commands: Commands,
    main_container_q: Query<&MainContainer>,
    z_index_q: Query<&ZIndex>,
) {
    let main_container = main_container_q.get(trigger.target()).unwrap();
    commands.entity(trigger.target()).insert((
        main_container.node(),
        BorderRadius::all(Val::Px(10.)),
        BackgroundColor::from(Color::BLACK.with_alpha(0.9999)),
        UiUtils::get_largest_z_index(&z_index_q),
        FocusPolicy::Block,
    ));
}
