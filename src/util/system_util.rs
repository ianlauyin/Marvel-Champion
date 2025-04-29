use bevy::prelude::*;

pub struct SystemUtil;

impl SystemUtil {
    pub fn cleanup_all<T: Component>(mut commands: Commands, component_q: Query<Entity, With<T>>) {
        for entity in component_q.iter() {
            commands.entity(entity).despawn();
        }
    }

    pub fn handle_button_click<T: Component>(
        button_q: Query<(&Interaction, &T), Changed<Interaction>>,
        mut func: impl FnMut(&T),
    ) {
        for (interaction, button) in button_q.iter() {
            if let Interaction::Pressed = interaction {
                func(button);
            }
        }
    }
}
