use bevy::prelude::*;

#[derive(Component)]
pub struct Infomation;

pub struct DeckEditorInfomationPlugin;

impl Plugin for DeckEditorInfomationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_added);
    }
}

fn on_added(trigger: Trigger<OnAdd, Infomation>, mut commands: Commands) {}
