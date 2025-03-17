use bevy::prelude::*;

pub struct ComponentUtil;

impl ComponentUtil {
    pub fn cleanup_all<T: Component>(mut commands: Commands, component_q: Query<Entity, With<T>>) {
        for entity in component_q.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
