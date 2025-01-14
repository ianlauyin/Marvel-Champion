mod card_state;
mod deck_card;
mod encounter_card;
mod out_of_play_area;
mod player_discard_pile;
mod villain_area;

use bevy::app::{App, Plugin};
pub use card_state::{change_card_state_on_added, CardState};
pub use deck_card::DeckCard;
pub use encounter_card::EncounterCard;
pub use out_of_play_area::OutOfPlayArea;
pub use player_discard_pile::PlayerDiscardPile;

pub struct PlayAreaPlugin;

impl Plugin for PlayAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            deck_card::DeckCardPlugin,
            encounter_card::EncounterCardPlugin,
            out_of_play_area::OutOfPlayAreaPlugin,
            player_discard_pile::PlayerDiscardPilePlugin,
            villain_area::VillainAreaPlugin,
        ));
    }
}
