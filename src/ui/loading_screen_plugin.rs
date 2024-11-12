use bevy::prelude::*;

use crate::constants::WINDOW_RESOLUTION;
pub struct LoadingScreenPlugin;

// Fire a LoadingScreenEvent to create a loading screen
impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.observe(handle_loading_screen);
    }
}

#[derive(Event)]
pub struct LoadingScreenEvent {
    loading: bool,
    identifier: String,
}

#[derive(Component)]
struct LoadingScreen {
    identifiers: Vec<String>,
}

fn handle_loading_screen(
    trigger: Trigger<LoadingScreenEvent>,
    mut commands: Commands,
    mut loading_screen_q: Query<(Entity, &mut LoadingScreen)>,
) {
    let LoadingScreenEvent {
        identifier,
        loading,
    } = trigger.event();
    match loading_screen_q.iter().len() {
        0 if !loading => {
            warn!("No loading screen found");
            return;
        }
        0 if *loading => {
            spawn_screen_mask(commands, identifier.clone());
            return;
        }
        1 => {}
        _ => warn!("More than one loading screen found."),
    }

    for (entity, loading_screen) in loading_screen_q.iter_mut() {
        handle_loading_screen_identifier(
            &mut commands,
            entity,
            loading_screen,
            *loading,
            identifier.clone(),
        );
    }
}

fn handle_loading_screen_identifier(
    commands: &mut Commands,
    entity: Entity,
    mut loading_screen: Mut<LoadingScreen>,
    loading: bool,
    identifier: String,
) {
    if loading {
        loading_screen.identifiers.push(identifier.clone());
        return;
    }
    let Some(index) = loading_screen
        .identifiers
        .iter()
        .position(|element| *element == identifier)
    else {
        warn!("Identifier: {identifier} not found");
        return;
    };
    loading_screen.identifiers.remove(index);
    if loading_screen.identifiers.is_empty() {
        commands.entity(entity).despawn();
    }
}

fn spawn_screen_mask(mut commands: Commands, identifier: String) {
    commands
        .spawn((
            LoadingScreen {
                identifiers: Vec::from([identifier]),
            },
            NodeBundle {
                style: Style {
                    width: Val::Px(WINDOW_RESOLUTION.x),
                    height: Val::Px(WINDOW_RESOLUTION.y),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BLACK.with_alpha(0.5)),
                ..default()
            },
        ))
        .with_children(|background_node| {
            background_node.spawn(TextBundle::from_section("Loading...", TextStyle::default()));
        });
}
