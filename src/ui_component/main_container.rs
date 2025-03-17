use bevy::prelude::*;

#[derive(Component)]
pub struct MainContainer;

pub struct MainContainerPlugin;

impl Plugin for MainContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_main_container_added);
    }
}

fn on_main_container_added(trigger: Trigger<OnAdd, MainContainer>, mut commands: Commands) {
    commands.entity(trigger.entity()).insert((
        Node {
            width: Val::Percent(90.),
            height: Val::Percent(90.),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(10.)),
        BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
    ));
}
