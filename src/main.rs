use bevy::{ecs::system::SystemId, prelude::*};
use features::cards::{Card, CardAbility, CardDatas};

mod constants;
mod features;
mod systems;
mod ui;
mod utils;

fn main() {
    App::new()
        .add_plugins(systems::SystemPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(features::FeaturePlugin)
        // .add_systems(
        //     OnEnter(AppState::MainMenu),
        //     (spawn_component, test_component_effect).chain(),
        // )
        .run();
}

#[derive(Component)]
struct TestEffect(SystemId);

fn spawn_component(mut commands: Commands, card_datas: Res<CardDatas>) {
    let Some(oppoenent_card) = card_datas.0.get("core_166") else {
        println!("card not found");
        return;
    };
    let Card::SideScheme(side_scheme) = oppoenent_card else {
        return;
    };
    let CardAbility::WhenRevealed(effect) = side_scheme.abilities[0] else {
        return;
    };
    let system_id = commands.register_system(effect);
    commands.spawn(TestEffect(system_id));
}

fn test_component_effect(mut commands: Commands, test_effect_q: Query<&TestEffect>) {
    if test_effect_q.is_empty() {
        warn!("No TestEffect")
    }
    for test_effect in test_effect_q.iter() {
        commands.run_system(test_effect.0);
    }
}
