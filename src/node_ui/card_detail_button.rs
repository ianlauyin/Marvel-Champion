use bevy::prelude::*;

use crate::component::Card;

use super::CardDetail;

#[derive(Component)]
#[require(Interaction, Card)]
pub struct CardDetailButton;

pub struct CardDetailButtonPlugin;

impl Plugin for CardDetailButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_interaction);
    }
}

fn listen_interaction(
    card_detail_button_q: Query<
        (&Interaction, &Card<'static>),
        (With<CardDetailButton>, Changed<Interaction>),
    >,
    mut commands: Commands,
) {
    for (interaction, card) in card_detail_button_q.iter() {
        if let Interaction::Pressed = interaction {
            commands.spawn(CardDetail::new(card.get_key(), card.is_vertical));
        }
    }
}
