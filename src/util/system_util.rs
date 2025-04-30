use bevy::prelude::*;

pub struct SystemUtil;

impl SystemUtil {
    pub fn cleanup_all<T: Component>(mut commands: Commands, component_q: Query<Entity, With<T>>) {
        for entity in component_q.iter() {
            commands.entity(entity).despawn();
        }
    }

    pub fn handle_component_click<T: Component>(
        component_q: Query<(&Interaction, &T), Changed<Interaction>>,
        mut func: impl FnMut(&T),
    ) {
        for (interaction, component) in component_q.iter() {
            if let Interaction::Pressed = interaction {
                func(component);
            }
        }
    }
}
