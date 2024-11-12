mod camera;
mod game_mat;
mod loading_screen;

pub const UI_PLUGINS: (
    game_mat::GameMatPlugin,
    camera::CameraPlugin,
    loading_screen::LoadingScreenPlugin,
) = (
    game_mat::GameMatPlugin,
    camera::CameraPlugin,
    loading_screen::LoadingScreenPlugin,
);
