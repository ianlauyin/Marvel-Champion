use bevy::prelude::*;

use crate::component::Card;

use super::CardDetail;

#[derive(Component)]
#[require(Interaction, Card)]
pub struct CardDetailButton;

pub struct CardDetailButtonPlugin;

impl Plugin for CardDetailButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_card_detail_button_click);
    }
}

fn listen_card_detail_button_click(
    card_detail_button_q: Query<
        (&Card<'static>, &Interaction),
        (With<CardDetailButton>, Changed<Interaction>),
    >,
    mut commands: Commands,
) {
    for (card_basic, interaction) in card_detail_button_q.iter() {
        match interaction {
            Interaction::Pressed => {
                commands.spawn(CardDetail::new(
                    card_basic.get_key(),
                    card_basic.is_vertical,
                ));
            }
            _ => {}
        }
    }
}
