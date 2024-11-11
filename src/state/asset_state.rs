use bevy::prelude::*;

use crate::constants::WINDOW_RESOLUTION;

#[derive(Default, States, Eq, PartialEq, Debug, Hash, Clone)]
pub enum AssetState {
    #[default]
    Loaded,
    Loading,
}

/// Fire AssetLoadingEvent to change AssetState into Loading State, Spawn a loading screen.
///
/// Asset Loading State will be checked and turn into Loaded automatically.
pub struct AssetStatePlugin;

impl Plugin for AssetStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetState>()
            .add_systems(
                Update,
                check_asset_loaded.run_if(in_state(AssetState::Loading)),
            )
            .observe(check_asset_loading);
    }
}

#[derive(Event)]
pub struct AssetLoadingEvent {
    paths: Vec<String>,
}

#[derive(Component)]
struct AssetLoaderScreen {
    paths: Vec<String>,
}

fn check_asset_loaded(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AssetState>>,
    mut asset_loader_screen_q: Query<(Entity, &mut AssetLoaderScreen)>,
    asset_server: Res<AssetServer>,
) {
    let (entity, mut asset_loader_screen) = asset_loader_screen_q.get_single_mut().unwrap();
    for (index, path) in asset_loader_screen.paths.clone().iter().enumerate() {
        match asset_server.get_handle_untyped(path) {
            None => return,
            Some(_) => {
                asset_loader_screen.paths.remove(index);
            }
        }
    }
    next_state.set(AssetState::Loaded);
    commands.entity(entity).despawn();
}

fn check_asset_loading(
    trigger: Trigger<AssetLoadingEvent>,
    commands: Commands,
    mut asset_loader_screen_q: Query<&mut AssetLoaderScreen>,
    mut next_state: ResMut<NextState<AssetState>>,
) {
    let paths = trigger.event().paths.clone();
    if asset_loader_screen_q.is_empty() {
        spawn_screen_mask(commands, paths)
    } else {
        let mut asset_loader_screen = asset_loader_screen_q.get_single_mut().unwrap();
        asset_loader_screen.paths.extend(paths);
    }
    next_state.set(AssetState::Loading);
}

fn spawn_screen_mask(mut commands: Commands, paths: Vec<String>) {
    commands
        .spawn((
            AssetLoaderScreen { paths },
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
