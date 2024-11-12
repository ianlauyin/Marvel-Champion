mod background_plugin;
mod camera_plugin;
mod loading_screen_plugin;
mod setup_plugin;

pub const UI_PLUGINS: (
    background_plugin::BackgroundPlugin,
    camera_plugin::CameraPlugin,
    loading_screen_plugin::LoadingScreenPlugin,
    setup_plugin::SetupPlugin,
) = (
    background_plugin::BackgroundPlugin,
    camera_plugin::CameraPlugin,
    loading_screen_plugin::LoadingScreenPlugin,
    setup_plugin::SetupPlugin,
);

pub use loading_screen_plugin::LoadingScreenEvent;
