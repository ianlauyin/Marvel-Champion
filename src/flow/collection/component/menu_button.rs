use bevy::prelude::*;

use crate::util::SystemUtil;

use super::super::CURRENT_STATE;
use super::SubMenu;

#[derive(Component)]
pub enum CollectionMenuButton {
    Aspect,
    IdentitySet,
    ModularSet,
    Scenario,
    StandardSet,
    ExpertSet,
}

impl CollectionMenuButton {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::Aspect,
            Self::IdentitySet,
            Self::ModularSet,
            Self::Scenario,
            Self::StandardSet,
            Self::ExpertSet,
        ]
    }

    pub fn get_text(&self) -> &str {
        match self {
            Self::Aspect => "Aspect",
            Self::IdentitySet => "Identity Set",
            Self::ModularSet => "Modular Set",
            Self::Scenario => "Scenario",
            Self::StandardSet => "Standard Set",
            Self::ExpertSet => "Expert Set",
        }
    }

    pub fn get_sub_menu(&self) -> SubMenu {
        match self {
            Self::Aspect => SubMenu::Aspect,
            Self::IdentitySet => SubMenu::IdentitySet,
            Self::ModularSet => SubMenu::ModularSet,
            Self::Scenario => SubMenu::Scenario,
            Self::StandardSet => SubMenu::StandardSet,
            Self::ExpertSet => SubMenu::ExpertSet,
        }
    }
}

pub struct CollectionMenuButtonPlugin;

impl Plugin for CollectionMenuButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_button_click.run_if(in_state(CURRENT_STATE)));
    }
}

fn handle_button_click(
    button_q: Query<(&Interaction, &CollectionMenuButton), Changed<Interaction>>,
    mut commands: Commands,
) {
    SystemUtil::handle_component_click(button_q, |button| {
        commands.spawn(button.get_sub_menu());
    });
}
