use bevy::prelude::*;

pub fn clean_up<T: Component>(mut commands: Commands, queries: Query<Entity, With<T>>) {
    for entity in queries.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
