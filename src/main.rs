use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, hello_world)
    .run();
}


fn hello_world(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Text::new("Hello World"));
}