use bevy::{
    app::MainScheduleOrder,
    ecs::schedule::ScheduleLabel,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    bevy_inspector, egui, DefaultInspectorConfigPlugin,
};

pub struct DevtoolPlugin;

impl Plugin for DevtoolPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_plugins(EguiPlugin)
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_plugins(InspectSchedulePlugin)
            .add_systems(Startup, spawn_devtool_window)
            .add_observer(kill_app_when_primary_window_closed)
            .add_systems(Inspect, inspector_ui);
    }
}

#[derive(Component)]
struct DevtoolWindow;

const DEFAULT_SIZE: (f32, f32) = (320., 600.);

fn spawn_devtool_window(world: &mut World) {
    world.spawn((
        DevtoolWindow,
        Window {
            title: "Devtool".to_string(),
            resolution: WindowResolution::new(DEFAULT_SIZE.0, DEFAULT_SIZE.1),
            ..default()
        },
    ));
}

fn kill_app_when_primary_window_closed(
    _: Trigger<OnRemove, PrimaryWindow>,
    mut exit: EventWriter<AppExit>,
) {
    exit.send(AppExit::Success);
}

// Modified from bevy_inspector_egui::quick::WorldInspectorPlugin;
fn inspector_ui(world: &mut World) {
    let Ok((egui_context, window)) = world
        .query_filtered::<(&mut EguiContext, &Window), With<DevtoolWindow>>()
        .get_single(world)
    else {
        return;
    };

    let mut devtool_egui_context = egui_context.clone();
    let devtool_window = window.clone();

    egui::Window::new("World Inspector")
        .default_size(DEFAULT_SIZE)
        .resizable(false)
        .collapsible(false)
        .movable(false)
        .show(devtool_egui_context.get_mut(), |window_ui| {
            egui::ScrollArea::both().show(window_ui, |scroll_area_ui| {
                bevy_inspector::ui_for_world(world, scroll_area_ui);
                scroll_area_ui.allocate_space(scroll_area_ui.available_size());
            });
            window_ui.set_height(devtool_window.resolution.height());
            window_ui.set_width(devtool_window.resolution.width());
        });
}

// Copied from bevy_inspector_egui::quick::WorldInspectorPlugin;
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct Inspect;

struct InspectSchedulePlugin;
impl Plugin for InspectSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(Inspect);

        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_after(Update, Inspect);
    }
}
