mod camera;
mod game_mat;
mod loading_screen;
mod main_menu;

pub const UI_PLUGINS: (
    game_mat::GameMatPlugin,
    camera::CameraPlugin,
    loading_screen::LoadingScreenPlugin,
    main_menu::MainMenuPlugin,
) = (
    game_mat::GameMatPlugin,
    camera::CameraPlugin,
    loading_screen::LoadingScreenPlugin,
    main_menu::MainMenuPlugin,
);
